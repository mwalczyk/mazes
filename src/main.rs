#![allow(dead_code)]

mod map;
mod search;

use map::Map;
use std::path::Path;
use crate::search::breadth_first;

// See: https://www.joshmcguigan.com/blog/custom-exit-status-codes-rust/
fn main() -> std::io::Result<()> {
    let map = Map::new((30, 30));
    map.save_ascii(Path::new("maze.txt"))?;
    println!("{:?}", map);

    let path = breadth_first(&map, (0, 0), (29, 29));
    //println!("{:?}", path);

    Ok(())
}
