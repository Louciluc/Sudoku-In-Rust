
mod grid;
use crate::grid::*;

pub mod cell;
use crate::cell::*;

pub type CellGrid = Vec<Vec<Cell>>;

pub fn main () {
    println!("hello world");

    let mut grid = HARD_SDK();
    grid.solve(true);
    grid.print_all_solutions();
}
fn HARD_SDK() -> Grid {
    return Grid::new_from_usize_grid_quadratic_box(&vec![
            vec![None,   Some(6),Some(1),None,  None,   Some(7),None,   None,   Some(3)],
            vec![None,   Some(9),Some(2),None,  None,   Some(3),None,   None,   None],
            vec![None,   None,   None,   None,  None,   None,   None,   None,   None],

            vec![None,   None,  Some(8),Some(5),Some(3),None,   None,   None,   None],
            vec![None,   None,  None,   None,   None,   None,   Some(5),None,   Some(4)],
            vec![Some(5),None,  None,   None,   None,   Some(8),None,  None,   None],

            vec![None,   Some(4),None,   None,   None,   None,   None,   None,   Some(1)],
            vec![None,   None,   None,   Some(1),Some(6),None,   Some(8),None,   None],
            vec![Some(6),None,   None,   None,   None,   None,   None,   None,   None],
    ]);
}

