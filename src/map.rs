use crate::generators::{Generator, Prims};
use rand::Rng;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// References:
// https://en.wikipedia.org/wiki/Maze_generation_algorithm
// https://www.redblobgames.com/pathfinding/a-star/introduction.html
// https://www.redblobgames.com/pathfinding/a-star/implementation.html

/// A struct representing a single grid cell in a 2D maze. All cells start out as walls,
/// i.e. where `visited` is `false`.
#[derive(Copy, Clone, Debug)]
pub struct Cell {
    // Whether or not this cell has already been processed ("visited")
    pub(crate) visited: bool,

    // Whether or not we can travel north from this cell
    pub n: bool,

    // Whether or not we can travel south from this cell
    pub s: bool,

    // Whether or not we can travel east from this cell
    pub e: bool,

    // Whether or not we can travel west from this cell
    pub w: bool,
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
pub struct Map {
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

        let generator = Prims {};

        map.build_maze(generator);
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
    fn build_maze(&mut self, generator: impl Generator) {
        generator.build(self);
    }

    /// Opens a path between cells `to` and `from`. For example, if `to` is
    /// above `from` on the map, then `to`'s "south" flag will be set to `true`,
    /// meaning that the user can travel south from `to` down to `from` and vice-versa.
    pub(crate) fn open_path_between(&mut self, to: (usize, usize), from: (usize, usize)) {
        if !self.get_neighbors(to.0, to.1).contains(&from) {
            panic!("Attempting to open a path between non-adjacent cells");
        }

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

    /// Returns a random pair of valid grid indices.
    pub(crate) fn get_random_grid_indices(&self) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        (
            rng.gen_range(0, self.dimensions.0),
            rng.gen_range(0, self.dimensions.1),
        )
    }

    /// Returns an immutable reference to cell <`i`, `j`>, where `i` is the row
    /// and `j` is the column.
    pub fn get_cell(&self, i: usize, j: usize) -> &Cell {
        &self.terrain[i * self.dimensions.1 + j]
    }

    /// Returns a mutable reference to cell <`i`, `j`>, where `i` is the row
    /// and `j` is the column.
    pub fn get_cell_mut(&mut self, i: usize, j: usize) -> &mut Cell {
        &mut self.terrain[i * self.dimensions.1 + j]
    }

    pub fn visit(&mut self, i: usize, j: usize) {
        self.get_cell_mut(i, j).visited = true;
    }

    pub(crate) fn get_unvisited_neighbors(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        self.get_neighbors(i, j)
            .iter()
            .cloned()
            .filter(|(ni, nj)| !self.get_cell(*ni, *nj).visited)
            .collect()
    }

    pub(crate) fn get_visited_neighbors(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        self.get_neighbors(i, j)
            .iter()
            .cloned()
            .filter(|(ni, nj)| self.get_cell(*ni, *nj).visited)
            .collect()
    }

    /// Returns the indices of all of the valid neighbors of cell <`i`, `j`>,
    /// respecting the borders of the map.
    pub fn get_neighbors(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
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

    pub fn get_open_neighbors(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        let cell = self.get_cell(i, j);

        // Top
        if cell.n && i > 0 {
            neighbors.push((i - 1, j + 0));
        }

        // Bottom
        if cell.s && i < self.dimensions.0 - 1 {
            neighbors.push((i + 1, j + 0))
        }

        // Left
        if cell.w && j > 0 {
            neighbors.push((i + 0, j - 1))
        }

        // Right
        if cell.e && j < self.dimensions.1 - 1 {
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
    fn test_get_neighbors_0() {
        let map = Map::new((4, 4));

        let actual = map.get_neighbors(0, 0);
        let expected = vec![(1, 0), (0, 1)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_neighbors_1() {
        let map = Map::new((4, 4));

        let actual = map.get_neighbors(1, 1);
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
