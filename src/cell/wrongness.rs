#[derive(Clone)]
pub enum Wrongness {
    Correct,
    Wrong,
    Responsible,
}

impl Wrongness {
    pub fn new() -> Wrongness {
        return Wrongness::Correct;
    }
}
