use clap::Parser;
use std::io::{self, BufRead};
use unicode_width_cli::{calculate_width, width_breakdown};

mod truncate;
use truncate::{Alignment, pad, truncate, truncate_and_pad};

#[derive(Parser)]
#[command(name = "unicode-width")]
#[command(about = "Calculate display width of Unicode strings", long_about = None)]
struct Cli {
    #[arg(help = "Input string(s) to measure")]
    input: Vec<String>,

    #[arg(short, long, help = "Truncate to specified width")]
    truncate: Option<usize>,

    #[arg(short, long, help = "Pad to specified width")]
    pad: Option<usize>,

    #[arg(short, long, value_parser = parse_alignment, default_value = "left")]
    align: Alignment,

    #[arg(short, long, help = "Show character-by-character breakdown")]
    breakdown: bool,
}

fn parse_alignment(s: &str) -> Result<Alignment, String> {
    match s.to_lowercase().as_str() {
        "left" => Ok(Alignment::Left),
        "right" => Ok(Alignment::Right),
        "center" => Ok(Alignment::Center),
        _ => Err(format!("Invalid alignment: {}", s)),
    }
}

fn process_string(s: &str, cli: &Cli) {
    if cli.breakdown {
        println!("String: {}", s);
        for (ch, width) in width_breakdown(s) {
            println!("  '{}' (U+{:04X}): width {}", ch, ch as u32, width);
        }
        println!("Total width: {}", calculate_width(s));
    } else {
        let output = match (cli.truncate, cli.pad) {
            (Some(t), Some(p)) => truncate_and_pad(s, t.min(p), cli.align),
            (Some(t), None) => truncate(s, t),
            (None, Some(p)) => pad(s, p, cli.align),
            (None, None) => {
                println!("{}: {}", s, calculate_width(s));
                return;
            }
        };
        println!("{}", output);
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if cli.input.is_empty() {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line?;
            process_string(&line, &cli);
        }
    } else {
        for input in &cli.input {
            process_string(input, &cli);
        }
    }

    Ok(())
}
