#![allow(dead_code)]

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

enum Generator {
    RandomizedPrims,
    RecursiveBacktracking,
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

    /// Returns `true` if all passages leading to this cell are "open," and
    /// `false` otherwise.
    pub fn is_completely_open(&self) -> bool {
        self.n && self.s && self.e && self.w
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
        println!(
            "Building map with {} rows and {} columns",
            dimensions.0, dimensions.1
        );

        let mut map = Map {
            dimensions,
            terrain: vec![Cell::new(); dimensions.0 * dimensions.1],
        };

        map.build_maze(Generator::RecursiveBacktracking);
        map
    }

    /// Returns the dimensions (width, height) of the map.
    pub fn get_dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    /// Returns an immutable reference to the map's terrain, which is a 1D
    /// vector of `Cell` structs.
    pub fn get_terrain(&self) -> &Vec<Cell> {
        &self.terrain
    }

    /// Saves an ASCII art representation of the maze to `path`.
    pub fn save_ascii(&self, path: &Path) -> std::io::Result<()> {
        let mut file = File::create(path)?;

        // Add a BOM unicode character (maybe not always necessary?)
        let header = vec![0xEF, 0xBB, 0xBF];
        let bom = std::str::from_utf8(&header).unwrap();

        file.write_all(&format!("{}{:?}", bom, self).as_bytes())?;

        Ok(())
    }

    /// Builds a maze using the specified `generator`.
    fn build_maze(&mut self, generator: Generator) {
        match generator {
            Generator::RandomizedPrims => self.randomized_prims(false),
            Generator::RecursiveBacktracking => self.recursive_backtracking(false),
        }
    }

