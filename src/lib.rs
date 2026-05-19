// This is the main file of sudoku containing the implementation of the sudoku and its solving
// algorithm.
// cell.rs contains the cells used in this sudoku.

//#![feature(test)]
//extern crate test;

pub mod cell;
pub use crate::cell::*;

pub mod grid;
pub use crate::grid::*;

pub type CellGrid = Vec<Vec<Cell>>;


#[cfg(test)]
mod tests {
    use super::*;

    fn hard_sdk_sdk() -> Grid {
        return Grid::new_from_usize_grid_quadratic_box(&vec![
            vec![None,   Some(6),Some(1),None,  None,   Some(7),None,   None,   Some(3)],
            vec![None,   Some(9),Some(2),None,  None,   Some(3),None,   None,   None],
            vec![None,   None,   None,   None,  None,   None,   None,   None,   None],

            vec![None,   None,  Some(8),Some(5),Some(3),None,   None,   None,   None],
            vec![None,   None,  None,   None,   None,   None,   Some(5),None,   Some(4)],
            vec![Some(5),None,  None,   None,   None,   Some(8),None,  None,   None],

            vec![None,   Some(4),None,   None,   None,   None,   None,   None,   Some(1)],
            vec![None,   None,   None,   Some(1),Some(6),None,   Some(8),None,   None],
            vec![Some(6),None,  None,   None,   None,   None,   None,   None,   None],
        ]);
    }

    fn mid_sdk_sdk() -> Grid {
        return Grid::new_from_usize_grid_quadratic_box(&vec![
			vec![None,    Some(8),Some(6),Some(9),None,   None,   None,   None,   None],
			vec![Some(5), Some(9),None,   Some(6),Some(2),None,   None,   None,   None],
			vec![Some(7), None,   None,   None,   None,   Some(1),None,   None,   None],
			vec![Some(2), Some(5),None,   None,   None,   None,   Some(7),None,   None],
			vec![None,    Some(4),None,   None,   None,   None,   None,   Some(3),None],
			vec![None,    None,   Some(3),None,   None,   None,   None,   Some(9),Some(2)],
			vec![None,    None,   None,   Some(3),None,   None,   None,   None,   Some(1)],
			vec![None,    None,   None,   None,   Some(5),Some(8),None,   Some(6),Some(9)],
			vec![None,    None,   None,   None,   None,   Some(2),Some(5),Some(8),None]]); 
		}


    //#[bench]
    //fn hard_sdk(b: &mut Bencher) {
    //    b.iter(|| {
    //    let mut sdk = tests::hard_sdk_sdk();
    //    sdk.print_grid();

    //    println!("totalsize: {}", sdk.total_size());
    //    println!("one to last: {:?}", sdk.one_to_last_values());

    //    sdk.solve(true);

    //    sdk.print_all_solutions();

    //    assert_eq!(sdk.solutions[0], vec![
    //        vec![4, 6, 1,9, 8, 7,2, 5, 3], 
    //        vec![7, 9, 2,4, 5, 3,1, 6, 8], 
    //        vec![3, 8, 5,2, 1, 6,4, 7, 9], 
    //        vec![1, 2, 8,5, 3, 4,7, 9, 6], 
    //        vec![9, 3, 6,7, 2, 1,5, 8, 4], 
    //        vec![5, 7, 4,6, 9, 8,3, 1, 2], 
    //        vec![8, 4, 9,3, 7, 5,6, 2, 1], 
    //        vec![2, 5, 3,1, 6, 9,8, 4, 7], 
    //        vec![6, 1, 7,8, 4, 2,9, 3, 5],]); 
    //    });
    //}

    #[test]
    fn mid_sdk() {
        let mut sdk = tests::mid_sdk_sdk();
        //sdk.print_grid();
        sdk.solve(true);

        //sdk.print_all_solutions();

        assert_eq!(sdk.solutions[0], vec![
            vec![4, 8, 6, 9, 3, 5, 1, 2, 7],
            vec![5, 9, 1, 6, 2, 7, 3, 4, 8,],
            vec![7, 3, 2, 4, 8, 1, 9, 5, 6,], 
            vec![2, 5, 9, 8, 6, 3, 7, 1, 4,], 
            vec![1, 4, 8, 2, 7, 9, 6, 3, 5,], 
            vec![6, 7, 3, 5, 1, 4, 8, 9, 2,], 
            vec![8, 2, 5, 3, 9, 6, 4, 7, 1,], 
            vec![3, 1, 4, 7, 5, 8, 2, 6, 9,], 
            vec![9, 6, 7, 1, 4, 2, 5, 8, 3,],
        ]); 
    }

    #[test]
    fn hard_sdk () {
        let mut sdk = tests::hard_sdk_sdk();

        sdk.solve(true);

        println!("hard sudoku done!");
    }


}
