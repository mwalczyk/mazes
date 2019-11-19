use crate::map::Map;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

// The idea to move this into a trait was inspired by:
//
// Reference: https://github.com/CianLR/mazegen-rs
pub trait Generator {
    fn build(&self, map: &mut Map);

    fn each_iteration(&self, map: &Map) {
        // Move up cursor
        println!("\x1b[{}F", map.get_dimensions().0 + 2);
        println!("{:?}", map);

        let duration = 100;
        sleep(Duration::from_millis(duration));
    }
}

/// A randomized Prim's algorithm
pub struct Prims {}

impl Generator for Prims {
    /// Builds a valid, "solvable" maze using a randomized version of Prim's
    /// algorithm.
    ///
    /// Reference: `https://en.wikipedia.org/wiki/Maze_generation_algorithm`
    fn build(&self, map: &mut Map) {
        let mut rng = rand::thread_rng();
        let mut current = map.get_random_grid_indices();
        map.visit(current.0, current.1);

        let mut frontier = map.get_neighbors(current.0, current.1);

        while !frontier.is_empty() {
            //self.each_iteration(map);

            // Two flags: IN and FRONTIER
            //
            // Mark the first cell (set it to IN and FRONTIER)
            //
            // Grab a random cell from the frontier
            // Look at its list of neighbors that are already in the maze, i.e. IN is true
            // Pull one randomly, carve a path
            // Mark the current cell:
            // 1. Set it to IN
            // 2. Add all UNVISITED neighbors to the frontier (also, avoiding ones that
            //    already have a FRONTIER flag set, i.e. they've been added before?)

            current = frontier.remove(rng.gen_range(0, frontier.len()));

            if map.get_cell(current.0, current.1).visited {
                // This neighbor is already part of the maze
                continue;
            }
            map.visit(current.0, current.1);

            // remove wall between last and current
            // add unvisited neighbors to frontier

            let neighbors = map.get_neighbors(current.0, current.1);

            let potential_paths= map.get_visited_neighbors(current.0, current.1);

            // Choose one of the visited neighbors at random
            let from = potential_paths[rng.gen_range(0, potential_paths.len())];
            let to = current;
            map.open_path_between(to, from);




            frontier.extend_from_slice(&neighbors);
        }
    }
}

/// A recursive backtracking algorithm
pub struct Backtracking {}

impl Generator for Backtracking {
    /// A method for randomly generating mazes.
    ///
    /// Reference: `https://en.wikipedia.org/wiki/Maze_generation_algorithm`
    fn build(&self, map: &mut Map) {
        let mut rng = rand::thread_rng();
        let mut current = map.get_random_grid_indices();
        map.visit(current.0, current.1);

        // Set up a stack for backtracking
        let mut stack: Vec<(usize, usize)> = vec![];

        'outer: loop {
            let potential_paths = map.get_unvisited_neighbors(current.0, current.1);

            if potential_paths.is_empty() {
                'inner: loop {
                    if let Some(indices) = stack.pop() {
                        // Work backwards and find the first cell that has at least one "closed" off wall
                        if !map.get_cell(indices.0, indices.1).is_completely_open() {
                            current = indices;
                            break 'inner;
                        }
                    } else {
                        // The stack is empty - end the recursion
                        break 'outer;
                    }
                }

                // We have a new "starting" cell - go back to the beginning of the algorithm
                continue;
            }

            // Choose one of the unvisited neighbors at random
            let from = potential_paths[rng.gen_range(0, potential_paths.len())];
            let to = current;
            map.open_path_between(to, from);

            // Mark the current cell as `visited` and recurse
            current = from;
            map.visit(current.0, current.1);

            // Add this cell to the stack - it may be visited again in the backwards pass
            stack.push(current);
        }
    }
}
