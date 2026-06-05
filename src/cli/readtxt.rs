use super::*;

pub struct ReadTxtCmd {
    // settings are in this format to save memory:  .   .   .   5   4   3   2   1
    // the bit representation:                      .   .   .   1   1   1   1   1
    // 1:   
    // 2:   
    // 3:   
    settings: u64,
    path: String,
}

impl ReadTxtCmd {
    // | means 8th bit:         64th|.......|.......|.......|.......|.......|.......|.......|...4321
    const DEFAULT_SETTINGS: u64 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    pub const NAME: &str = "read-txt";
}

impl Cmd for ReadTxtCmd {
    fn parse(args: &Vec<String>) -> Self {
        return ReadTxtCmd {
            settings: Self::DEFAULT_SETTINGS,
            path: String::new(),
        }
    }
    fn execute(&self, g: &mut Grid) {
        //
    }
    fn help_string() -> String {
        return String::from("TODO: Helper function");
    }
}
