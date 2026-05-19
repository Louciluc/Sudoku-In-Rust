use std::time::{Duration, Instant};

pub mod terminal;

pub use super::*;
pub use crate::wrongness::Wrongness;


#[derive(Clone)]
pub struct Grid {
    pub grid: CellGrid,
    pub box_size: (usize, usize),

    pub solutions: Vec<Vec<Vec<usize>>>,
}

impl Grid {
    pub fn total_size(&self) -> usize {
        self.box_size.0 * self.box_size.1
    }
    pub fn box_count(&self) -> (usize, usize) {
        (self.box_size.1, self.box_size.0)
    }
    pub fn one_to_last_values(&self) -> Vec<usize> {
        (1..=(self.box_size.0 * self.box_size.1)).collect()
    }

    // Constructors if box size is quadratic:
    pub fn new_from_grid_quadratic_box(g: &CellGrid) -> Grid {
        if !Grid::check_correct_size(g) {
            panic!("Input grid (CellGrid) hadnt had a quadratic size");
        }

        let s = g.len().isqrt();
        return Grid {
            grid: g.clone(),
            box_size: (s, s),
            solutions: Vec::new(),
        };
    }
    // Constructor:
    pub fn new_empty_quadratic_box(size: usize) -> Grid {
        let s = size.isqrt();
        return Grid {
            grid: vec![vec![Cell::new_empty(); size]; size],
            box_size: (s, s),
            solutions: Vec::new(),
        };
    }
    // Constructor:
    pub fn new_from_usize_grid_quadratic_box(u_g: &Vec<Vec<Option<usize>>>) -> Grid {
        let mut res = Vec::new();

        for x in 0..u_g.len() {
            res.push(Vec::new());
            for y in 0..u_g[0].len() {
                res[x].push(Cell::new(u_g[x][y], u_g[x][y].is_some()));
            }
        }

        return Grid::new_from_grid_quadratic_box(&res);
    }

    // Constructors if box size is not quadratic:
    pub fn new_from_grid_rectangle_box(grid: &CellGrid, wideness: usize) -> Grid {
        if grid.len() < wideness {
            panic!("The input box is to be wider than the whole sudoku");
        }
        if !Grid::check_correct_size(grid) {
            panic!("The input sudoku has different sizes in x and y direction.!");
        }

        let height = grid.len() / wideness;
        return Grid {
            grid: grid.clone(),
            box_size: (wideness, height),
            solutions: Vec::new(),
        };
    }
    // Constructor:
    pub fn new_empty_rectangle_box(total_size: usize, wideness: usize) -> Grid {
        if total_size < wideness {
            panic!("The input box is to be wider than the whole sudoku");
        }

        let height = total_size / wideness;
        return Grid {
            grid: vec![vec![Cell::new_empty(); total_size]; total_size],
            box_size: (wideness, height),
            solutions: Vec::new(),
        };
    }
    // Constructor:
    pub fn new_from_usize_grid_rectangle_box(
        u_g: &Vec<Vec<Option<usize>>>,
        wideness: usize,
    ) -> Grid {
        let mut res = Vec::new();

        for x in 0..u_g.len() {
            res.push(Vec::new());
            for y in 0..u_g[0].len() {
                res[x].push(Cell::new(u_g[x][y], u_g[x][y].is_some()));
            }
        }

        return Grid::new_from_grid_rectangle_box(&res, wideness);
    }

    // Solver:
    pub fn solve(&mut self, mult_solutions: bool) -> bool {
        return self.recursive_solve(mult_solutions);
    }

    fn box_pos_to_abs_pos(
        &self,
        pos_in_box: (usize, usize),
        box_where_pos_is_in: (usize, usize),
    ) -> (usize, usize) {
        (
            pos_in_box.0 + box_where_pos_is_in.0 * self.box_size.0,
            pos_in_box.1 + box_where_pos_is_in.1 * self.box_size.1,
        )
    }
    fn abs_pos_to_box_pos(
        &self,
        abs_pos: (usize, usize),
        out_box_where_pos_is_in: &mut (usize, usize),
    ) -> (usize, usize) {
        *out_box_where_pos_is_in =
            (abs_pos.0 / self.box_size.0, abs_pos.1 / self.box_size.1).clone();
        return (abs_pos.0 % self.box_size.0, abs_pos.1 % self.box_size.1);
    }

    /*
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
    */

    fn find_possible_options_for_cell(&self, cell_pos: (usize, usize)) -> Vec<usize> {
        let mut found_nums = Vec::new();

        let t_measure = Instant::now();
        let t_measure_h = Instant::now();
        // horizontal x=const
        for y in 0..self.total_size() {
            // skip input pos
            if y == cell_pos.1 { continue; }

            match self.grid[cell_pos.0][y].value {
                Some(val) => found_nums.push(val),
                None => {}
            }
        }
        println!("Time elapsed for horizontal: {}", t_measure_h.elapsed().as_nanos());

        let t_measure_v = Instant::now();
        // vertical y=const
        for x in 0..self.total_size() {
            // skip input pos
            if x == cell_pos.0 { continue; }

            match self.grid[x][cell_pos.1].value {
                Some(val) => found_nums.push(val),
                None => {}
            }
        }
        println!("Time elapsed for vertical: {}", t_measure_v.elapsed().as_nanos());

        // inside box
        let mut box_of_pos = (0, 0);
        let pos_in_box = self.abs_pos_to_box_pos(cell_pos, &mut box_of_pos);

        let t_measure_b = Instant::now();
        // loop through box
        for x in 0..self.box_size.0 {
            for y in 0..self.box_size.1 {
                // skip input pos
                if x == pos_in_box.0 && y == pos_in_box.1 {
                    continue;
                }

                let global_pos = self.box_pos_to_abs_pos((x, y), box_of_pos);

                match self.grid[global_pos.0][global_pos.1].value {
                    Some(val) => found_nums.push(val),
                    None => {}
                }
            }
        }
        println!("Time elapsed for box: {}", t_measure_b.elapsed().as_nanos());
        println!("Time elapsed for getting options: {}", t_measure.elapsed().as_nanos());

        let mut remaining_nums = self.one_to_last_values();

        let t_measure_sort = Instant::now();
        // keep all nums that werent found:
        // in other words: every number that occurs in both lists should be removed
        remaining_nums.retain(|n| !found_nums.contains(n));
        println!("Time elapsed for sorting: {}", t_measure_sort.elapsed().as_nanos());

        return remaining_nums;
    }

