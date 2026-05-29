
use super::*;

use std::fs::File;
use std::io::{BufRead, BufReader};

impl Grid {
    pub fn read_txt(file: &str) -> std::io::Result<Self> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);

        let mut grid: Vec<Vec<Option<ValType>>> = vec![];
        let mut box_size: (usize, usize) = (0,0);
        let mut next_pos: (usize, usize) = (0,0);

        for line in reader.lines() {
            let line = line?;

            // skip comments '#'
            if line.starts_with('#') || line.is_empty() { continue; }

            // the line contains information
            let mut cells: Vec<_> = line.split(|c| c == ' ' || c == '|').collect();
            cells.retain(|x| *x != "");

            if grid.is_empty() {
                // grid and box_size arent set yet
                let h_boxes: Vec<_> = line.split('|').collect();
                let total_size: usize = cells.len();
                // box_size.0 == box_count.1 <- true
                // h_boxes.len() is the number of boxes in horizontal direction
                // aka the box_size in vertical direction
                box_size = (h_boxes.len(), total_size / h_boxes.len());
                // check if box size works:
                if box_size.0 * box_size.1 != total_size {
                    // Error boxes dont fit in total_size
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "box_size doesnt fit in this sudoku!",
                    ));
                }
                grid = vec![vec![None; total_size]; total_size];
            }

            for cell in cells {
                print!("pos: {:?}", next_pos);
                // This every cell below here should be a number or '.'
                if cell.contains(".") {
                    grid[next_pos.0][next_pos.1] = None;
                    next_pos.1 += 1;
                    continue;
                }
                // from here is should only be numbers
                grid[next_pos.0][next_pos.1] = Some(cell.parse().unwrap());
                next_pos.1 += 1;
            }
            next_pos.0 += 1;
            next_pos.1 = 0;
        }
        if box_size == (0,0) || grid.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Something went wrong, the grid was never initialized. Typically that means no line with data was found!"
            ));
        }
        let sdk: Self;
        if box_size.0 == box_size.1 {
            sdk = Self::new_from_raw_grid_quadratic_box(&grid);
        }
        else {
            sdk = Self::new_from_raw_grid_rectangle_box(&grid, box_size.0);
        }
        Ok(sdk)
    }
}
