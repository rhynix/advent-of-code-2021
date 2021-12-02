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
    let mut curr: [u32; 3] = [0; 3];
    let mut prev: [u32; 3];

    curr[0] = read_depth(&mut stdin_lock)?.unwrap();
    curr[1] = read_depth(&mut stdin_lock)?.unwrap();
    curr[2] = read_depth(&mut stdin_lock)?.unwrap();
    prev = curr;

    while let Some(value) = read_depth(&mut stdin_lock)? {
        curr[0] = curr[1];
        curr[1] = curr[2];
        curr[2] = value;

        if curr.iter().sum::<u32>() > prev.iter().sum::<u32>() {
            count += 1;
        }

        prev = curr;
    }

    println!("{}", count);

    Ok(())
}
