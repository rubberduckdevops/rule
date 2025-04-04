# RULE - Rust Utility Log Enrichment

RULE is a simple command-line utility for viewing and analyzing log files with enhanced readability through color-coding of different log levels.

## Features

- **Color-coded log levels**: Automatically highlights INFO, ERROR, WARNING, and DEBUG log levels with appropriate colors
- **Search functionality**: Find and highlight specific text within log files
- **Simple interface**: Easy-to-use command-line interface built with Clap

## Installation

```bash
cargo install rule
```

Or clone and build from source:

```bash
git clone https://github.com/yourusername/rule.git
cd rule
cargo build --release
```

## Usage

### View a log file with color-coded log levels

```bash
rule read --path /path/to/logfile.log
```

### Find specific text in a log file

```bash
rule find --path /path/to/logfile.log --string "error message"
```

### Display current date information

```bash
rule today
```

### Get help

```bash
rule --help
```

## Author

Michael Hall <michael@rubberduck-labs.com>