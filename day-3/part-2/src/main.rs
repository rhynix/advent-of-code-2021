use std::io::{self, BufRead, stdin};
use std::fmt;
use std::result;
use std::process::exit;
use std::vec::Vec;
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

type Result<T> = result::Result<T, AppError>;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Commonality {
    Least,
    Most,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Bit {
    Zero,
    One,
}

impl Bit {
    fn complement(self) -> Self {
        match self {
            Bit::Zero => Bit::One,
            Bit::One  => Bit::Zero,
        }
    }

    fn from_char(chr: char) -> Option<Self> {
        match chr {
            '0' => Some(Bit::Zero),
            '1' => Some(Bit::One),
            _   => None,
        }
    }
}

fn most_common_bit(bits: &Vec<Bit>) -> Option<Bit> {
    let zeros = bits.iter().filter(|item| item == &&Bit::Zero).count();
    let ones  = bits.iter().filter(|item| item == &&Bit::One).count();

    if zeros > ones {
        return Some(Bit::Zero);
    }

    if ones > zeros {
        return Some(Bit::One);
    }

    None
}

fn least_common_bit(bits: &Vec<Bit>) -> Option<Bit> {
    most_common_bit(bits).map(Bit::complement)
}

fn find_value(vec: &Vec<String>, common: Commonality) -> Result<u32> {
    let mut lines = vec.iter().collect::<Vec<&String>>();
    let mut idx: usize = 0;

    while lines.len() > 1 {
        if idx >= 12 {
            return Err(AppError::new("Multiple lines left".to_string()))
        }

        let bits = lines
            .iter()
            .map(|value| value.chars().nth(idx).and_then(Bit::from_char))
            .collect::<Option<Vec<Bit>>>()
            .unwrap();

        let required_bit = match common {
            Commonality::Most  => most_common_bit(&bits).unwrap_or(Bit::One),
            Commonality::Least => least_common_bit(&bits).unwrap_or(Bit::Zero),
        };

        lines = lines
            .into_iter()
            .filter(|value| {
                let chr = value.chars().nth(idx);
                let bit = chr.and_then(Bit::from_char).unwrap();

                bit == required_bit
            }).collect();
        idx += 1;
    }

    let line = lines
        .first()
        .ok_or_else(|| AppError::new("No lines left".to_string()))?;

    let value = u32::from_str_radix(line.as_str(), 2)?;

    Ok(value)
}

fn run() -> Result<u32> {
    let lines: Vec<String> = stdin()
        .lock()
        .lines()
        .collect::<result::Result<Vec<String>, _>>()?;

    let oxygen_generator_rating = find_value(&lines, Commonality::Most)?;
    let co2_scrubber_rating     = find_value(&lines, Commonality::Least)?;

    Ok(oxygen_generator_rating * co2_scrubber_rating)
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
