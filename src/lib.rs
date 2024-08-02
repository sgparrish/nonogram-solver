use bit_vec::BitVec;
use std::cmp;
use std::fmt;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Cell {
    Unknown = 0,
    Block = 1,
    Void = 2,
}

pub struct Nonogram {
    x_hints: Vec<Vec<u8>>,
    y_hints: Vec<Vec<u8>>,
    board: Vec<Cell>,
}

impl fmt::Display for Nonogram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let longest_x_hint = self.x_hints.iter().fold(0, |acc, e| cmp::max(acc, e.len()));
        let longest_y_hint = self.y_hints.iter().fold(0, |acc, e| cmp::max(acc, e.len()));

        let x_hint_pad = (longest_y_hint * 3) + 1;

        // Write x hints
        for i in 0..longest_x_hint {
            write!(f, "\n{: <1$}", "", x_hint_pad)?;
            for hints in self.x_hints.iter() {
                let hint_idx = (i as i32) + (hints.len() as i32) - (longest_x_hint as i32);
                let mut hint_txt = String::from("");
                if hint_idx >= 0 {
                    if let Some(one_hint) = hints.get(hint_idx as usize) {
                        hint_txt = one_hint.to_string();
                    }
                }
                write!(f, "{: >2} ", hint_txt)?;
            }
        }

        // Write y hints and board
        let mut y: usize = 0;
        for hints in self.y_hints.iter() {
            // y hints
            let y_hint_pad = (longest_y_hint - hints.len()) * 3;
            write!(f, "\n{: <1$}", "", y_hint_pad)?;
            for one_hint in hints {
                write!(f, "{: >2} ", one_hint)?;
            }

            // board
            let width = self.x_hints.len();
            for x in 0..width {
                let mut cell_txt = "    ";
                if let Some(cell) = self.board.get(x + (y * width)) {
                    cell_txt = match cell {
                        Cell::Unknown => "    ",
                        Cell::Block => "██",
                        Cell::Void => "><",
                    }
                }
                write!(f, "|{}", cell_txt)?;
            }
            write!(f, "|")?;
            y += 1;
        }

        Ok(())
    }
}

impl Nonogram {
    pub fn new(x_hints: Vec<Vec<u8>>, y_hints: Vec<Vec<u8>>) -> Nonogram {
        let board_size = x_hints.len() * y_hints.len();
        let mut board: Vec<Cell> = Vec::with_capacity(board_size);
        for _ in 0..board_size {
            board.push(Cell::Unknown);
        }

        Nonogram {
            x_hints,
            y_hints,
            board,
        }
    }

    fn get_row(&self, row_idx: usize) -> Vec<Cell> {
        let start = row_idx * self.y_hints.len();
        let end = start + self.x_hints.len();

        let mut cells = Vec::with_capacity(self.y_hints.len());
        for i in start..end {
            cells.push(self.board[i]);
        }
        cells
    }

    fn set_row(&mut self, row_idx: usize, cells: Vec<Cell>) {
        let start = row_idx * self.y_hints.len();
        let end = start + self.x_hints.len();

        let mut j = 0;
        for i in start..end {
            self.board[i] = cells[j];
            j += 1;
        }
    }

    fn get_row_hints(&self, row_idx: usize) -> &[u8] {
        let hints = self.y_hints.get(row_idx).unwrap();
        &hints[..]
    }

    fn get_column(&self, col_idx: usize) -> Vec<Cell> {
        let start = col_idx;
        let end = self.board.len();
        let step = self.x_hints.len();

        let mut cells = Vec::with_capacity(self.y_hints.len());
        for i in (start..end).step_by(step) {
            cells.push(self.board[i]);
        }
        cells
    }

    fn set_column(&mut self, col_idx: usize, cells: Vec<Cell>) {
        let start = col_idx;
        let end = self.board.len();
        let step = self.x_hints.len();

        let mut j = 0;
        for i in (start..end).step_by(step) {
            self.board[i] = cells[j];
            j += 1;
        }
    }

    fn get_column_hints(&self, col_idx: usize) -> &[u8] {
        let hints = self.x_hints.get(col_idx).unwrap();
        &hints[..]
    }

    pub fn solve(&mut self, max_iterations: u32) {
        let mut iteration = 0;
        let mut index: usize = 0;
        let mut row_mode = true;
        while !self.is_solved() && iteration < max_iterations {
            
            // Get hints for current row/column
            let hints = if row_mode {
                self.get_row_hints(index)
            } else {
                self.get_column_hints(index)
            };

            // Get current known cells for current row/column
            let cells = if row_mode {
                self.get_row(index)
            } else {
                self.get_column(index)
            };

            // Get number of rows/columns to know when to switch between rows/columns
            let max = if row_mode {
                self.y_hints.len()
            } else {
                self.x_hints.len()
            };

            // If this row/column has any unknown cells, solve the vector, then update row/column
            if cells.iter().any(|cell| matches!(cell, Cell::Unknown)) {
                let new_cells = self.solve_vector(hints, cells);

                if row_mode {
                    self.set_row(index, new_cells);
                } else {
                    self.set_column(index, new_cells);
                }
            }

            index += 1;
            if index == max {
                row_mode = !row_mode;
                index = 0;
            }
            iteration += 1;
        }
    }

