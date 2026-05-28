use color_print::{cformat, cprint};

use super::*;

use std::io::{stdout};
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

        let mut num_buffer = String::new();
        let mut input_msg = String::from(&cformat!("<g>The cursor alignment only works if you can see the sudoku above correctly! If you dont see a sudoku above try zooming out!</g>"));

        loop {
            // x and y are intentionally swapped
            input_msg.push_str(&cformat!("\n\rInput buffer: <c>{}</>", &num_buffer));
            self.render_grid_ui((position.1, position.0), &input_msg);
            input_msg = String::new();

            if let Event::Key(event) = read().unwrap() {
                match event.code {
                    // Exit and save
                    KeyCode::Enter | KeyCode::Char('q') => {
                        if num_buffer.is_empty() { break; }
                        else {
                            if self.set_number_maybe(position, num_buffer.parse::<ValType>().unwrap()) {
                                // setting the number worked!
                            }
                            else {
                                input_msg = String::from(&cformat!("<r>Input number out of range!</r> Clearing input buffer!"));
                            }
                            num_buffer = String::new();
                        }
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
                        if position.1 + 1 < self.total_size() { position.1 += 1; }
                        else { position.1 = 0; }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if position.0 > 0 { position.0 -= 1; }
                        else { position.0 = self.total_size() - 1; }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if position.0 + 1 < self.total_size() {position.0 += 1; }
                        else { position.0 = 0; }
                    }

                    // Numbers
                    KeyCode::Char(c @ '0'..='9') => {
                        num_buffer.push(c);
                        if num_buffer.len() == self.total_size().to_string().len() {
                            // max digits reached, set this number
                            if self.set_number_maybe(position, num_buffer.parse::<ValType>().unwrap()) {
                                // setting the number worked!
                            }
                            else {
                                input_msg = String::from(&cformat!("<r>Input number out of range!</r> Clearing input buffer!"));
                            }
                            num_buffer = String::new();
                        }
                        //
                    }

                    KeyCode::Backspace => {
                        num_buffer.pop();
                    }
                    // Delete 
                    KeyCode::Delete 
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
    fn set_number_maybe(&mut self, pos: (usize, usize), num: ValType) -> bool {
        if num < 1 || num > self.total_size().try_into().unwrap() { return false; }
        self.grid[pos.0][pos.1].value = Some(num);
        self.grid[pos.0][pos.1].is_def_right = true;
        return true;
    }
    fn render_grid_ui(&self, pos:(usize, usize), msg: &String) {
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

        print!("{}\n\r", &msg);

        cprint!("- Use arrow keys (or hjkl) to move around\n\r- Use numbers to enter a number\n\r - <u>the input buffer will print its contents automatically when max digits are entered</u> (its {} digits in this sudoku)\n\r - press Enter to set the number from the buffer\n\r - use backspace to delete last digit of buffer\n\r- \' \', \'.\', \'d\', \'x\' to delete a cell\n\r- enter (only with empty buffer) or \'q\' to save and exit\n\r- Use Esc or  \'c\' to cancel\n\r", self.total_size().to_string().len());

        let cursorpos = self.get_cursor_pos(pos, 1u16);
        execute!(
            stdout,
            MoveTo(cursorpos.0, cursorpos.1),
        ).unwrap();
    }
    // Its u16, because MoveTo() function takes u16
    fn get_cursor_pos(&self, pos_in_grid: (usize, usize), space_above: u16) -> (u16, u16) {
        let cell_w = self.total_size().to_string().len();
        // every cell has a width of cell_w.
        // every cell has a space of width 1 (*1 is reduced).
        // + cell_w to get to the end of the input field (cell_w-1 to not go to the whitespace after)
        let x = pos_in_grid.0 * cell_w + pos_in_grid.0 + cell_w - 1;

        // box_size.1 == box_count.0 <- true
        // the box where the position is in:
        let box_y: u16 = (pos_in_grid.1 / self.box_size.0).try_into().unwrap();
        // the position inside the box:
        let rem_y: u16 = <usize as TryInto<u16>>::try_into(pos_in_grid.1 % self.box_size.0).unwrap();
        print!("pos: {:?}", pos_in_grid);
        print!("box in y dir: {}, pos inside box: {}", box_y, rem_y);
        print!("box_size x: {}", self.box_size.0);
        // I have no idea why its box_size.0 but it works
        // box_y for spacing
        return (x.try_into().unwrap(), box_y * <usize as TryInto<u16>>::try_into(self.box_size.0).unwrap() + box_y + rem_y + space_above);
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
            // Check, if its a row which needs spacing:
            if (x + 1) % g.box_size.0 == 0 && x + 1 != g.total_size() {
                // Were counting in boxes here:
                // To make the crosspoint between colums and rows
                for box_of_pos in 0..g.box_count().1 {
                    for pos_in_box in 0..g.box_size.1 {
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
            let mut tmp = Grid::new_from_raw_grid_quadratic_box(&tmp_vec);
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
