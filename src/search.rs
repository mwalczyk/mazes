use crate::map::Map;
use std::collections::HashMap;

// Reference: https://www.redblobgames.com/pathfinding/a-star/implementation.html
pub fn breadth_first(map: &Map, from: (usize, usize), to: (usize, usize)) -> Vec<(usize, usize)> {
    // The cells that still need to be processed
    let mut frontier = vec![];
    frontier.push(from);

    // A map that tells us which cell a given cell "came from" during traversal
    let mut came_from = HashMap::new();
    came_from.insert(from, from);

    loop {
        if let Some(current_indices) = frontier.pop() {
            // Get this cell's neighbors
            //let neighbors = map.get_neighbor_indices(current_indices.0, current_indices.1);

            // TODO: this should be handled in the cell struct or something
            let mut neighbors = vec![];
            let cell = map.get_cell(current_indices.0, current_indices.1);
            if cell.n {
                neighbors.push((current_indices.0 - 1, current_indices.1 + 0));
            }
            if cell.s {
                neighbors.push((current_indices.0 + 1, current_indices.1 + 0));
            }
            if cell.e {
                neighbors.push((current_indices.0 + 0, current_indices.1 + 1));
            }
            if cell.w {
                neighbors.push((current_indices.0 + 0, current_indices.1 - 1));
            }

            for neighbor_indices in neighbors.iter() {
                // If this neighbor hasn't already been visited
                if !came_from.contains_key(neighbor_indices) {
                    frontier.push(*neighbor_indices);
                    came_from.insert(*neighbor_indices, current_indices);
                }
            }
        } else {
            // The frontier is empty
            break;
        }
    }

    // Reconstruct the path from `from` to `to` by indexing into the map data structure
    let mut current_indices = to;
    let mut path = vec![];

    while current_indices != from {
        println!("{:?}", current_indices);
        path.push(current_indices);
        current_indices = came_from[&current_indices];
    }
    path.push(from);
    println!("{:?}", from);

    path
}
