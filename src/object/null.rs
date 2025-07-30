#[derive(Debug, Clone, Copy, Default)]
pub struct Null;

impl Null {
    pub const fn new() -> Self {
        Self {}
    }
}
