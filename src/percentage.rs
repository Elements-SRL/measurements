/// Represents a percentage value between 0.0 and 1.0 (inclusive).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Percentage(f64);

impl Percentage {
    /// Creates a new constant [`Percentage`] at compile time.
    ///
    /// # Panics
    /// Panics if the value is not between 0.0 and 1.0 (inclusive).
    ///
    /// # Arguments
    /// * `value` - The percentage value as a float between 0.0 and 1.0.
    pub const fn new_const(value: f64) -> Self {
        if value < 0.0 || value > 1.0 {
            panic!("Percentage must be between 0.0 and 1.0");
        }
        Percentage(value)
    }

    /// Returns the underlying floating-point value of the percentage.
    pub fn get_value(&self) -> f64 {
        self.0
    }
}

/// Helper macro to create a constant [`Percentage`] value at compile time.
///
/// # Example
/// ```
/// use measurements::{percentage, percentage::Percentage};
/// let p = percentage!(0.5);
/// assert_eq!(p.get_value(), 0.5);
/// ```
#[macro_export]
macro_rules! percentage {
    ($val:expr) => {{
        const P: Percentage = Percentage::new_const($val);
        P
    }};
}

#[cfg(test)]
mod percentage {
    use super::*;

    #[test]
    fn get_value() {
        let p = percentage!(1.0);
        assert_eq!(p.get_value(), 1.0);
    }
}
