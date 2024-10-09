#[derive(Debug, Clone)]
pub struct Calc {}

impl Calc {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, expr: &String) -> i64 {
        100
    }
}
