use clap::Parser;
use serde_json::Value;
use indexmap::IndexMap;
use std::io::{self, Read};
use std::cmp::max;
use colored::*;
use atty::Stream;
use term_size;
use regex::Regex;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    width: Option<usize>,

    #[arg(short, long)]
    truncate: bool,

    #[arg(short = 'm', long)]
    truncate_min: Option<usize>,

    #[arg(short = 'n', long)]
    truncate_to: Option<usize>,

    #[arg(short = 'x', long)]
    no_ruler: bool,

    #[arg(short = 'b', long)]
    no_color: bool,

    #[arg(short, long, default_value = "id")]
    primary: String,

    #[arg(short = 'l', long, default_value = "name")]
    highlight: String,

    #[arg(short, long)]
    yellow: Option<String>,

    #[arg(short = 'g', long)]
    green: Option<String>,

    #[arg(short = 'a', long)]
    magenta: Option<String>,

    #[arg(short, long)]
    red: Option<String>,

    #[arg(short = 'e', long)]
    grep: Option<String>,
}

fn process_record(map: &IndexMap<String, Value>, cli: &Cli, width: usize) {
    let mut columns = Vec::new();
    for (key, value) in map {
        let value_str = match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            _ => value.to_string(),
        };
        columns.push((key.clone(), value_str));
    }

    let mut header_line = String::new();
    let mut value_line = String::new();
    let mut current_width = 0;

    let truncate_min = cli.truncate_min.unwrap_or(width);
    let truncate_to = cli.truncate_to.unwrap_or(width - 1);

    for (key, value_str) in columns {
        let is_wide = value_str.len() > width;

        if is_wide {
            // Print the current line of normal columns
            if !header_line.is_empty() {
                println!("{}", header_line);
                println!("{}", value_line);
                header_line.clear();
                value_line.clear();
                current_width = 0;
            }

            // Print the wide column vertically
            let header_colored = key.as_str().cyan().on_bright_black();
            let value_colored = if key == cli.primary {
                value_str.as_str().red()
            } else if key == cli.highlight {
                value_str.as_str().black().on_white()
            } else if Some(&key) == cli.yellow.as_ref() {
                value_str.as_str().black().on_yellow()
            } else if Some(&key) == cli.green.as_ref() {
                value_str.as_str().black().on_green()
            } else if Some(&key) == cli.magenta.as_ref() {
                value_str.as_str().black().on_magenta()
            } else if Some(&key) == cli.red.as_ref() {
                value_str.as_str().black().on_red()
            } else {
                value_str.as_str().yellow()
            };
            println!("{}", header_colored);
            println!("{}", value_colored);

        } else { // Normal column
            let mut value_display = value_str.clone();
            let effective_col_width;

            if cli.truncate && value_str.len() > truncate_min {
                if truncate_to > 3 {
                    value_display = format!("{}...", &value_str[..(truncate_to - 3)]);
                } else {
                    value_display = "...".to_string();
                }
                effective_col_width = max(key.len(), value_display.len());
            } else {
                if value_str.len() > truncate_to {
                    effective_col_width = key.len();
                } else {
                    effective_col_width = max(key.len(), value_str.len());
                }
            }

            if current_width > 0 && current_width + effective_col_width + 1 > width {
                println!("{}", header_line);
                println!("{}", value_line);
                header_line.clear();
                value_line.clear();
                current_width = 0;
            }

            let header = format!("{:<width$}", key.clone(), width = effective_col_width);
            let value = format!("{:<width$}", value_display, width = effective_col_width);
            
            let header_colored = header.cyan().on_bright_black();

            let value_colored;
            if key == cli.primary {
                value_colored = value.red();
            } else if key == cli.highlight {
                value_colored = value.black().on_white();
            } else if Some(&key) == cli.yellow.as_ref() {
                value_colored = value.black().on_yellow();
            } else if Some(&key) == cli.green.as_ref() {
                value_colored = value.black().on_green();
            } else if Some(&key) == cli.magenta.as_ref() {
                value_colored = value.black().on_magenta();
            } else if Some(&key) == cli.red.as_ref() {
                value_colored = value.black().on_red();
            } else {
                value_colored = value.yellow();
            }

            header_line.push_str(&format!("{} ", header_colored));
            value_line.push_str(&format!("{} ", value_colored));
            current_width += effective_col_width + 1;
        }
    }

    // Print any remaining normal columns
    if !header_line.is_empty() {
        println!("{}", header_line);
        println!("{}", value_line);
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.no_color || !atty::is(Stream::Stdout) {
        colored::control::set_override(false);
    }

    let width = cli.width.or_else(|| term_size::dimensions().map(|(w, _)| w)).unwrap_or(80);

    let grep_re = cli.grep.as_ref().map(|pattern| {
        Regex::new(pattern).unwrap_or_else(|e| {
            eprintln!("Invalid grep pattern: {}", e);
            std::process::exit(1);
        })
    });

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");

    let records: Vec<IndexMap<String, Value>> = serde_json::from_str(&buffer).unwrap_or_else(|e| {
        eprintln!("Error parsing JSON: {}", e);
        std::process::exit(1);
    });

    if !records.is_empty() && !cli.no_ruler {
        println!("{}", "─".repeat(width).bright_black());
    }

    let mut first_record = true;
    for map in &records {
        let obj_str = serde_json::to_string(map).unwrap_or_default();
        if let Some(re) = &grep_re {
            if !re.is_match(&obj_str) {
                continue;
            }
        }

        if !first_record && !cli.no_ruler {
            println!("{}", "─".repeat(width).bright_black());
        }
        first_record = false;

        process_record(map, &cli, width);
    }

    if !records.is_empty() && !cli.no_ruler {
         println!("{}", "─".repeat(width).bright_black());
    }
}
