use std::collections::HashSet;

pub struct Board {
    pub cells: HashSet<(i32, i32)>,
    pub rows: i32,
    pub cols: i32,
}

impl Board {
    pub fn new(r: i32, c: i32) -> Self {
        Self {
            cells: HashSet::new(),
            rows: r,
            cols: c,
        }
    }

    pub fn get_cell(&self, row: i32, col: i32) -> bool {
        self.cells.contains(&(row, col))
    }
    pub fn set_cell(&mut self, row: i32, col: i32, value: bool) {
        if value {
            self.cells.insert((row, col));
        } else {
            self.cells.remove(&(row, col));
        }
    }
    pub fn flip_cell(&mut self, row: i32, col: i32) {
        self.set_cell(row, col, !self.get_cell(row, col))
    }

    pub fn print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.get_cell(row, col) {
                    print!("0");
                } else {
                    print!("*");
                }
            }
            println!();
        }
    }

    pub fn get_alive_neighbor_count(&self, row: i32, col: i32) -> i32 {
        let mut neighbor_count = 0;
        for i in -1..2 {
            for j in -1..2 {
                if (i != 0 || j != 0) && self.get_cell(row + i, col + j) {
                    neighbor_count += 1
                }
            }
        }
        neighbor_count
    }
    pub fn iterate(&mut self) {
        let mut new_cells: HashSet<(i32, i32)> = HashSet::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell_state = self.get_cell(row, col);
                let ncount = self.get_alive_neighbor_count(row, col);

                if cell_state {
                    if ncount == 2 || ncount == 3 {
                        new_cells.insert((row, col));
                    }
                } else {
                    if ncount == 3 {
                        new_cells.insert((row, col));
                    }
                }
            }
        }

        self.cells = new_cells;
    }
}
