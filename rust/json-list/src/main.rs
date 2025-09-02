use clap::Parser;
use serde_json::{Value, Map};
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

fn process_record(map: &Map<String, Value>, cli: &Cli, width: usize) {
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

        let header = format!("{:<width$}", key, width = effective_col_width);
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

    let json_data: Value = serde_json::from_str(&buffer).unwrap_or_else(|e| {
        eprintln!("Error parsing JSON: {}", e);
        std::process::exit(1);
    });

    if let Some(array) = json_data.as_array() {
        if !cli.no_ruler {
            println!("{}", "─".repeat(width).bright_black());
        }

        let mut first_record = true;
        for obj in array.iter() {
            if let Some(map) = obj.as_object() {
                if let Some(re) = &grep_re {
                    if !re.is_match(&obj.to_string()) {
                        continue;
                    }
                }

                if !first_record && !cli.no_ruler {
                    println!("{}", "─".repeat(width).bright_black());
                }
                first_record = false;

                process_record(map, &cli, width);
            }
        }

        if !cli.no_ruler {
             println!("{}", "─".repeat(width).bright_black());
        }
    } else {
        eprintln!("Error: Input JSON is not an array.");
    }
}
