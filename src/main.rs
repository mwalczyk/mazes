use rand::Rng;

//http://weblog.jamisbuck.org/2011/1/10/maze-generation-prim-s-algorithm
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Copy, Clone)]
struct Cell {
    visited: bool,
    n: bool,
    s: bool,
    e: bool,
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

type Index = (usize, usize);

struct Map {
    // The dimensions of the map (width, height)
    dimensions: (usize, usize),

    // The actual map data (a list-of-lists containing cells)
    terrain: Vec<Vec<Cell>>,
}

impl Map {
    pub fn new(dimensions: (usize, usize)) -> Map {
        Map {
            dimensions,
            terrain: vec![vec![Cell::new(); dimensions.0]; dimensions.1]
        }
    }

    fn open_path_between(&mut self, to: (usize, usize), from: (usize, usize)) {
        if to.0 < from.0 {
            // `to` is above `from`: we can move down from `to` and up from `from`
            self.terrain[to.0][to.1].s = true;
            self.terrain[from.0][from.1].n = true;
        }
        if to.0 > from.0 {
            // `to` is below `from`: we can move up from `to` and down from `from`
            self.terrain[to.0][to.1].n = true;
            self.terrain[from.0][from.1].s = true;
        }
        if to.1 < from.1 {
            // `to` is left from `from`: we can move right from `to` and left from `from`
            self.terrain[to.0][to.1].e = true;
            self.terrain[from.0][from.1].w = true;
        }
        if to.1 > from.1 {
            // `to` is right from `from`: we can move left from `to` and right from `from`
            self.terrain[to.0][to.1].w = true;
            self.terrain[from.0][from.1].e = true;
        }
    }

    fn build_maze(&mut self) {
        let mut frontier = vec![];

        // Set a random "starting" cell to `visited`, i.e. make it part of the final maze
        let start_i = 2;
        let start_j = 2;
        self.terrain[start_i][start_j].visited = true;

        // To kick off the recursion, add the "starting" cell's neighbors - this builds
        // a "frontier" of candidate cells
        frontier.extend_from_slice(&self.get_neighbor_indices(start_i, start_j));

        let mut rng = rand::thread_rng();

        // Keep going until there are no more candidate cells
        while !frontier.is_empty() {
            // Choose one of the frontier cells at random
            let random_index = rng.gen_range(0, frontier.len());
            let next = frontier.remove(random_index);

            // This cell is now part of the maze
            self.terrain[next.0][next.1].visited = true;

            // The path is built by connecting this cell to one of its neighbors that
            // is ON, i.e. already part of the maze (there will always be at least one)
            let neighbors = self.get_neighbor_indices(next.0, next.1);

            let mut potential_paths = vec![];
            let mut potential_frontier = vec![];
            for neighbor in neighbors.iter() {
                if self.terrain[neighbor.0][neighbor.1].visited {
                    // This cell has been visited already - we can build a path from it
                    potential_paths.push(*neighbor);
                } else {
                    // This cell hasn't been visited yet - it will be added to the frontier
                    if !frontier.contains(neighbor) {
                        potential_frontier.push(*neighbor);
                    }
                }
            }

            let to = next;
            let from = potential_paths.remove(rng.gen_range(0, potential_paths.len()));

            //println!("    creating path between {:?} and {:?}", to, from);
            self.open_path_between(to, from);


            frontier.extend_from_slice(&potential_frontier);
        }

        self.display();
    }

    pub fn display(&self) {
        for (row_index, row) in self.terrain.iter().enumerate() {
            // Print line above
            for (col_index, col) in row.iter().enumerate() {
                // Can we move up from this cell?
                if col.n {
                    print!("+  ");
                } else {
                    print!("+--");
                }
                if col_index == self.dimensions.1 - 1 {
                    print!("+\n");
                }
            }

            // Print middle (cell) line
            for (col_index, col) in row.iter().enumerate() {
                // Can we move left from this cell?
                if col.w {
                    print!("   ");
                } else {
                    print!("|  ");
                }
                if col_index == self.dimensions.1 - 1 {
                    print!("|\n");
                }
            }

            if row_index == self.terrain.len() - 1 {
                for _ in 0..self.dimensions.1 {
                    print!("+--");
                }
                print!("+\n");
            }
        }
    }

    pub fn get_cell(&self, i: usize, j: usize) -> Cell {
        // `i` is the row, `j` is the column
        self.terrain[i][j]
    }

    pub fn set_cell(&mut self, i: usize, j: usize, entry: char) {
        //self.terrain[i][j].data = entry;
    }

    /// Returns the neighboring values around cell <`i`, `j`> in the following
    /// order: top, down, left, right.
    pub fn get_neighbor_indices(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut neighbors= vec![];

        if i > 0 {
            neighbors.push((i - 1, j + 0))
        };
        if i < self.dimensions.0 - 1 {
            neighbors.push((i + 1, j + 0))
        }

        if j > 0 {
            neighbors.push((i + 0, j - 1))
        }
        if j < self.dimensions.1 - 1 {
            neighbors.push((i + 0, j + 1))
        }

        neighbors
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        for row in self.terrain.iter() {
            for col in row.iter() {
                //write!(f, "{:?}", data);
            }
        }
        Ok(())
    }
}

fn main() {
    let mut map = Map::new((10, 10));
    map.build_maze();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cell_0() {
        let map = Map::new((4, 4));
        //assert_eq!(add(1, 2), 3);
    }
}
