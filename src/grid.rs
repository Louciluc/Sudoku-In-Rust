pub mod terminal;
pub mod file;

pub use super::*;
pub use crate::wrongness::Wrongness;

#[derive(Clone)]
pub struct Grid {
    pub grid: CellGrid,
    pub needs_new_find: Vec<u64>,

    pub box_size: (usize, usize),

    // a list (Vec) of grids
    pub solutions: Vec<Vec<Vec<ValType>>>,
}

impl Grid {
    pub fn total_size(&self) -> usize {
        self.box_size.0 * self.box_size.1
    }
    pub fn box_count(&self) -> (usize, usize) {
        (self.box_size.1, self.box_size.0)
    }
    #[allow(dead_code)]
    pub fn one_to_last_values(&self) -> Vec<Mask> {
        (1..=(self.total_size() as u64)).collect()
    }
    #[allow(dead_code)]
    pub fn full_mask(&self) -> Mask {
        Self::static_full_mask(self.total_size())
    }
    fn static_full_mask(total_size: usize) -> Mask {
        // shift overflow safe! e.g. 9:
        // 1u64 << total_size      1000000000 <- doesnt work, since on size of 64 it would go to 65th bit
        // 1u64 << total_size -1    100000000
        // - 1                      011111111
        // << 1                     111111110
        // + 1                      111111111
        (((1u64 << total_size -1) - 1) << 1) + 1
    }

    // Constructors if box size is quadratic:
    #[allow(dead_code)]
    pub fn new_from_grid_quadratic_box(g: CellGrid) -> Grid {
        if !Grid::check_correct_size(&g) {
            panic!("Input grid (CellGrid) hadnt had a quadratic size");
        }

        let size = g.len();
        let s = size.isqrt();
        let mut grid = Grid {
            grid: g,
            needs_new_find: vec![Self::static_full_mask(size); size],
            box_size: (s, s),
            solutions: Vec::new(),
        };
        grid.find_all_remains_in_all_cells();
        return grid;
    }
    // Constructor:
    #[allow(dead_code)]
    pub fn new_empty_quadratic_box(box_size: usize) -> Grid {
        let size = box_size * box_size;
        return Grid {
            grid: vec![vec![Cell::new_empty(); size]; size],
            needs_new_find: vec![Self::static_full_mask(size); size],
            box_size: (box_size, box_size),
            solutions: Vec::new(),
        };
    }
    // Constructor:
    #[allow(dead_code)]
    pub fn new_from_raw_grid_quadratic_box(u_g: &Vec<Vec<Option<ValType>>>) -> Grid {
        let mut res = Vec::new();

        for x in 0..u_g.len() {
            res.push(Vec::new());
            for y in 0..u_g[0].len() {
                res[x].push(Cell::new(u_g[x][y], u_g[x][y].is_some()));
            }
        }

        return Self::new_from_grid_quadratic_box(res);
    }