    fn find_and_set_all_cells(&mut self) {
        let do_for_all = |x: usize, y: usize, g: &mut Grid| {
            match g.grid[x][y].value {
                Some(_) => {}
                None => {
                    g.grid[x][y].options_left = g.find_possible_options_for_cell((x, y));
                    //println!("found options of ({},{}): {:?}", x,y,g.find_possible_options_for_cell((x,y)));
                }
            };
        };

        Grid::loop_all_mut(self, &do_for_all, &|_, _| {});
    }

    fn recursive_solve(&mut self, calc_all_sol: bool) -> bool {
        let time_measure = Instant::now();
        self.find_and_set_all_cells();
        println!("Time elapsed for setting all cells: {}", time_measure.elapsed().as_nanos());

        let mut empty_cells = Vec::new();
        Grid::loop_all_mut_mut(self,
            &mut |x, y, grid: &mut Grid|
            match grid.grid[x][y].value {
                Some(_) => {}
                None => empty_cells.push((x, y)),
            },
            &mut |_, _| {},
        );

        // if there are no empty cells, the sudoku is solved
        if empty_cells.len() == 0 {
            let mut new_sol = vec![vec![0usize; self.total_size()]; self.total_size()];
            // unwrap consumes the Option<T>, thats why its cloned before
            Grid::loop_all_mut_mut( self,
                &mut |x, y, grid: &mut Grid| {
                    new_sol[x][y] = grid.grid[x][y].value.clone().unwrap();
                },
                &mut |_, _| {},
            );
            self.solutions.push(new_sol);
            return true;
        }

        /*
        self.print_grid();
        for c in &empty_cells {
            println!("found at {:?}: {:?}", c, self.grid[c.0][c.1].options_left);
        }
        */

        // if there are empty cells, sort by remaining options:
        empty_cells.sort_by(|a: &(usize, usize), b: &(usize, usize)| {
            self.grid[a.0][a.1].options_left.len()
                .cmp(&self.grid[b.0][b.1].options_left.len())
        });

        // if the first cell has zero options left, backtrack one step:
        if self.grid[empty_cells[0].0][empty_cells[0].1].options_left.len()== 0 { return false; }

        let mut at_least_one_workes = false;
        let old_value = self.grid[empty_cells[0].0][empty_cells[0].1].value;

        // make immutable list of nums to try, just to make sure nothing will
        // get overwritten when continuing to solve
        let options_to_try = self.grid[empty_cells[0].0][empty_cells[0].1]
            .options_left
            .clone();

        for try_num in options_to_try {
            //println!("Trying {}", try_num);
            self.grid[empty_cells[0].0][empty_cells[0].1].value = Some(try_num);
            let worked = self.recursive_solve(calc_all_sol);
            // if its solved, the result will already be saved at this point

            if worked {
                at_least_one_workes = true;

                if !calc_all_sol {
                    return true;
                }
            }
        }

        // Reset value to old_value
        self.grid[empty_cells[0].0][empty_cells[0].1].value = old_value;

        return at_least_one_workes;
    }

    fn check_correct_size(g: &CellGrid) -> bool {
        g.len() == g[0].len()
    }

    fn loop_all<FnAll, FnXes>(g: &Grid, fn_all: &FnAll, fn_xes: &FnXes)
    where
        FnAll: Fn(usize, usize, &Grid),
        FnXes: Fn(usize, &Grid),
    {
        for x in 0..g.total_size() {
            for y in 0..g.total_size() {
                fn_all(x, y, g);
            }
            fn_xes(x, g);
        }
    }

    fn loop_all_mut<FnAll, FnXes>(g: &mut Grid, fn_all: &FnAll, fn_xes: &FnXes)
    where
        FnAll: Fn(usize, usize, &mut Grid),
        FnXes: Fn(usize, &mut Grid),
    {
        for x in 0..g.total_size() {
            for y in 0..g.total_size() {
                fn_all(x, y, g);
            }
            fn_xes(x, g);
        }
    }

    fn loop_all_mut_mut<FnAll, FnXes>(g: &mut Grid, fn_all: &mut FnAll, fn_xes: &mut FnXes)
    where
        FnAll: FnMut(usize, usize, &mut Grid),
        FnXes: FnMut(usize, &mut Grid),
    {
        for x in 0..g.total_size() {
            for y in 0..g.total_size() {
                fn_all(x, y, g);
            }
            fn_xes(x, g);
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