    /// A method for randomly generating mazes.
    ///
    /// Reference: `https://en.wikipedia.org/wiki/Maze_generation_algorithm`
    fn recursive_backtracking(&mut self, save_progress: bool) {
        let mut rng = rand::thread_rng();

        // Start at a random cell
        let mut current_indices = (
            rng.gen_range(0, self.dimensions.0),
            rng.gen_range(0, self.dimensions.1),
        );
        self.get_cell_mut(current_indices.0, current_indices.1)
            .visited = true;

        // Set up a stack for backtracking
        let mut stack: Vec<(usize, usize)> = vec![];

        let mut iteration = 0;

        'outer: loop {
            // Save out .txt files as the maze is being built
            if save_progress {
                self.save_ascii(Path::new(&format!("iteration_{}.txt", iteration)))
                    .unwrap();
                iteration += 1;
            }

            let potential_paths = self.get_unvisited_neighbor_indices(current_indices.0, current_indices.1);

            if potential_paths.is_empty() {
                'inner: loop {
                    if let Some(indices) = stack.pop() {
                        // Work backwards and find the first cell that has at least one "closed" wall
                        if !self.get_cell(indices.0, indices.1).is_completely_open() {
                            // Set this to the current cell and return to the beginning
                            current_indices = indices;
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
            let random_index = rng.gen_range(0, potential_paths.len());
            let from = potential_paths[random_index];
            let to = current_indices;

            // Open a path between the last cell and the chosen neighbor
            self.open_path_between(to, from);

            // Mark the current cell as `visited` and recurse
            current_indices = from;
            self.get_cell_mut(current_indices.0, current_indices.1)
                .visited = true;

            // Add this cell to the stack - it may be visited again in the backwards pass
            stack.push(current_indices);
        }
        if save_progress {
            self.save_ascii(Path::new(&format!("iteration_{}.txt", iteration)))
                .unwrap();
        }
    }

    /// Builds a valid, "solvable" maze using a randomized version of Prim's
    /// algorithm.
    ///
    /// Reference: `https://en.wikipedia.org/wiki/Maze_generation_algorithm`
    fn randomized_prims(&mut self, save_progress: bool) {
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

        let mut iteration = 0;

        // Keep going until there are no more candidate cells
        while !frontier.is_empty() {
            // Save out .txt files as the maze is being built
            if save_progress {
                self.save_ascii(Path::new(&format!("iteration_{}.txt", iteration)))
                    .unwrap();
                iteration += 1;
            }

            // Choose one of the frontier cells at random
            let random_index = rng.gen_range(0, frontier.len());
            let next_indices = frontier.remove(random_index);

            // Set this cell's `visited` flag, denoting that it is now part of the final maze
            self.get_cell_mut(next_indices.0, next_indices.1).visited = true;

            let potential_paths = self.get_visited_neighbor_indices(next_indices.0, next_indices.1);
            let potential_front = self.get_unvisited_neighbor_indices(next_indices.0, next_indices.1);

            // Choose one of the visited neighbors at random
            let random_index = rng.gen_range(0, potential_paths.len());
            let from = potential_paths[random_index];
            let to = next_indices;

            // Open a path between the last cell and the chosen neighbor
            self.open_path_between(to, from);

            if frontier.is_empty() && potential_front.is_empty() {
                break;
            }

            // Build up the frontier, keeping sure to not re-add indices that are
            // already part of the frontier
            for neighbor in potential_front.iter() {
                if !frontier.contains(neighbor) {
                    frontier.push(*neighbor);
                }
            }
        }
        if save_progress {
            self.save_ascii(Path::new(&format!("iteration_{}.txt", iteration)))
                .unwrap();
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

    /// Given a 1D index into this map's array of cells, returns the 2D
    /// grid index <`i`, `j`> corresponding to this cell's position in the
    /// terrain.
    fn absolute_to_grid_indices(&self, idx: usize) -> (usize, usize) {
        // Row
        let i = idx / self.dimensions.1;

        // Column
        let j = idx % self.dimensions.1;

        (i, j)
    }

    /// Returns an immutable reference to cell <`i`, `j`>, where `i` is the row
    /// and `j` is the column.
    fn get_cell(&self, i: usize, j: usize) -> &Cell {
        &self.terrain[i * self.dimensions.1 + j]
    }

    /// Returns a mutable reference to cell <`i`, `j`>, where `i` is the row
    /// and `j` is the column.
    fn get_cell_mut(&mut self, i: usize, j: usize) -> &mut Cell {
        &mut self.terrain[i * self.dimensions.1 + j]
    }

    /// Sets cell <`i`, `j`> to `cell` (effectively replacing the old cell).
    fn set_cell(&mut self, i: usize, j: usize, cell: &Cell) {
        *self.get_cell_mut(i, j) = *cell;
    }

    fn get_unvisited_neighbor_indices(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        self.get_neighbor_indices(i, j)
            .iter()
            .cloned()
            .filter(|(ni, nj)| !self.get_cell(*ni, *nj).visited)
            .collect()
    }

    fn get_visited_neighbor_indices(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        self.get_neighbor_indices(i, j)
            .iter()
            .cloned()
            .filter(|(ni, nj)| self.get_cell(*ni, *nj).visited)
            .collect()
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
        for row in 0..self.dimensions.0 {
            // Print the line above this row
            for col in 0..self.dimensions.1 {
                // Can we move up from this cell?
                if self.get_cell(row, col).n {
                    write!(f, "◼◻◻")?;
                } else {
                    write!(f, "◼◼◼")?;
                }
                if col == self.dimensions.1 - 1 {
                    write!(f, "◼\n")?;
                }
            }

            // Print the middle (cell) line (twice, because of unicode spacing)
            for _ in 0..2 {
                for col in 0..self.dimensions.1 {
                    if self.get_cell(row, col).visited {
                        // Can we move left from this cell?
                        if self.get_cell(row, col).w {
                            write!(f, "◻◻◻")?;
                        } else {
                            write!(f, "◼◻◻")?;
                        }
                    } else {
                        write!(f, "◼◼◼")?;
                    }
                    if col == self.dimensions.1 - 1 {
                        write!(f, "◼\n")?;
                    }
                }
            }

            // If this is the last row, add an additional line of chars below
            if row == self.dimensions.0 - 1 {
                for _ in 0..self.dimensions.1 {
                    write!(f, "◼◼◼")?;
                }
                write!(f, "◼\n")?;
            }
        }
        Ok(())
    }
}

// See: https://www.joshmcguigan.com/blog/custom-exit-status-codes-rust/
fn main() -> std::io::Result<()> {
    let map = Map::new((30, 30));
    map.save_ascii(Path::new("maze.txt"))?;
    println!("{:?}", map);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absolute_to_grid_indices() {
        let map = Map::new((3, 4));

        let mut actual = vec![];

        for idx in 0..map.get_terrain().len() {
            actual.push(map.absolute_to_grid_indices(idx));
        }

        let expected = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 0),
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 0),
            (2, 1),
            (2, 2),
            (2, 3),
        ];

        assert_eq!(actual, expected);
    }
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
    fn test_open_path_between_0() {
        let mut map = Map::new((4, 4));
        map.open_path_between((0, 0), (1, 0));

        let to = map.get_cell(0, 0);
        let from = map.get_cell(1, 0);

        assert!(to.s);
        assert!(from.n);
    }

    #[test]
    fn test_open_path_between_1() {
        let mut map = Map::new((4, 4));
        map.open_path_between((0, 0), (0, 1));

        let to = map.get_cell(0, 0);
        let from = map.get_cell(0, 1);

        assert!(to.e);
        assert!(from.w);
    }
}
