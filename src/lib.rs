
// This is the main file of sudoku containing the implementation of the sudoku and its solving
// algorithm. 
// cell.rs contains the cells used in this sudoku.


mod cell;

type CellGrid= Vec<Vec<cell::Cell>>;

#[derive(Clone)]
pub struct Grid {
    pub grid: CellGrid,
    pub box_size: (usize, usize),
}

impl Grid {
    pub fn total_size(&self) -> usize { self.box_size.0 * self.box_size.1 }
    pub fn box_count(&self) -> (usize, usize) { (self.box_size.1, self.box_size.0) }
    pub fn one_to_last_values(&self) -> Vec<usize> { (1..(self.box_size.0 * self.box_size.1)).collect() }

    // Constructors:
    pub fn new_from_grid_quadratic_box(g: &CellGrid) -> Grid {
        if !Grid::check_correct_size(g) { panic!("Input grid (CellGrid) hadnt had a quadratic size"); }

        let s = g.len().isqrt();
        return Grid{
            grid: g.clone(),
            box_size: (s,s),
        }
    }
    // Contructor:
    pub fn new_empty_quadratic_box(size: usize) -> Grid {
        let s = size.isqrt();
        let mut this = Grid { grid : vec![vec![cell::Cell::new_empty(); size];size],
            box_size : (s,s),
        };

        this.loop_all(
            |x,y,grid| 
                { grid.grid[x][y] = cell::Cell::new(None, (x,y), false); }, 
            |_,_| {},);
        return this;
    }
    // Constructor:
    pub fn new_from_grid_rectangle_box(grid: &CellGrid, wideness: usize) -> Grid {
        if grid.len() < wideness { panic!("The input box is to be wider than the whole sudoku"); }
        if !Grid::check_correct_size(grid) { panic!("The input sudoku has different sizes in x and y direction.!"); }

        let height = grid.len() / wideness;
        return Grid {
            grid: grid.clone(),
            box_size: (wideness, height),
        };
    }
    // Constructot:
    pub fn new_empty_rectangle_box(total_size: usize, wideness: usize) -> Grid {
        if total_size < wideness { panic!("The input box is to be wider than the whole sudoku"); }

        let height = total_size / wideness;
        let mut this = Grid { grid : vec![vec![cell::Cell::new_empty(); total_size]; total_size],
            box_size : (wideness,height),
        };

        this.loop_all(
            |x,y,grid| 
                { grid.grid[x][y] = cell::Cell::new(None, (x,y), false); }, 
            |_,_| {},);
        return this;
    }

    fn box_pos_to_abs_pos(&self, pos_in_box: (usize, usize), box_where_pos_is_in: (usize, usize)) -> (usize, usize) {
        (pos_in_box.0 + box_where_pos_is_in.0 * self.box_size.0, pos_in_box.1 + box_where_pos_is_in.1 * self.box_size.1)
    }
    fn abs_pos_to_box_pos(&self, abs_pos: (usize, usize), out_box_where_pos_is_in: &mut (usize, usize)) -> (usize, usize) {
        *out_box_where_pos_is_in = (abs_pos.0 / self.box_size.0, abs_pos.1 / self.box_size.1).clone();
        return (abs_pos.0 % self.box_size.0, abs_pos.1 % self.box_size.1);
    }

    fn conv_2d_vec_to_1d<T>(two_d_vec: &Vec<Vec<T>>) -> Vec<T> where T: Copy {
        let mut result = Vec::new();
        let mut index = 0;
        for x in 0..two_d_vec.len() {
            for y in 0..two_d_vec[0].len() {
                result[index] = two_d_vec[x][y];
                index += 1;
            }
        }
        return result;
    }



    fn check_correct_size(g: &CellGrid) -> bool { g.len() == g[0].len() }

    fn loop_all<FnAll, FnXes>(&mut self, fn_all: FnAll, fn_xes: FnXes) where FnAll: Fn(usize, usize, &mut Grid), FnXes: Fn(usize, &mut Grid) {
        for x in 0..self.total_size() {
            for y in 0..self.total_size() {
                fn_all(x, y,self);
            }
            fn_xes(x, self);
        }
    }

    /*
    fn find_box_size(total_len: usize) -> (usize, usize) {
        let int_root = total_len.isqrt();
        let float_root = (total_len as f64).sqrt() as usize;

        if int_root == float_root {
            // The box is quadratic
            return (int_root, int_root);
        }

        let mut possible_matches = Vec::new();
        for i in 1..total_len {
            for j in 1..total_len {
                if i * j == total_len { possible_matches.push(BoxSizeComparer {value:(i,j),}); }
            }
        }

        if possible_matches.len() == 0 {panic!("No box size found"); }
        possible_matches.sort();
        return possible_matches[0].value;

    }
*/    
}
/*
use std::cmp::Ordering;

struct BoxSizeComparer { pub value: (usize, usize), }

impl Ord for BoxSizeComparer { fn cmp(&self, other: &Self) -> Ordering { return (self.value.0.abs_diff(self.value.1)).cmp(&other.value.0.abs_diff(other.value.1)); } }
impl PartialOrd for BoxSizeComparer { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { return Some(self.cmp(other)) } }
impl PartialEq for BoxSizeComparer { fn eq(&self, other: &Self) -> bool { self.value.0 == other.value.0 && self.value.1 == other.value.1 || self.value.0 == other.value.1 && self.value.1 == other.value.0 } }
impl Eq for BoxSizeComparer {}
*/


pub fn add(left: u64, right: u64) -> u64 {
    cell::cell_fn();
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
