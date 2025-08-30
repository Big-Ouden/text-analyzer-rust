/*!
 * Project: text-analyzer
 * Description: A minimal command-line text analyzer in Rust.
 * Author: BigOuden
 * GitHub: https://github.com/Big-Ouden/text-analyzer-rust
 *
 * Notes:
 *  - Written for learning Rust and CLI application development
 */

// ********* Uses **********

use atty::Stream;
use clap::{Parser, Subcommand};
use prettytable::{Table, cell, row};
use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

// ********* CLI Stuff **********

#[derive(Parser)]
#[command(name = "text-analyzer")]
#[command(about="Minimal text analyzer in Rust", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Command enum
#[derive(Subcommand)]
enum Commands {
    // Analyze a text
    Analyze {
        // Specifiy a file
        #[arg(
            short,
            long,
            value_name = "FILE",
            help = "Text file to analyze (default: stdin)"
        )]
        file: Option<PathBuf>,
    },
}

// *********    Type    **********
type FunctionResult<T> = Result<T, Box<dyn std::error::Error>>;

// ********* Structures **********

struct Report {
    char_count: usize,
    word_count: usize,
    line_count: usize,
}

// ********* Functions **********

fn analyze(text: String) -> FunctionResult<Report> {
    panic!("Analyze not implemented yet");
}

fn print_report(report: Report) -> FunctionResult<()> {
    panic!("print report not implemented yet");
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_char(text: &str) -> usize {
    text.chars().count()
}

fn count_lines(text: &str) -> usize {
    text.lines().count()
}

// *********      Test     **********
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("lorem ipsum"), 2);
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("lorem ipsum^#~"), 2);
        assert_eq!(count_words("lorem ipsu\n coucou"), 3);
        assert_eq!(count_words("lorem   ipsum   "), 2);
    }

    #[test]
    fn test_count_char() {
        assert_eq!(count_char(""), 0);
        assert_eq!(count_char("abc"), 3);
        assert_eq!(count_char(" abc "), 5);
        assert_eq!(count_char("a.b.c"), 5);
        assert_eq!(count_char("😊"), 1);
    }

    #[test]
    fn test_count_lines() {
        assert_eq!(count_lines("1\n2\n2\n4"), 4);
        assert_eq!(count_lines("1"), 1);
        assert_eq!(count_lines(""), 0);
    }
}

// ********* Main Function **********

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { file } => {
            let mut content = String::new();

            if let Some(path) = file {
                // case 1 :  read specified file
                content = fs::read_to_string(path)?;
            } else if !atty::is(Stream::Stdin) {
                // case 2 : stdin not a terminal, read from a pipe
                io::stdin().read_to_string(&mut content)?;
            } else {
                // case 3 : stdin is a interactive terminal and no file specified
                eprintln!("No input provided. Use --file <path> or pipe data into stdin.");
                std::process::exit(1);
            };

            // analyze text
            let report: Report = analyze(content)?;

            // print result
            print_report(report)?;
        }
    }

    Ok(())
}
