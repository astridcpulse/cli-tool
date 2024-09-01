use anyhow::{Context, Result};
use clap::Parser;
use log::{info};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
    
    fn answer() -> u32 {
        return 42;
    }

}

fn main() -> Result<()> {
    env_logger::init();
    let args = Cli::parse();

    
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    info!("before loop");
    for line in content.lines() {
        
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
