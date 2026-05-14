#[derive(Debug)]
pub enum MeasurementErrors {
    DifferentShape(Vec<usize>, Vec<usize>),
}
