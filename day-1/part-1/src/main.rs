use std::io::{self, BufRead};
use std::option::Option;

fn read_depth<R: BufRead>(r: &mut R) -> io::Result<Option<u32>> {
    let mut buf = String::new();
    let bytes_read = r.read_line(&mut buf)?;

    if bytes_read > 0 {
        Ok(Some(buf.trim().parse().unwrap()))
    } else {
        Ok(None)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut count: u32 = 0;
    let mut prev = read_depth(&mut stdin_lock)?.unwrap();

    while let Some(curr) = read_depth(&mut stdin_lock)? {
        if curr > prev {
            count += 1;
        }

        prev = curr;
    }

    println!("{}", count);

    Ok(())
}
