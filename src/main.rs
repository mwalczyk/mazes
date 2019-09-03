use rand::Rng;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// References: https://en.wikipedia.org/wiki/Maze_generation_algorithm

/// An enum representing a cardinal direction, i.e. "north."
enum Direction {
    N,
    S,
    E,
    W,
}

/// A struct representing a single grid cell in a 2D maze.
#[derive(Copy, Clone, Debug)]
struct Cell {
    // Whether or not this cell has already been processed ("visited")
    visited: bool,

    // Whether or not we can travel north from this cell
    n: bool,

    // Whether or not we can travel south from this cell
    s: bool,

    // Whether or not we can travel east from this cell
    e: bool,

    // Whether or not we can travel west from this cell
    w: bool,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            visited: false,
            n: false,
            s: false,
            e: false,
            w: false,
        }
    }
}

/// A struct representing a game map that is filled from edge-to-edge by a
/// 2-dimensional maze.
struct Map {
    // The dimensions of the map (width, height)
    dimensions: (usize, usize),

    // The actual map data (a 1D-array of cells, interpreted as a 2D-array)
    terrain: Vec<Cell>,
}

impl Map {
    /// Constructs and populates a new map.
    pub fn new(dimensions: (usize, usize)) -> Map {
        let mut map = Map {
            dimensions,
            terrain: vec![Cell::new(); dimensions.0 * dimensions.1],
        };

        map.build_maze();
        map
    }

    /// Saves an ASCII art representation of the maze to `path`.
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&format!("{:?}", self).as_bytes());

        Ok(())
    }

    /// Builds a valid, "solvable" maze using a randomized version of Prim's
    /// algorithm.
    fn build_maze(&mut self) {
        let mut rng = rand::thread_rng();

        // We could use a `HashSet` here, but Rust's `HashSet` does not offer constant
        // time indexing, which we need below
        let mut frontier = vec![];

        // Start at a random cell
        let start_indices = (
            rng.gen_range(0, self.dimensions.0),
            rng.gen_range(0, self.dimensions.1),
        );
        self.get_cell_mut(start_indices.0, start_indices.1).visited = true;

        // To kick off the recursion, add the "starting" cell's neighbors - this builds
        // a "frontier" of candidate cells
        frontier.extend_from_slice(&self.get_neighbor_indices(start_indices.0, start_indices.1));

        // Keep going until there are no more candidate cells
        while !frontier.is_empty() {
            // Choose one of the frontier cells at random
            let random_index = rng.gen_range(0, frontier.len());
            let next_indices = frontier.remove(random_index);

            // Set this cell's `visited` flag, denoting that it is now part of the final maze
            self.get_cell_mut(next_indices.0, next_indices.1).visited = true;

            let neighbors = self.get_neighbor_indices(next_indices.0, next_indices.1);

            let mut potential_paths = vec![];
            let mut potential_front = vec![];

            for neighbor in neighbors.iter() {
                if self.get_cell(neighbor.0, neighbor.1).visited {
                    // This cell has been visited already - we can build a path from it
                    potential_paths.push(*neighbor);
                } else {
                    // This cell hasn't been visited yet - it will be added to the frontier
                    if !frontier.contains(neighbor) {
                        potential_front.push(*neighbor);
                    }
                }
            }

            // Choose one of the visited neighbors at random
            let random_index = rng.gen_range(0, potential_paths.len());
            let from = potential_paths.remove(random_index);
            let to = next_indices;

            // Open a path between the last cell and the chosen neighbor
            self.open_path_between(to, from);

            if frontier.is_empty() && potential_front.is_empty() {
                break;
            }

            // Build up the frontier
            frontier.extend_from_slice(&potential_front);
        }

    }

    /// Opens a path between cells `to` and `from`. For example, if `to` is
    /// above `from` on the map, then `to`'s "south" flag will be set to `true`,
    /// meaning that the user can travel south from `to` down to `from` and vice-versa.
    fn open_path_between(&mut self, to: (usize, usize), from: (usize, usize)) {
        if to.0 < from.0 {
            // `to` is above `from`: we can move down from `to` and up from `from`
            self.get_cell_mut(to.0, to.1).s = true;
            self.get_cell_mut(from.0, from.1).n = true;
        }
        if to.0 > from.0 {
            // `to` is below `from`: we can move up from `to` and down from `from`
            self.get_cell_mut(to.0, to.1).n = true;
            self.get_cell_mut(from.0, from.1).s = true;
        }
        if to.1 < from.1 {
            // `to` is left from `from`: we can move right from `to` and left from `from`
            self.get_cell_mut(to.0, to.1).e = true;
            self.get_cell_mut(from.0, from.1).w = true;
        }
        if to.1 > from.1 {
            // `to` is right from `from`: we can move left from `to` and right from `from`
            self.get_cell_mut(to.0, to.1).w = true;
            self.get_cell_mut(from.0, from.1).e = true;
        }
    }

    /// Returns an immutable reference to cell <`i`, `j`>, where `i` is the row
    /// and `j` is the column.
    fn get_cell(&self, i: usize, j: usize) -> &Cell {
        &self.terrain[i * self.dimensions.0 + j]
    }

    /// Returns a mutable reference to cell <`i`, `j`>, where `i` is the row
    /// and `j` is the column.
    fn get_cell_mut(&mut self, i: usize, j: usize) -> &mut Cell {
        &mut self.terrain[i * self.dimensions.0 + j]
    }

    /// Sets cell <`i`, `j`> to `cell` (effectively replacing the old cell).
    fn set_cell(&mut self, i: usize, j: usize, cell: &Cell) {
        *self.get_cell_mut(i, j) = *cell;
    }

    /// Returns the indices of all of the valid neighbors of cell <`i`, `j`>,
    /// respecting the borders of the map.
    fn get_neighbor_indices(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];

        // Top
        if i > 0 {
            neighbors.push((i - 1, j + 0))
        }

        // Bottom
        if i < self.dimensions.0 - 1 {
            neighbors.push((i + 1, j + 0))
        }

        // Left
        if j > 0 {
            neighbors.push((i + 0, j - 1))
        }

        // Right
        if j < self.dimensions.1 - 1 {
            neighbors.push((i + 0, j + 1))
        }

        neighbors
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row_index in 0..self.dimensions.0 {
            // Print line above
            for col_index in 0..self.dimensions.1 {
                // Can we move up from this cell?
                if self.get_cell(row_index, col_index).n {
                    write!(f, "+  ");
                } else {
                    write!(f, "+--");
                }
                if col_index == self.dimensions.1 - 1 {
                    write!(f, "+\n");
                }
            }

            // Print middle (cell) line
            for col_index in 0..self.dimensions.1 {
                // Can we move left from this cell?
                if self.get_cell(row_index, col_index).w {
                    write!(f, "   ");
                } else {
                    write!(f, "|  ");
                }
                if col_index == self.dimensions.1 - 1 {
                    write!(f, "|\n");
                }
            }

            if row_index == self.dimensions.1 - 1 {
                for _ in 0..self.dimensions.1 {
                    write!(f, "+--");
                }
                write!(f, "+\n");
            }
        }
        Ok(())
    }
}

