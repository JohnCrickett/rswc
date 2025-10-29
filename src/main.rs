use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "John Crickett",
    version,
    about = "rswc, a simple wc clone in Rust"
)]
struct Arguments {
    /// Count bytes.  Count the number of bytes in the file.
    #[arg(short)]
    c: bool,

    /// Count words.  Count the number of words in the file.
    #[arg(short)]
    w: bool,

    /// Count lines.  Count the number of lines in the file.
    #[arg(short)]
    l: bool,

    /// Count characters.  Count the number of characters in the file.
    #[arg(short)]
    m: bool,

    files: Vec<String>,
}

fn main() -> std::io::Result<()> {
    let args = Arguments::parse();

    if let Some(first) = args.files.first() {
        let file = File::open(first)?;
        let mut reader = BufReader::new(file);
        let mut byte_count = 0;
        let mut line_count = 0;
        let mut word_count = 0;
        let mut char_count = 0;

        let mut buf = String::new();
        loop {
            buf.clear();
            let n = reader.read_line(&mut buf)?; // n is bytes read; includes '\n' if present
            if n == 0 { break; }

            byte_count += n; // bytes, including newline
            line_count += 1;
            word_count += buf.split_whitespace().count();
            char_count += buf.chars().count(); // includes newline as one character
        }


        let max_digits = max_digits(byte_count, char_count, line_count, word_count);
        let byte_count_str = format!("{:>width$}", byte_count, width = max_digits as usize);
        let line_count_str = format!("{:>width$}", line_count, width = max_digits as usize);
        let word_count_str = format!("{:>width$}", word_count, width = max_digits as usize);
        let char_count_str = format!("{:>width$}", char_count, width = max_digits as usize);

        if args.l {
            let _ = write!(std::io::stdout(), "{} ", line_count_str);
        }
        if args.w {
            let _ = write!(std::io::stdout(), "{} ", word_count_str);
        }
        if args.c {
            let _ = write!(std::io::stdout(), "{} ", byte_count_str);
        }
        if args.m {
            let _ = write!(std::io::stdout(), "{} ", char_count_str);
        }

        let _ = writeln!(std::io::stdout(), "{}", first);
    }
    Ok(())
}

fn max_digits(byte_count: usize, char_count: usize, line_count: usize, word_count: usize) -> usize {
    let mut max_len = 0;

    let len_bytes = byte_count.to_string().len();
    if len_bytes > max_len {
        max_len = len_bytes;
    }

    let len_chars = char_count.to_string().len();
    if len_chars > max_len {
        max_len = len_chars;
    }

    let len_lines = line_count.to_string().len();
    if len_lines > max_len {
        max_len = len_lines;
    }

    let len_words = word_count.to_string().len();
    if len_words > max_len {
        max_len = len_words;
    }

    if max_len < 7 {
        max_len = 7;
    }
    max_len + 1
}
