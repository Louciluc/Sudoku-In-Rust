

pub mod cell;
pub use crate::cell::*;

pub mod grid;
pub use crate::grid::*;

pub type CellGrid = Vec<Vec<Cell>>;



pub fn main () {
    println!("hello world");

    let mut sdk = hard_sdk();
        sdk.print_grid();

        println!("totalsize: {}", sdk.total_size());
        println!("one to last: {:?}", sdk.one_to_last_values());

        sdk.solve(true);

        sdk.print_all_solutions();

}
fn hard_sdk() -> Grid {
    return Grid::new_from_usize_grid_quadratic_box(&vec![
            vec![None, Some(6), Some(1), None, None, Some(7), None, None, Some(3)],
            vec![None, Some(9), Some(2), None, None, Some(3), None, None, None],
            vec![None, None, None, None, None, None, None, None, None],

            vec![None, None, Some(8), Some(5), Some(3), None, None, None, None],
            vec![None, None, None, None, None, None, Some(5), None, Some(4)],
            vec![Some(5), None, None, None, None, Some(8), None, None, None],

            vec![None, Some(4), None, None, None, None, None, None,Some(1)],
            vec![None, None, None,Some(1),Some(6),None,Some(8), None, None],
            vec![Some(6), None, None, None, None, None, None, None, None],
        ]);
}

