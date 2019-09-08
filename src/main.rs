#![allow(dead_code)]

mod map;

use map::Map;
use std::path::Path;

// See: https://www.joshmcguigan.com/blog/custom-exit-status-codes-rust/
fn main() -> std::io::Result<()> {
    let map = Map::new((30, 30));
    map.save_ascii(Path::new("maze.txt"))?;
    println!("{:?}", map);

    Ok(())
}

