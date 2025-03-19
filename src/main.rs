use clap::Parser;
use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

/// CLI to read Phrack magazine articles
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Phrack issue number
    issue: String,
    /// Article number
    article: String,
}

struct ScreenState {
    content: Vec<(String, bool, bool)>, // (line, is_highlighted, is_heading)
    current_line: usize,
    terminal_height: usize,
    terminal_width: usize,
}

impl ScreenState {
    fn new(content: Vec<(String, bool, bool)>) -> io::Result<Self> {
        let (width, height) = terminal::size()?;
        Ok(Self {
            content,
            current_line: 0,
            terminal_height: height as usize - 2,
            terminal_width: width as usize,
        })
    }

    fn display(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        let end_line = std::cmp::min(self.current_line + self.terminal_height, self.content.len());

        for (i, (line, is_highlighted, is_heading)) in
            self.content[self.current_line..end_line].iter().enumerate()
        {
            execute!(stdout, cursor::MoveTo(0, i as u16))?;

            let displayed_line = if UnicodeWidthStr::width(line.as_str()) > self.terminal_width {
                let mut displayed_width = 0;
                let mut truncated = String::new();
                for c in line.chars() {
                    let char_width = UnicodeWidthStr::width(c.to_string().as_str());
                    if displayed_width + char_width > self.terminal_width - 3 {
                        break;
                    }
                    truncated.push(c);
                    displayed_width += char_width;
                }
                format!("{}...", truncated)
            } else {
                line.clone()
            };

            if *is_highlighted {
                write!(stdout, "{}", displayed_line.bright_cyan().bold())?;
            } else if *is_heading {
                write!(stdout, "{}", displayed_line.bright_red().bold())?;
            } else {
                write!(stdout, "{}", displayed_line)?;
            }
        }

        let percentage = if self.content.is_empty() {
            100
        } else {
            (end_line * 100) / self.content.len()
        };
        let status = format!(
            " Lines {}-{} of {} ({}%)  [↑/↓/PgUp/PgDn to navigate, q to quit] ",
            self.current_line + 1,
            end_line,
            self.content.len(),
            percentage
        );

        execute!(
            stdout,
            cursor::MoveTo(0, self.terminal_height as u16),
            terminal::Clear(ClearType::CurrentLine)
        )?;
        write!(stdout, "{}", status.black().on_bright_white())?;
        stdout.flush()?;

        Ok(())
    }

    fn scroll_up(&mut self, lines: usize) {
        self.current_line = self.current_line.saturating_sub(lines);
    }

    fn scroll_down(&mut self, lines: usize) {
        let max_start_line = if self.content.len() > self.terminal_height {
            self.content.len() - self.terminal_height
        } else {
            0
        };
        self.current_line = std::cmp::min(self.current_line + lines, max_start_line);
    }

    fn page_up(&mut self) {
        self.scroll_up(self.terminal_height);
    }

    fn page_down(&mut self) {
        self.scroll_down(self.terminal_height);
    }
}

fn should_highlight_headings(line: &str) -> bool {
    // Create patterns that match lines starting with |= and ending with =|
    let pattern1 = Regex::new(r"^\|=").unwrap(); // Match lines starting with |=
    let pattern2 = Regex::new(r"=\|$").unwrap(); // Match lines ending with =|

    // Check if the line matches both patterns
    pattern1.is_match(line) && pattern2.is_match(line)
}

fn should_highlight_topics(line: &str) -> bool {
    let pattern = Regex::new(r"--\[").unwrap();

    // Check if the line matches both patterns
    pattern.is_match(line)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let url = format!("https://phrack.org/issues/{}/{}", args.issue, args.article);

    println!(
        "{} {}",
        "Fetching Article from:".bright_blue().bold(),
        url.bright_green()
    );
    println!("Loading content, please wait...");

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;

    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        println!("Failed to fetch article: HTTP {}", response.status());
        if response.status().as_u16() == 404 {
            println!("Article not found. Please check the issue and article numbers.");
        }
        return Ok(());
    }

    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let pre_selector = Selector::parse("pre").unwrap();

    let mut content_lines: Vec<(String, bool, bool)> = Vec::new();
    let mut count = 0;

    content_lines.push((String::new(), false, false));
    content_lines.push((format!("╔{}╗", "═".repeat(78)), false, false));

    let padded_title = format!("  Phrack Issue {} Article {}  ", args.issue, args.article);
    let side_padding = (78 - UnicodeWidthStr::width(padded_title.as_str())) / 2;
    let left_padding = side_padding;
    let right_padding = 78 - UnicodeWidthStr::width(padded_title.as_str()) - left_padding;

    content_lines.push((
        format!(
            "║{}{}{}║",
            " ".repeat(left_padding),
            padded_title,
            " ".repeat(right_padding)
        ),
        false,
        false,
    ));
    content_lines.push((format!("╚{}╝", "═".repeat(78)), false, false));
    content_lines.push((String::new(), false, false));
    content_lines.push(("ARTICLE CONTENT:".to_string(), false, false));
    content_lines.push((String::new(), false, false));

    for element in document.select(&pre_selector) {
        count += 1;
        let pre_content = element.text().collect::<Vec<_>>().join("");

        if count > 1 {
            content_lines.push((format!("{}", "═".repeat(80)), false, false));
        }

        content_lines.push((format!("SECTION {}", count), false, false));
        content_lines.push((format!("{}", "─".repeat(80)), false, false));

        for line in pre_content.lines() {
            let is_highlighted = should_highlight_headings(line);
            let is_heading = should_highlight_topics(line);
            content_lines.push((line.to_string(), is_highlighted, is_heading));
        }
    }

    if count == 0 {
        content_lines.push((format!("\nNo Article tags found on the page."), false, true));
    } else {
        content_lines.push((String::new(), false, false));
        content_lines.push((format!("╔{}╗", "═".repeat(78)), false, false));

        let summary_text = format!("Direct Link to Article {}", url);
        let padded_summary = format!("  {}  ", summary_text);
        let side_padding = (78 - UnicodeWidthStr::width(padded_summary.as_str())) / 2;
        let left_padding = side_padding;
        let right_padding = 78 - UnicodeWidthStr::width(padded_summary.as_str()) - left_padding;

        content_lines.push((
            format!(
                "║{}{}{}║",
                " ".repeat(left_padding),
                padded_summary,
                " ".repeat(right_padding)
            ),
            false,
            false,
        ));
        content_lines.push((format!("╚{}╝", "═".repeat(78)), false, false));
    }

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;

    let mut screen = ScreenState::new(content_lines)?;
    screen.display(&mut stdout)?;

    loop {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => screen.scroll_up(1),
                KeyCode::Down => screen.scroll_down(1),
                KeyCode::PageUp => screen.page_up(),
                KeyCode::PageDown => screen.page_down(),
                KeyCode::Char('q') | KeyCode::Esc => break,
                _ => {}
            }
            screen.display(&mut stdout)?;
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
