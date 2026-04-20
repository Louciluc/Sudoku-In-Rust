
pub mod wrongness;
use wrongness::Wrongness;

use std::fmt;
pub fn cell_fn(){
    println!("Hi from cell.rs");
}

#[derive(Clone)]
pub struct Cell {
    pub value : Option<usize>,
    pub options_left : Vec<usize>,
    pub is_def_right : bool,
    pub wrongn: Wrongness,
    //pub position : (usize, usize),
}

impl Cell {
    pub fn new(value: Option<usize>,is_def_right: bool) -> Self {
        return Cell{
            value : value,
            is_def_right : is_def_right,
            options_left : Vec::new(),
            wrongn: Wrongness::Correct,
        };
    }

    pub fn new_empty() -> Self {
        return Cell{
            value: None,
            is_def_right: false,
            options_left : Vec::new(),
            wrongn: Wrongness::Correct,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Some(x) => return write!(f, "{}", x),
            None => return write!(f, "."),
        }
    }
}
