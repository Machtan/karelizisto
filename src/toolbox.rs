
pub enum Tool {
    Paint
}

pub struct Toolbox {
    unused: bool
}

impl Toolbox {
    pub fn new() -> Toolbox {
        Toolbox {
            unused: true,
        }
    }
}