mod grid;
use crate::grid::*;

use std::time::Instant;

pub mod cell;
use crate::cell::*;

// ! Make sure to change type declarations also in lib.rs
pub type CellGrid = Vec<Vec<Cell>>;
pub type Mask = u64;
// with a mask of 64 bit, the max number is 64, which is storable in 7 bits, so the type must not
// contain more than 8 bits:
pub type ValType = u8;

#[allow(unused)]
pub fn main() {
    //println!("hello world");

    let mut grid = VERY_HARD_SDK();
    let full_mask = grid.full_mask();
    //println!("full_mask: {full_mask:b}");
    //grid.print_needs_new_find();

    grid.print_grid();
    let timer = Instant::now();
    grid.solve(true);
    println!("Time: {} milliseconds", timer.elapsed().as_millis());
    grid.print_all_solutions();
}
//#[allow(nonstandard_style)]
//fn HARD_SDK() -> Grid {
//    return Grid::new_from_u8_grid_quadratic_box(&vec![
//        vec![None,   Some(6),Some(1),None,   None,   Some(7),None,   None,   Some(3)],
//        vec![None,   Some(9),Some(2),None,   None,   Some(3),None,   None,   None],
//        vec![None,   None,   None,   None,   None,   None,   None,   None,   None],
//
//        vec![None,   None,   Some(8),Some(5),Some(3),None,   None,   None,   None],
//        vec![None,   None,   None,   None,   None,   None,   Some(5),None,   Some(4)],
//        vec![Some(5),None,   None,   None,   None,   Some(8),None,   None,   None],
//
//        vec![None,   Some(4),None,   None,   None,   None,   None,   None,   Some(1)],
//        vec![None,   None,   None,   Some(1),Some(6),None,   Some(8),None,   None],
//        vec![Some(6),None,   None,   None,   None,   None,   None,   None,   None],
//    ]);
//}
#[allow(nonstandard_style)]
fn VERY_HARD_SDK() -> Grid {
    return Grid::new_from_u8_grid_quadratic_box(&vec![
        vec![Some(6),None,  None,   None,   None,   None,   None,   Some(9),Some(2)],
        vec![None,  None,   None,   Some(8),None,   Some(1),None,   None,   None],
        vec![None,  None,   None,   None,   None,   None,   None,   None,   None],

        vec![None,  None,   None,   Some(7),Some(2),None,   None,   Some(4),None],
        vec![Some(1),Some(8),None,  None,   None,   None,   Some(5),None,   None],
        vec![None,  None,   None,   None,   None,   None,   None,   None,   None],

        vec![Some(2),Some(9),None,  None,   Some(6),None,   None,   None,   None],
        vec![None,  None,   None,   Some(3),None,   None,   Some(8),None,   None],
        vec![None,  None,   None,   None,   None,   None,   None,   None,   Some(7)]
    ])
}
