#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Prefix {
    Tera,
    Giga,
    Mega,
    Kilo,
    None,
    Milli,
    Micro,
    Nano,
    Femto,
}

impl Prefix {
    pub fn get_exp_value(&self) -> i16 {
        match self {
            Self::Tera => 12,
            Self::Giga => 9,
            Self::Mega => 6,
            Self::Kilo => 3,
            Self::None => 0,
            Self::Milli => -3,
            Self::Micro => -6,
            Self::Nano => -9,
            Self::Femto => -12,
        }
    }

    /// Returns a prefix and a exponent reminder
    pub fn from_exp_value(exp: i16) -> (Self, i16) {
        match exp {
            e if e >= 12 => (Self::Tera, exp - Self::Tera.get_exp_value()),
            e if (9..12).contains(&e) => (Self::Giga, exp - Self::Giga.get_exp_value()),
            e if (6..9).contains(&e) => (Self::Mega, exp - Self::Mega.get_exp_value()),
            e if (3..6).contains(&e) => (Self::Kilo, exp - Self::Kilo.get_exp_value()),
            e if (0..3).contains(&e) => (Self::None, exp - Self::None.get_exp_value()),
            e if (-3..0).contains(&e) => (Self::Milli, exp - Self::Milli.get_exp_value()),
            e if (-6..-3).contains(&e) => (Self::Micro, exp - Self::Micro.get_exp_value()),
            e if (-9..-6).contains(&e) => (Self::Nano, exp - Self::Nano.get_exp_value()),
            e if e < -9 => (Self::Femto, exp - Self::Femto.get_exp_value()),
            _ => panic!("should have caught everything"),
        }
    }

    pub fn get_conversion_factor(&self, other: Self) -> f64 {
        let exp = self.get_exp_value() - other.get_exp_value();
        10f64.powi(exp as i32)
    }

    pub fn get_label(&self) -> &str {
        match self {
            Self::Tera => "T",
            Self::Giga => "G",
            Self::Mega => "M",
            Self::Kilo => "k",
            Self::None => "",
            Self::Milli => "m",
            Self::Micro => "u",
            Self::Nano => "n",
            Self::Femto => "f",
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Mul for Prefix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let exp = self.get_exp_value() + rhs.get_exp_value();
        match Prefix::from_exp_value(exp) {
            (p, 0) => p,
            _ => panic!("Should never be here"),
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Div for Prefix {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let exp = self.get_exp_value() - rhs.get_exp_value();
        match Prefix::from_exp_value(exp) {
            (p, 0) => p,
            _ => panic!("Should never be here"),
        }
    }
}

#[test]
fn getter() {
    assert_eq!(Prefix::from_exp_value(4), (Prefix::Kilo, 1));
}

#[cfg(test)]
mod prefix_tests {
    use super::*;

    #[test]
    fn kilo_times_kilo() {
        assert_eq!(Prefix::Kilo * Prefix::Kilo, Prefix::Mega);
    }

    #[test]
    fn kilo_div_by_kilo() {
        assert_eq!(Prefix::Kilo / Prefix::Kilo, Prefix::None);
    }

    #[test]
    fn mega_div_by_kilo() {
        assert_eq!(Prefix::Mega / Prefix::Kilo, Prefix::Kilo);
    }

    #[test]
    #[should_panic]
    fn femto_div_by_femto() {
        let _ = Prefix::Tera * Prefix::Tera;
    }
}