    fn is_solved(&self) -> bool {
        !self.board.iter().any(|cell| matches!(cell, Cell::Unknown))
    }

    fn solve_vector(&self, hints: &[u8], cells: Vec<Cell>) -> Vec<Cell> {
        let gap_sum = cells.len() - (hints.iter().sum::<u8>() as usize);
        let (known_blocks, known_voids) = self.cell_vector_to_bitvecs(&cells);
        let mut scratch_bitvec = BitVec::with_capacity(cells.len());

        let possible_gaps = self.get_possible_gaps(gap_sum, hints.len() + 1);

        let possibilities: Vec<BitVec> = possible_gaps
            .iter()
            .map(|x| self.get_bit_vec(cells.len(), &hints, &x))
            .filter(|x| {
                let mut matches_blocks = true;

                // Filter out elements that don't match the existing known values if we have existing known values
                if let Some(block_bits) = &known_blocks {
                    scratch_bitvec.clone_from(x);

                    scratch_bitvec.and(&block_bits);
                    matches_blocks = scratch_bitvec.eq(&block_bits);
                }

                let mut matches_voids = true;
                if let Some(void_bits) = &known_voids {
                    scratch_bitvec.clone_from(x);
                    scratch_bitvec.negate();

                    scratch_bitvec.and(&void_bits);
                    matches_voids = scratch_bitvec.eq(&void_bits);
                }

                matches_blocks && matches_voids
            })
            .collect();

        // Find all elements that are always blocks
        scratch_bitvec = BitVec::from_elem(cells.len(), true);
        possibilities.iter().for_each(|e| {
            scratch_bitvec.and(e);
        });
        let new_blocks = scratch_bitvec.clone();

        // Find all elements that are never blocks
        scratch_bitvec.clear();
        possibilities.iter().for_each(|e| {
            scratch_bitvec.or(e);
        });
        scratch_bitvec.negate();

        self.bitvecs_to_cell_vector(&new_blocks, &scratch_bitvec)
    }

    fn get_possible_gaps(&self, total: usize, parts: usize) -> Vec<Vec<u8>> {
        // Pretend start and end have an imaginary gap to simplify partitioning
        // These 1s are removed when appending to the results list
        let total_plus_two = total + 2;

        let mut result: Vec<Vec<u8>> = Vec::new();

        let mut stack: Vec<Vec<u8>> = Vec::new();
        stack.push(Vec::new());

        while stack.len() > 0 {
            let cur = stack.pop().unwrap();
            let parts_left = parts - cur.len();
            let sum = cur.iter().sum::<u8>() as usize;

            if parts_left == 1 {
                let mut one_result: Vec<u8> = Vec::with_capacity(parts);
                for i in cur.iter() {
                    one_result.push(*i);
                }
                let remainder = total_plus_two - sum;
                one_result.push(remainder as u8);

                // Remove one from first and last partition to remove imaginary gap
                if let Some(first_element) = one_result.get_mut(0) {
                    *first_element -= 1;
                }
                let last_index = one_result.len() - 1;
                if let Some(last_element) = one_result.get_mut(last_index) {
                    *last_element -= 1;
                }

                result.push(one_result);
            } else {
                let max: usize = total_plus_two - sum - parts_left + 2;
                for i in 1..max {
                    let mut stack_value: Vec<u8> = Vec::with_capacity(cur.len() + 1);

                    // copy cur to stack_value
                    for j in cur.iter() {
                        stack_value.push(*j);
                    }
                    stack_value.push(i as u8);

                    stack.push(stack_value);
                }
            }
        }

        result
    }

    fn get_bit_vec(&self, size: usize, hints: &[u8], gaps: &[u8]) -> BitVec {
        let mut result = BitVec::from_elem(size, false);

        let mut idx: usize = gaps[0] as usize;
        for i in 0..hints.len() {
            for _ in 0..hints[i] {
                result.set(idx, true);
                idx += 1;
            }
            idx += gaps[i + 1] as usize;
        }

        result
    }

    fn cell_vector_to_bitvecs(&self, cells: &Vec<Cell>) -> (Option<BitVec>, Option<BitVec>) {
        let mut block_bits = BitVec::from_elem(cells.len(), false);
        let mut void_bits = BitVec::from_elem(cells.len(), false);

        for i in 0..cells.len() {
            if let Some(cell) = cells.get(i) {
                match cell {
                    Cell::Block => block_bits.set(i, true),
                    Cell::Void => void_bits.set(i, true),
                    _ => (),
                }
            }
        }

        let known_blocks = if block_bits.any() {
            Some(block_bits)
        } else {
            None
        };
        let known_voids = if void_bits.any() {
            Some(void_bits)
        } else {
            None
        };

        (known_blocks, known_voids)
    }

    fn bitvecs_to_cell_vector(&self, new_blocks: &BitVec, new_volds: &BitVec) -> Vec<Cell> {
        let mut cells = vec![Cell::Unknown; new_blocks.len()];

        for i in 0..new_blocks.len() {
            if let Some(block) = new_blocks.get(i) {
                if block {
                    cells[i] = Cell::Block;
                }
            }
            if let Some(void) = new_volds.get(i) {
                if void {
                    cells[i] = Cell::Void;
                }
            }
        }

        cells
    }
}
