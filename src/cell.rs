pub mod wrongness;

pub use super::*;

use std::fmt;
pub fn cell_fn() {
    println!("Hi from cell.rs");
}

#[derive(Clone)]
pub struct Cell {
    pub value: Option<u8>,
    // ...00000001 -> 1 is possible in this cell
    // ...00010011 -> 1,2,5 are possible in this cell
    pub options_left: Mask,
    pub is_def_right: bool,
    pub wrongn: Wrongness,
    //pub position : (usize, usize),
}

impl Cell {
    pub fn new(value: Option<ValType>, is_def_right: bool) -> Self {
        return Cell {
            value: value,
            is_def_right: is_def_right,
            options_left: Mask::MAX,
            wrongn: Wrongness::Correct,
        };
    }

    pub fn new_empty() -> Self {
        return Cell {
            value: None,
            is_def_right: false,
            options_left: Mask::MAX,
            wrongn: Wrongness::Correct,
        };
    }

    pub fn value_as_bits(&self) -> u64 {
        match self.value {
            Some(n) => return 1 << (n - 1),
            None => return 0b0,
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
