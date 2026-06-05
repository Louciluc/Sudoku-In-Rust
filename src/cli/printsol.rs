use super::*;

pub struct PrintSolCmd {
    // settings are in this format to save memory:  .   .   .   5   4   3   2   1
    // the bit representation:                      .   .   .   1   1   1   1   1
    // 1:   Print all solutions [default: true]
    // 2:   
    // 3:   
    settings: u64,
    max_solutions: usize, // default: 1
}

impl PrintSolCmd {
    // | means 8th bit:         64th|.......|.......|.......|.......|.......|.......|.......|...4321
    const DEFAULT_SETTINGS: u64 = 0x0000000000000000000000000000000000000000000000000000000000000001;
    pub const NAME: &str = "print-solution";
}

impl Cmd for PrintSolCmd {
    fn parse(args: &Vec<String>) -> Self {
        return PrintSolCmd {
            settings: Self::DEFAULT_SETTINGS,
            max_solutions: 1,
        };
    }
    fn execute(&self, g: &mut Grid) {
        //
    }
    fn help_string() -> String {
        return String::from("TODO: Helper function");
    }
}