//impl std::fmt::Debug for Map {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        for row_index in 0..self.dimensions.0 {
//            // Print line above
//            for col_index in 0..self.dimensions.1 {
//                // Can we move up from this cell?
//                if self.get_cell(row_index, col_index).n {
//                    write!(f, "◼◻◻◻");
//                } else {
//                    write!(f, "◼◼◼◼");
//                }
//                if col_index == self.dimensions.1 - 1 {
//                    write!(f, "◼\n");
//                }
//            }
//
//            // Print middle (cell) line
//            for col_index in 0..self.dimensions.1 {
//                // Can we move left from this cell?
//                if self.get_cell(row_index, col_index).w {
//                    write!(f, "◻◻◻◻");
//                } else {
//                    write!(f, "◼◻◻◻");
//                }
//                if col_index == self.dimensions.1 - 1 {
//                    write!(f, "◼\n");
//                }
//            }
//
//            if row_index == self.dimensions.1 - 1 {
//                for _ in 0..self.dimensions.1 {
//                    write!(f, "◼◼◼◼");
//                }
//                write!(f, "◼\n");
//            }
//        }
//        Ok(())
//    }
//}

fn main() {
    let mut map = Map::new((3, 3));
    //map.build_maze();
    map.save(Path::new("maze.txt"));
    println!("{:?}", map);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_neighbor_indices_0() {
        let map = Map::new((4, 4));

        let actual = map.get_neighbor_indices(0, 0);
        let expected = vec![(1, 0), (0, 1)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_neighbor_indices_1() {
        let map = Map::new((4, 4));

        let actual = map.get_neighbor_indices(1, 1);
        let expected = vec![(0, 1), (2, 1), (1, 0), (1, 2)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_open_path_between() {
        let mut map = Map::new((4, 4));
        map.open_path_between((0, 0), (1, 0));

        let to = map.get_cell(0, 0);
        let from = map.get_cell(1, 0);

        assert!(to.s);
        assert!(from.n);
    }
}
