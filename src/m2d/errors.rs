#[derive(Debug)]
pub enum ErrorCodes {
    DifferentShape(Vec<usize>, Vec<usize>),
}
