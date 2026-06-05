use super::*;

pub struct SaveTxtCmd {
    // settings are in this format to save memory:  .   .   .   5   4   3   2   1
    // the bit representation:                      .   .   .   1   1   1   1   1
    // 1:   overwrite existing file, when it already exists [default: false]
    // 2:   print help in front [default: true]
    // 3:   
    settings: u64,
    path: String,
    filename: String,
}

impl SaveTxtCmd {
    // | means 8th bit:         64th|.......|.......|.......|.......|.......|.......|.......|...4321
    const DEFAULT_SETTINGS: u64 = 0x0000000000000000000000000000000000000000000000000000000000000010;
    pub const NAME: &str = "save-txt";
}

impl Cmd for SaveTxtCmd {
    fn parse(args: &Vec<String>) -> Self {
        return SaveTxtCmd {
            settings: Self::DEFAULT_SETTINGS,
            path: String::new(),
            filename: String::new(),
        };
    }
    fn execute(&self, g: &mut Grid) {
        //
    }
    fn help_string() -> String {
        return String::from("TODO: Helper function");
    }
}
