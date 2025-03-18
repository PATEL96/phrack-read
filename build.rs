use std::env;
use std::fs;
use std::process::Command;

fn main() {
    println!("Running phrack-read post-install script...");

    if cfg!(target_os = "windows") {
        println!("Skipping automatic setup on Windows. Please add to PATH manually.");
    } else {
        let home_dir = env::var("HOME").expect("HOME environment variable not set");
        let bin_dir = format!("{}/.cargo/bin", home_dir);
        let script_content = format!(
            r#"
#!/bin/bash
set -e

echo "Adding phrack-read to PATH..."
echo 'export PATH="{}:$PATH"' >> ~/.bashrc
echo 'export PATH="{}:$PATH"' >> ~/.zshrc

echo "Installation complete! Run 'phrack-read <issue> <article>'"
"#,
            bin_dir, bin_dir
        );

        let install_script = format!("{}/phrack-read-install.sh", env::var("OUT_DIR").unwrap());
        fs::write(&install_script, script_content).expect("Failed to write install script");

        Command::new("sh")
            .arg(&install_script)
            .status()
            .expect("Failed to execute install script");
    }
}