    // Constructors if box size is not quadratic:
    #[allow(dead_code)]
    pub fn new_from_grid_rectangle_box(g: CellGrid, box_wideness: usize) -> Grid {
        let size = g.len();
        if size < box_wideness {
            panic!("The input box is wider than the whole sudoku");
        }
        if !Grid::check_correct_size(&g) {
            panic!("The input sudoku has different sizes in x and y direction.!");
        }

        let height = size / box_wideness;
        if height * box_wideness != size {
            panic!("The input sudoku cannot have this box_wideness. Its not possible to split it in boxes with this wideness.")
        }
        let mut grid = Grid {
            grid: g,
            needs_new_find: vec![Self::static_full_mask(size); size],
            box_size: (box_wideness, height),
            solutions: Vec::new(),
        };
        grid.find_all_remains_in_all_cells();
        return grid;
    }
    // Constructor:
    #[allow(dead_code)]
    pub fn new_empty_rectangle_box(box_wideness: usize, box_height: usize) -> Grid {
        let size = box_wideness * box_height;
        return Grid {
            grid: vec![vec![Cell::new_empty(); size]; size],
            needs_new_find: vec![Self::static_full_mask(size); size],
            // height and width are swapped intentionally since in [][] notations x has to be first
            box_size: (box_height, box_wideness),
            solutions: Vec::new(),
        };
    }
    // Constructor:
    #[allow(dead_code)]
    pub fn new_from_raw_grid_rectangle_box( u_g: &Vec<Vec<Option<ValType>>>, box_wideness: usize,) -> Grid {
        let mut res = Vec::new();

        for x in 0..u_g.len() {
            res.push(Vec::new());
            for y in 0..u_g[0].len() {
                res[x].push(Cell::new(u_g[x][y], u_g[x][y].is_some()));
            }
        }

        return Self::new_from_grid_rectangle_box(res, box_wideness);
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

    fn find_options_for_cell(&self, cell_pos: (usize, usize)) -> Mask {
        let mut used_nums: Mask = 0b0;

        // horizontal x=const
        for y in 0..self.total_size() {
            used_nums |= self.grid[cell_pos.0][y].value_as_bits();
        }

        // vertical y=const
        for x in 0..self.total_size() {
            used_nums |= self.grid[x][cell_pos.1].value_as_bits();
        }

        // Inside Box:
        let mut box_func = |x:usize, y:usize, box_of_pos: (usize, usize), g: &Grid| {
            let global_pos = g.box_pos_to_abs_pos((x, y), box_of_pos);
            used_nums |= g.grid[global_pos.0][global_pos.1].value_as_bits();
        };
        Self::loop_all_in_box_of_pos(self, cell_pos, &mut box_func);
        //println!("Time elapsed for box: {}", t_measure_b.elapsed().as_nanos());
        //println!(
        //    "Time elapsed for getting options: {}",
        //    t_measure.elapsed().as_nanos()
        //);

        let rem_nums = !used_nums & self.full_mask();
        //println!("Pos: {:?} remaining as bits: {rem_nums:b}", cell_pos);
        return rem_nums;
    }

    fn find_all_remains_in_all_cells(&mut self) {
        let do_for_all = |x: usize, y: usize, g: &mut Grid| {
            match g.grid[x][y].value {
                Some(_) => {}
                None => {
                    let pos_mask = g.needs_new_find[x] & g.usize_to_mask(y);
                    if pos_mask != 0 {
                        g.grid[x][y].options_left = g.find_options_for_cell((x, y));
                        g.needs_new_find[x] ^= pos_mask;
                    }
                }
            };
        };

        Grid::loop_all_mut(self, &do_for_all, &|_, _| {});
    }

    fn recursive_solve(&mut self, calc_all_sol: bool) -> bool {
        //let time_measure = Instant::now();
        self.find_all_remains_in_all_cells();

        let mut empty_cells = Vec::new();
        Grid::loop_all_mut_mut(
            self,
            &mut |x, y, grid: &mut Grid| match grid.grid[x][y].value {
                Some(_) => {}
                None => empty_cells.push((x, y)),
            },
            &mut |_, _| {},
        );

        // if there are no empty cells, the sudoku is solved
        if empty_cells.len() == 0 {
            let mut new_sol = vec![vec![0u8; self.total_size()]; self.total_size()];
            // unwrap consumes the Option<T>, thats why its cloned before
            Grid::loop_all_mut_mut(
                self,
                &mut |x, y, grid: &mut Grid| {
                    new_sol[x][y] = grid.grid[x][y].value.clone().unwrap();
                },
                &mut |_, _| {},
            );
            self.solutions.push(new_sol);
            return true;
        }

        // if there are empty cells, sort by remaining options:
        empty_cells.sort_by(|a: &(usize, usize), b: &(usize, usize)| {
            self.grid[a.0][a.1]
                .options_left
                .count_ones()
                .cmp(&self.grid[b.0][b.1].options_left.count_ones())
        });

        // if the first cell has zero options left, backtrack one step:
        if self.grid[empty_cells[0].0][empty_cells[0].1]
            .options_left.count_ones() == 0
        {
            return false;
        }

        let mut at_least_one_workes = false;
        let old_value = self.grid[empty_cells[0].0][empty_cells[0].1].value;

        // make immutable list of nums to try, just to make sure nothing will
        // get overwritten when continuing to solve
        let mut options_to_try = self.grid[empty_cells[0].0][empty_cells[0].1].options_left;

        // loop through all options left in the cell
        while options_to_try != 0 {
            let try_num = Cell::mask_to_first_valtype(options_to_try);

            self.grid[empty_cells[0].0][empty_cells[0].1].value = Some(try_num);
            self.make_all_needing_new_find(empty_cells[0]);

            //self.print_grid();
            //self.print_needs_new_find();

            let worked = self.recursive_solve(calc_all_sol);

            if worked {
                at_least_one_workes = true;

                if !calc_all_sol {
                    return true;
                }
            }
            self.make_all_needing_new_find(empty_cells[0]);

            options_to_try &= options_to_try - 1;
        }

        // Reset value to old_value
        self.grid[empty_cells[0].0][empty_cells[0].1].value = old_value;

        return at_least_one_workes;
    }

    fn check_correct_size(g: &CellGrid) -> bool {
        g.len() == g[0].len()
    }

    //fn loop_all<FnAll, FnXes>(g: &Grid, fn_all: &FnAll, fn_xes: &FnXes)
    //where
    //    FnAll: Fn(usize, usize, &Grid),
    //    FnXes: Fn(usize, &Grid),
    //{
    //    for x in 0..g.total_size() {
    //        for y in 0..g.total_size() {
    //            fn_all(x, y, g);
    //        }
    //        fn_xes(x, g);
    //    }
    //}

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

    fn loop_all_in_box_of_pos_mut_mut<Func>(g: &mut Self, pos: (usize, usize), f: &mut Func)
    where 
        Func: FnMut(usize, usize,(usize, usize), &mut Grid),
    {
        // inside box
        let mut box_of_pos = (0, 0);
        let _pos_in_box = g.abs_pos_to_box_pos(pos, &mut box_of_pos);

        //let t_measure_b = Instant::now();
        // loop through box
        for x in 0..g.box_size.0 {
            for y in 0..g.box_size.1 {
                f(x,y,box_of_pos,g);
            }
        }
    }

    fn loop_all_in_box_of_pos<Func>(g: &Self, pos:(usize, usize), f: &mut Func) 
        where Func: FnMut (usize, usize, (usize, usize), &Grid),
    {
        // inside box
        let mut box_of_pos = (0, 0);
        let _pos_in_box = g.abs_pos_to_box_pos(pos, &mut box_of_pos);

        //let t_measure_b = Instant::now();
        // loop through box
        for x in 0..g.box_size.0 {
            for y in 0..g.box_size.1 {
                f(x,y,box_of_pos,g);
            }
        }

    }
 
    fn make_all_needing_new_find(&mut self, pos: (usize, usize)){
        // set needs_new_find respectively:
        for x in 0..self.total_size() {
            self.make_one_needing_new_find((x, pos.1));
        }
        // vertical 
        for y in 0..self.total_size() {
            self.make_one_needing_new_find((pos.0, y));
        }
        // box:
        let mut f_box = |x:usize, y:usize, box_of_pos: (usize, usize), g: &mut Grid|{
            let global_pos = g.box_pos_to_abs_pos((x,y), box_of_pos);
            g.make_one_needing_new_find(global_pos);
        };
        Self::loop_all_in_box_of_pos_mut_mut(self, pos, &mut f_box);
    }
    fn make_one_needing_new_find(&mut self, pos_to_change: (usize, usize)) {
        self.needs_new_find[pos_to_change.0] |= self.usize_to_mask(pos_to_change.1);
    }

    pub fn human_readable_grid(&self, with_carriage_return: bool, with_colors: bool) -> String {
        use color_print::cformat;
        let max_str_size = self.max_str_size();
        let mut res = String::new();

        for x in 0..self.total_size() {

            // For each row
            for y in 0..self.total_size() {
                // Make the string:
                let s = match self.grid[x][y].value {
                    Some(val) => val.to_string(),
                    None => ".".repeat(max_str_size),
                };
                let spacing = " ".repeat(max_str_size - s.len());

                let mut output = spacing + &s;

                // format:
                //
                if with_colors {
                    match self.grid[x][y].value {
                        Some(_) => {
                            // when def_right -> cyan fg
                            if self.grid[x][y].is_def_right {
                                output = cformat!("<c>{}</c>", output);
                            }

                            // when wrong or responsible -> orange, yellow bg
                            match self.grid[x][y].wrongn {
                                Wrongness::Wrong => output = cformat!("<bg:#BF6900>{}</>", output),
                                Wrongness::Responsible => output = cformat!("<bg:#A38802>{}</>", output),
                                _ => {}
                            }
                        }
                        None => output = cformat!("<#787878>{}</>", output),
                    };
                }

                // Print
                res.push_str(&cformat!("{}", output));

                // Print spacing:
                //
                // | when box ends ' ' in any other case
                if (y + 1) % self.box_count().0 == 0 && y + 1 != self.total_size() {
                    res.push('|');
                } else {
                    res.push(' ');
                }
            }

            // Now the spacing
            if with_carriage_return { res.push('\r'); }
            res.push_str("\n");
            // Check, if its a row which needs spacing:
            if (x + 1) % self.box_size.0 == 0 && x + 1 != self.total_size() {
                // make # for txt files
                // add '-' for first cell. But -1 because '#' is already there
                res.push('#');
                res.push_str("-".repeat(max_str_size -1).as_str());
                // Were counting in boxes here:
                // To make the crosspoint between colums and rows
                for box_of_pos in 0..self.box_count().1 {
                    for pos_in_box in 0..self.box_size.1 {
                        if box_of_pos == 0 && pos_in_box == 0 {
                            res.push(' ');
                            continue;
                        }
                        // print '-' where anumber would be
                        for _ in 0..self.max_str_size() {
                            res.push('-');
                        }

                        // print '+' at crosspoints (end of box)
                        if (pos_in_box + 1) % self.box_size.1 == 0
                            && box_of_pos * self.box_size.1 + pos_in_box + 1 != self.total_size()
                        {
                            res.push('+');
                        } else {
                            res.push(' ');
                        }
                    }
                }
                if with_carriage_return { res.push('\r'); }
                res.push_str("\n");
            }
        }
        return res;
    }

    fn usize_to_mask(&self, u: usize) -> Mask {
        return 1u64 << self.total_size() -u -1;
    }
    pub fn max_str_size(&self) -> usize {
        return self.total_size().to_string().len();
    }
    /*
        fn find_box_size(total_len: Mask) -> (Mask, Mask) {
            let int_root = total_len.isqrt();
            let float_root = (total_len as f64).sqrt() as Mask;

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

struct BoxSizeComparer { pub value: (Mask, Mask), }

impl Ord for BoxSizeComparer { fn cmp(&self, other: &Self) -> Ordering { return (self.value.0.abs_diff(self.value.1)).cmp(&other.value.0.abs_diff(other.value.1)); } }
impl PartialOrd for BoxSizeComparer { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { return Some(self.cmp(other)) } }
impl PartialEq for BoxSizeComparer { fn eq(&self, other: &Self) -> bool { self.value.0 == other.value.0 && self.value.1 == other.value.1 || self.value.0 == other.value.1 && self.value.1 == other.value.0 } }
impl Eq for BoxSizeComparer {}
*/
