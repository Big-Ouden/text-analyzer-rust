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

// ********* ratatui Stuff **********
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

fn ui(f: &mut ratatui::Frame, report: &Report) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let text = format!(
        "Characters: {}\nWords: {}\nLines: {}",
        report.char_count, report.word_count, report.line_count
    );

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title("Text Analysis Report")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, chunks[0]);
}

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

#[derive(Debug, Eq, PartialEq)]
struct Report {
    char_count: usize,
    word_count: usize,
    line_count: usize,
}

// ********* Functions **********
fn read_input(file: Option<PathBuf>) -> FunctionResult<String> {
    let mut content = String::new();
    if let Some(path) = file {
        // case 1 :  read specified file
        let data = fs::read_to_string(path)?;
        Ok(data)
    } else if !atty::is(Stream::Stdin) {
        // case 2 : stdin not a terminal, read from a pipe
        io::stdin().read_to_string(&mut content)?;
        Ok(content)
    } else {
        // case 3 : stdin is a interactive terminal and no file specified
        eprintln!("No input provided. Use --file <path> or pipe data into stdin.");
        std::process::exit(1);
    }
}

fn print_report(report: Report) -> FunctionResult<()> {
    let mut terminal = init_terminal()?;

    terminal.draw(|f| {
        // Utilisation d'un block basique avec le titre
        let size = f.size();
        let block = ratatui::widgets::Block::default()
            .title("Report")
            .borders(ratatui::widgets::Borders::ALL);

        f.render_widget(block, size);

        // Tu peux ajouter un Paragraph avec ton report ici
        use ratatui::text::{Line, Span};
        use ratatui::widgets::Paragraph;

        let lines = vec![
            Line::from(vec![Span::raw(format!("Chars: {}", report.char_count))]),
            Line::from(vec![Span::raw(format!("Words: {}", report.word_count))]),
            Line::from(vec![Span::raw(format!("Lines: {}", report.line_count))]),
        ];
        let paragraph = Paragraph::new(lines);

        f.render_widget(paragraph, size);
    })?;

    // ⚠️ Si tu restaures direct, tu "casses" l'affichage.
    // À ce stade, soit tu:
    //  - attends un input utilisateur (genre touche q), soit
    //  - tu fais un sleep pour voir le rendu avant le restore.
    std::thread::sleep(std::time::Duration::from_secs(3));

    restore_terminal()?;
    Ok(())
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

fn analyze(text: &str) -> FunctionResult<Report> {
    let mut report: Report = Report {
        char_count: count_char(&text),
        word_count: count_words(&text),
        line_count: count_lines(&text),
    };
    Ok(report)
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

    #[test]
    fn test_analyze() -> FunctionResult<()> {
        let text: &str = "aaa\naa";
        let expected_report: Report = Report {
            char_count: 6,
            word_count: 2,
            line_count: 2,
        };

        assert_eq!(analyze(&text)?, expected_report);
        Ok(())
    }
}

// ********* Main Function **********

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { file } => {
            let content: String = read_input(file)?;

            // analyze text
            let report: Report = analyze(&content)?;

            // print result
            print_report(report)?;
        }
    }

    Ok(())
}
