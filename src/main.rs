use std::io::{self, Write};
use clap::Parser;
use reqwest;
use scraper::{Html, Selector};
use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};
use unicode_width::UnicodeWidthStr;
use clipboard::{ClipboardProvider, ClipboardContext};

/// CLI to read Phrack magazine articles
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Phrack issue number
    issue: String,
    /// Article number
    article: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let url = format!("https://phrack.org/issues/{}/{}", args.issue, args.article);

    println!("{} {}", "Fetching Article from:".bright_blue().bold(), url.bright_green());
    println!("Loading content, please wait...");

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;

    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let pre_selector = Selector::parse("pre").unwrap();

    let mut content_lines = Vec::new();
    content_lines.push(format!("Share this article with friends: {}", url.bright_yellow()));
    content_lines.push("Press 'c' to copy the link, 'q' to quit".to_string());

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;

    loop {
        execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        for line in &content_lines {
            writeln!(stdout, "{}", line)?;
        }
        stdout.flush()?;

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char('c') => {
                    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
                    ctx.set_contents(url.clone())?;
                    content_lines.push("Copied link to clipboard!".green().to_string());
                }
                KeyCode::Char('q') | KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
