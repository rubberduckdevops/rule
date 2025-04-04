use clap::{Error, Parser, Subcommand};
use colored::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum RustyLogError {
    IoError(std::io::Error),
    UnknownError(String),
}

impl From<std::io::Error> for RustyLogError {
    fn from(error: std::io::Error) -> Self {
        RustyLogError::IoError(error)
    }
}

impl std::fmt::Display for RustyLogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RustyLogError::IoError(e) => write!(f, "I/O Error: {}", e),
            RustyLogError::UnknownError(e) => write!(f, "Unknown Error: {}", e),
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Today,
    Read {
        #[arg(short, long)]
        path: String
    },
    Find {
        #[arg(short, long)]
        path: String,
        #[arg(short, long, required = false)]
        string: String
    }
}

fn today() -> Result<(), RustyLogError> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // This is a simplified way to get the date
    // In a real app, you'd use a proper date/time library like chrono
    println!("Today is approximately {} days since Jan 1, 1970",
             since_epoch.as_secs() / 86400);
    if since_epoch.as_secs() / 86400 > 20500 {
        return Err(RustyLogError::UnknownError("Epoch is way to high".to_owned()));
    }
    Ok(())
}

fn read_file(path: &str) {
    if let Ok(lines)= read_log_lines(path){
        for line in lines.map_while(Result::ok) {
            let mut p_line = color_lines(&line);
            println!("{}", p_line)
        }
    }
}


fn find_file(path: &str, string: &str) {
    if let Ok(lines)= read_log_lines(path){
        for line in lines.map_while(Result::ok) {
            let mut p_line = color_lines(&line);
            if p_line.contains(string) {
                p_line = p_line.replace(string, &string.bold().underline().to_string());
                println!("{}", p_line)
            }
        }
    }
}

fn read_log_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn color_lines(line: &str) -> String {
    match line {
        line if line.contains("INFO") => line.replace("INFO", &"INFO".bold().to_string()),
        line if line.contains("ERROR") => line.replace("ERROR", &"ERROR".red().bold().to_string()),
        line if line.contains("WARNING") => line.replace("WARNING", &"WARNING".yellow().to_string()),
        line if line.contains("DEBUG") => line.replace("DEBUG", &"DEBUG".blue().to_string()),
        _ => line.to_string(),
    }
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Today) => {
            if let Err(e) = today() {
                println!("{}", format!("Something went wrong {}", e).red().bold());
            }
        },
        Some(Commands::Read { path }) => {
          read_file(&path)
        },
        Some(Commands::Find { path, string}) => {
            find_file(&path, &string)
        }
        None => {
            println!("Use --help to print options ");
        }
    }
}