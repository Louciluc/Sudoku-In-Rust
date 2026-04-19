
use std::fmt;

pub fn cell_fn(){
    println!("Hi from cell.rs");
}

#[derive(Clone)]
pub struct Cell {
    pub value : Option<usize>,
    pub options_left : Vec<usize>,
    pub is_def_right : bool,
    pub position : (usize, usize),
}

impl Cell {
    pub fn new(value: Option<usize>, pos: (usize, usize), is_def_right: bool) -> Self {
        return Cell{
            value : value,
            position : pos,
            is_def_right : is_def_right,
            options_left : Vec::new(),
        };
    }

    pub fn new_empty() -> Self {
        return Cell{
            value: None,
            position: (0,0),
            is_def_right: false,
            options_left : Vec::new(),
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
