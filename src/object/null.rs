#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Null;

impl Null {
    pub const fn new() -> Self {
        Self {}
    }
}
