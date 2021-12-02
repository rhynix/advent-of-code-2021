use std::io::{self, BufRead};
use std::fmt;
use std::result;
use std::process::exit;
use std::num;

#[derive(Debug)]
struct AppError {
    description: String
}

impl AppError {
    fn new(description: String) -> AppError {
        AppError{ description: description }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> AppError {
        AppError::new(err.to_string())
    }
}

impl From<num::ParseIntError> for AppError {
    fn from(_: num::ParseIntError) -> AppError {
        AppError::new("Parse error".to_string())
    }
}

fn run() -> result::Result<u32, AppError> {
    let mut zeros: [u32; 12] = [0; 12];
    let mut ones:  [u32; 12] = [0; 12];

    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf = String::new();
    let mut linenr = 0;

    while stdin_lock.read_line(&mut buf)? > 0 {
        let mut chrs = buf.trim().chars();
        linenr += 1;

        for idx in 0..12 {
            let chr = chrs
                .next()
                .ok_or_else(|| {
                    AppError::new(
                        format!("Too few characters on line {}", linenr)
                    )
                })?;

            if chr == '0' {
                zeros[idx] += 1;
            }

            if chr == '1' {
                ones[idx] += 1;
            }
        }

        buf.clear();
    }

    let mut gamma   = 0;
    let mut epsilon = 0;

    for idx in 0..12 {
        gamma   = gamma   << 1;
        epsilon = epsilon << 1;

        if ones[idx] > zeros[idx] {
            gamma = gamma | 1;
        } else {
            epsilon = epsilon | 1;
        }
    }

    Ok(gamma * epsilon)
}

fn main() {
    match run() {
        Ok(val) => {
            println!("{}", val);
            exit(0);
        },
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    }
}
