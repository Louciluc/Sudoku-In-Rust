use color_print::{cformat, cprint};

use super::*;

use std::io::{stdout, Write};
use crossterm::{
    cursor::MoveTo,
    execute,
    event::{read, Event, KeyCode},
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
};

impl Grid {
    pub fn edit_sudoku(&mut self) {
        let _ = enable_raw_mode().unwrap();

        let mut position = (0usize, 0usize);
        let backup_grid = self.clone();

        loop {
            // x and y are intentionally swapped
            self.render_grid_ui((position.1, position.0));

            if let Event::Key(event) = read().unwrap() {
                match event.code {
                    // Exit and save
                    KeyCode::Enter | KeyCode::Char('q') => {
                        break;
                    }
                    // cancel
                    KeyCode::Char('c') | KeyCode::Esc => {
                        *self = backup_grid;
                        break;
                    }
                    // Movement
                    KeyCode::Left | KeyCode::Char('h') => {
                        if position.1 > 0 { position.1 -= 1; }
                        else { position.1 = self.total_size() - 1; }
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        if position.1 + 1 < self.total_size() - 1 { position.1 += 1; }
                        else { position.1 = 0; }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if position.0 > 0 { position.0 -= 1; }
                        else { position.0 = self.total_size() - 1; }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if position.0 + 1 < self.total_size() - 1 {position.0 += 1; }
                        else { position.0 = 0; }
                    }

                    // Numbers
                    //KeyCode::Char(c @ '1'..=)
                    //
                    // Delete 
                    KeyCode::Backspace 
                    | KeyCode::Delete 
                    | KeyCode::Char('x')
                    | KeyCode::Char('d')
                    | KeyCode::Char('.')
                    | KeyCode::Char(' ') => {
                        self.grid[position.0][position.1].value = None;
                        self.grid[position.0][position.1].is_def_right = false;
                        self.make_all_needing_new_find(position);
                    }

                    KeyCode::Tab => {}
                    KeyCode::Home => {}
                    KeyCode::PageUp => {}
                    KeyCode::PageDown => {}
                    KeyCode::End => { position = (self.total_size() - 1, self.total_size() - 1); }
                    KeyCode::BackTab => {}
                    KeyCode::Insert => {}
                    KeyCode::Null => {}
                    KeyCode::F(_u8) => {}
                    KeyCode::CapsLock => {}
                    KeyCode::ScrollLock => {}
                    KeyCode::NumLock => {}
                    KeyCode::PrintScreen => {}
                    KeyCode::Pause => {}
                    KeyCode::Menu => {}
                    KeyCode::KeypadBegin => {}
                    _ => {}
                }
            }
        }
        execute!(stdout(), Clear(ClearType::All), MoveTo(0,0)).unwrap();
        let _ = disable_raw_mode();
    }
    fn render_grid_ui(&self, pos:(usize, usize)) {
        let mut stdout = stdout();

        execute!(
            stdout,
            MoveTo(0,0),
            Clear(ClearType::All)
        ).unwrap();

        print!(
            "Your current position is {},{}\n\r",
            pos.0 + 1,
            pos.1 + 1
        );

        self.print_grid();

        print!("- Use arrow keys to move around\n\r- Use numbers to enter a number\n\r- \' \', \'.\', \'d\', \'x\' to delete a cell\n\r- enter or \'q\' to save and exit\n\r- c to cancel\n\r");

        let cursorpos = self.get_cursor_pos(pos, 1u16);
        execute!(
            stdout,
            MoveTo(cursorpos.0, cursorpos.1),
        ).unwrap();
    }
    // Its u16, because MoveTo() function takes u16
    fn get_cursor_pos(&self, pos_in_grid: (usize, usize), space_above: u16) -> (u16, u16) {
        let x = pos_in_grid.0 * 2;
        let box_y: u16 = (pos_in_grid.1 / self.box_size.0).try_into().unwrap();
        let rem_y: u16 = <usize as TryInto<u16>>::try_into((pos_in_grid.1 % self.box_size.0)).unwrap() + space_above;
        return (x.try_into().unwrap(), box_y * <usize as TryInto<u16>>::try_into((self.box_size.1+1)).unwrap() + rem_y);
    }
    pub fn print_grid(&self) {
        let max_str_size = self.total_size().to_string().len();

        let print_horiz = |x: usize, y: usize, g: &Grid| {
            // Make the string:
            let s = match g.grid[x][y].value {
                Some(val) => val.to_string(),
                None => ".".repeat(max_str_size),
            };
            let spacing = " ".repeat(max_str_size - s.len());

            let mut output = spacing + &s;

            // format:
            //
            match g.grid[x][y].value {
                Some(_) => {
                    // when def_right -> cyan fg
                    if g.grid[x][y].is_def_right {
                        output = cformat!("<c>{}</c>", output);
                    }

                    // when wrong or responsible -> orange, yellow bg
                    match g.grid[x][y].wrongn {
                        Wrongness::Wrong => output = cformat!("<bg:#BF6900>{}</>", output),
                        Wrongness::Responsible => output = cformat!("<bg:#A38802>{}</>", output),
                        _ => {}
                    }
                }
                None => output = cformat!("<#787878>{}</>", output),
            };

            // Print
            cprint!("{}", output);

            // Print spacing:
            //
            // | when box ends ' ' in any other case
            if (y + 1) % g.box_count().0 == 0 && y + 1 != g.total_size() {
                print!("|");
            } else {
                print!(" ");
            }
        };
        // That was for each row
        //
        // Now the spacing for the boxes between certain rows
        let print_horiz_spacing = |x: usize, g: &Grid| {
            print!("\r\n");
            if (x + 1) % g.box_count().1 == 0 && x + 1 != g.total_size() {
                // Were counting in boxes here:
                // To make the crosspoint between colums and rows
                for box_of_pos in 0..g.box_count().1 {
                    for pos_in_box in 0..g.box_size.0 {
                        // print '-' where anumber would be
                        for _ in 0..g.total_size().to_string().len() {
                            print!("-");
                        }

                        // print '+' at crosspoints (end of box)
                        if (pos_in_box + 1) % g.box_size.1 == 0
                            && box_of_pos * g.box_size.1 + pos_in_box + 1 != g.total_size()
                        {
                            print!("+");
                        } else {
                            print!(" ");
                        }
                    }
                }
                print!("\r\n");
            }
        };

        // execute the lambdas:
        Grid::loop_all(self, &print_horiz, &print_horiz_spacing);
    }

    pub fn print_all_solutions(&self) {
        println!("\n{} solution(s) found.", self.solutions.len());
        for (i, sol) in self.solutions.iter().enumerate() {
            // copy all elments of one solution in a grid of Option<Mask>:
            let mut tmp_vec = vec![vec![None; sol.len()]; sol.len()];
            for x in 0..sol.len() {
                for y in 0..sol[0].len() {
                    tmp_vec[x][y] = Some(sol[x][y]);
                }
            }

            println!("\n\rSolution: {}", i + 1);

            // print that grid of Option<u8> as a sudoku Grid
            let mut tmp = Grid::new_from_u8_grid_quadratic_box(&tmp_vec);
            for x in 0..sol.len() {
                for y in 0..sol[0].len() {
                    tmp.grid[x][y].is_def_right = self.grid[x][y].is_def_right;
                }
            }

            tmp.print_grid();
        }
    }

    #[allow(dead_code)]
    pub fn print_needs_new_find(&self) {
for x in 0..self.total_size() {
            let row = self.needs_new_find[x];
            let stri = format!("{row:b}");

            println!("{}{}", "0".repeat(self.total_size()-stri.len()), stri);
        }
    }
}
