#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Percentage(f64);

impl Percentage {
    pub const fn new_const(value: f64) -> Self {
        if value < 0.0 || value > 1.0 {
            panic!("Percentage must be between 0.0 and 1.0");
        }
        Percentage(value)
    }
    pub fn get_value(&self) -> f64 {
        self.0
    }
}

// Helper macro that expands to a const expression
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
