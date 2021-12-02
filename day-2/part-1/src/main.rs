use std::io::{self, BufRead};
use std::fmt;
use std::option::Option;
use std::result;

#[derive(Debug)]
struct OpError {
    operation: String,
}

impl OpError {
    fn new(operation: String) -> OpError {
        OpError{operation: operation.to_string()}
    }
}

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid operation: {}", self.operation)
    }
}

enum Operation {
    Forward(i32),
    Up(i32),
    Down(i32)
}

fn parse_op_name(s: &str, delta: i32) -> Option<Operation> {
    match s {
        "forward" => Some(Operation::Forward(delta)),
        "up"      => Some(Operation::Up(delta)),
        "down"    => Some(Operation::Down(delta)),
        _         => None
    }
}

fn parse_op(s: &String) -> result::Result<Operation, OpError> {
    let mut split = s.split(" ");

    let op_name = split
        .next()
        .ok_or_else(|| OpError::new(s.clone()))?;

    let delta: i32 = split
        .next()
        .ok_or_else(|| OpError::new(s.clone()))?
        .trim()
        .parse()
        .map_err(|_| OpError::new(s.clone()))?;

    let op = parse_op_name(op_name, delta)
        .ok_or_else(|| OpError::new(s.clone()))?;

    if split.count() > 0 {
        return Err(OpError::new(s.clone()));
    }

    Ok(op)
}

fn read_op<R: BufRead>(r: &mut R) -> result::Result<Option<Operation>, OpError> {
    let mut buf = String::new();

    if r.read_line(&mut buf).unwrap() > 0 {
        Ok(Some(parse_op(&buf)?))
    } else {
        Ok(None)
    }
}

fn main() {
    let mut x: i32 = 0;
    let mut z: i32 = 0;

    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    while let Some(operation) = read_op(&mut stdin_lock).unwrap() {
        match operation {
            Operation::Up(delta)      => z += delta,
            Operation::Down(delta)    => z -= delta,
            Operation::Forward(delta) => x += delta,
        }
    }

    println!("{}", x * -z);
}
