use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("notes/foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}
