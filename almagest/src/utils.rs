use core::cmp::{PartialEq, PartialOrd};
use core::fmt::{Debug, Display};
use core::ops::{Add, Div, Mul, Sub};

pub type Real = f64;

/// Archimedes’ constant (π)
pub const PI: Real = 3.14159265358979323846264338327950288;
/// The full circle constant (τ)
/// Equal to 2π.
pub const TAU: Real = 6.28318530717958647692528676655900577;
/// Euler's number (e)
pub const E: Real = 2.71828182845904523536028747135266250;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Meters(pub Real);

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Kilometers(pub Real);

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct MetersSquared(pub Real);

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct MetersCubed(pub Real);

impl Meters {
    pub const ZERO: Self = Meters(0.0);

    pub fn to_km(&self) -> Kilometers {
        Kilometers(self.value() / 1_000.0)
    }

    pub fn value(&self) -> Real {
        self.0
    }
}

impl Add for Meters {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Meters(self.0 + rhs.0)
    }
}

// Meters / Meters = dimensionless ratio
impl Div for Meters {
    type Output = Real;
    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

// Scalar multiplication
impl Mul<Real> for Meters {
    type Output = Self;
    fn mul(self, rhs: Real) -> Self::Output {
        Meters(self.0 * rhs)
    }
}

// Scalar division
impl Div<Real> for Meters {
    type Output = Self;
    fn div(self, rhs: Real) -> Self::Output {
        Meters(self.0 / rhs)
    }
}

// Meters * Meters = MetersSquared (area)
impl Mul for Meters {
    type Output = MetersSquared;
    fn mul(self, rhs: Self) -> Self::Output {
        MetersSquared(self.0 * rhs.0)
    }
}

// Display implementations
impl Display for Meters {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} m", self.0)
    }
}

impl Display for MetersSquared {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} m²", self.0)
    }
}

impl Display for MetersCubed {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} m³", self.0)
    }
}

// MetersSquared operations
impl MetersSquared {
    pub const fn value(self) -> Real { self.0 }
}

// MetersCubed operations  
impl MetersCubed {
    pub const fn value(self) -> Real { self.0 }
}

impl Add for MetersSquared {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { MetersSquared(self.0 + rhs.0) }
}

impl Sub for MetersSquared {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { MetersSquared(self.0 - rhs.0) }
}

impl Mul<Real> for MetersSquared {
    type Output = Self;
    fn mul(self, rhs: Real) -> Self::Output { MetersSquared(self.0 * rhs) }
}

impl Div<Real> for MetersSquared {
    type Output = Self;
    fn div(self, rhs: Real) -> Self::Output { MetersSquared(self.0 / rhs) }
}

// MetersSquared / Meters = Meters
impl Div<Meters> for MetersSquared {
    type Output = Meters;
    fn div(self, rhs: Meters) -> Self::Output { Meters(self.0 / rhs.0) }
}

// Meters * MetersSquared = MetersCubed
impl Mul<MetersSquared> for Meters {
    type Output = MetersCubed;
    fn mul(self, rhs: MetersSquared) -> Self::Output { MetersCubed(self.0 * rhs.0) }
}

// Real * Meters = Meters (commutative scalar multiplication)
impl Mul<Meters> for Real {
    type Output = Meters;
    fn mul(self, rhs: Meters) -> Self::Output { Meters(self * rhs.0) }
}

impl Sub for Meters {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Meters(self.0 - rhs.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Eccentricity(Real);

impl Eccentricity {
    pub fn new(value: Real) -> Result<Self, &'static str> {
        if value < 0.0 {
            Err("Eccentricity cannot be negative")
        } else {
            Ok(Eccentricity(value))
        }
    }

    pub fn value(&self) -> Real {
        self.0
    }
}

#[cfg(test)]
mod units {
    use super::*;
    use approx::assert_relative_eq;

    // === Basic Arithmetic Operations ===
    
    #[test]
    fn meters_addition() {
        let a = Meters(10.0);
        let b = Meters(5.0);
        assert_eq!(a + b, Meters(15.0));
    }

    #[test]
    fn meters_subtraction() {
        let a = Meters(10.0);
        let b = Meters(3.0);
        assert_eq!(a - b, Meters(7.0));
    }

    #[test]
    fn meters_scalar_multiplication() {
        let m = Meters(5.0);
        assert_eq!(m * 3.0, Meters(15.0));
    }

    #[test]
    fn meters_scalar_division() {
        let m = Meters(15.0);
        assert_eq!(m / 3.0, Meters(5.0));
    }

    #[test]
    fn meters_ratio_division() {
        let a = Meters(15.0);
        let b = Meters(3.0);
        assert_relative_eq!(a / b, 5.0, epsilon = 1e-10);
    }

    // === Dimensional Analysis Tests ===
    
    #[test]
    fn meters_multiplication_creates_area() {
        let length = Meters(4.0);
        let width = Meters(3.0);
        let area: MetersSquared = length * width;
        assert_eq!(area.value(), 12.0);
    }

    #[test]
    fn area_division_by_meters_gives_meters() {
        let area = MetersSquared(20.0);
        let width = Meters(4.0);
        let length: Meters = area / width;
        assert_eq!(length, Meters(5.0));
    }

    #[test]
    fn meters_times_area_gives_volume() {
        let height = Meters(2.0);
        let area = MetersSquared(10.0);
        let volume: MetersCubed = height * area;
        assert_eq!(volume.value(), 20.0);
    }

    #[test]
    fn area_arithmetic() {
        let a1 = MetersSquared(10.0);
        let a2 = MetersSquared(5.0);
        assert_eq!(a1 + a2, MetersSquared(15.0));
        assert_eq!(a1 - a2, MetersSquared(5.0));
        assert_eq!(a1 * 2.0, MetersSquared(20.0));
        assert_eq!(a1 / 2.0, MetersSquared(5.0));
    }

    // === Unit Conversion Tests ===
    
    #[test]
    fn meters_convert_to_km() {
        let m = Meters(1_000.0);
        let km = Kilometers(1.0);
        assert_eq!(m.to_km(), km);
    }

    #[test]
    fn meters_convert_precision() {
        let m = Meters(1_234.567);
        let km = m.to_km();
        assert_relative_eq!(km.0, 1.234567, epsilon = 1e-10);
    }

    // === Constants and Special Values ===
    
    #[test]
    fn meters_zero_constant() {
        assert_eq!(Meters::ZERO, Meters(0.0));
        assert_eq!(Meters::ZERO + Meters(5.0), Meters(5.0));
    }

    #[test]
    fn meters_value_accessor() {
        let m = Meters(42.0);
        assert_eq!(m.value(), 42.0);
    }

    // === Edge Cases and Error Conditions ===
    
    #[test]
    fn meters_with_infinity() {
        let inf = Meters(Real::INFINITY);
        let finite = Meters(10.0);
        assert!(inf.value().is_infinite());
        assert!((inf + finite).value().is_infinite());
    }

    #[test]
    fn meters_with_nan() {
        let nan = Meters(Real::NAN);
        assert!(nan.value().is_nan());
        // NaN propagates through operations
        assert!((nan + Meters(5.0)).value().is_nan());
    }

    #[test]
    fn meters_division_by_zero() {
        let m = Meters(10.0);
        let result = m / 0.0;
        assert!(result.value().is_infinite());
    }

    #[test]
    fn zero_divided_by_meters() {
        let zero = Meters(0.0);
        let divisor = Meters(5.0);
        assert_eq!(zero / divisor, 0.0);
    }

    // === Eccentricity Validation Tests ===
    
    #[test]
    fn eccentricity_valid_values() {
        assert!(Eccentricity::new(0.0).is_ok());
        assert!(Eccentricity::new(0.5).is_ok());
        assert!(Eccentricity::new(0.999).is_ok());
        assert!(Eccentricity::new(1.0).is_ok());
    }

    #[test]
    fn eccentricity_invalid_negative() {
        assert!(Eccentricity::new(-0.1).is_err());
        assert!(Eccentricity::new(-1.0).is_err());
    }

    #[test]
    fn eccentricity_value_accessor() {
        let e = Eccentricity::new(0.5).unwrap();
        assert_eq!(e.value(), 0.5);
    }

    // === Comparison and Ordering Tests ===
    
    #[test]
    fn meters_comparison() {
        let a = Meters(5.0);
        let b = Meters(10.0);
        let c = Meters(5.0);
        
        assert!(a < b);
        assert!(b > a);
        assert_eq!(a, c);
        assert!(a <= c);
        assert!(a >= c);
    }

    #[test]
    fn area_comparison() {
        let small = MetersSquared(5.0);
        let large = MetersSquared(10.0);
        
        assert!(small < large);
        assert!(large > small);
    }

    // === Mathematical Properties ===
    
    #[test]
    fn meters_associativity() {
        let a = Meters(2.0);
        let b = Meters(3.0);
        let c = Meters(4.0);
        
        // Addition associativity: (a + b) + c = a + (b + c)
        assert_eq!((a + b) + c, a + (b + c));
    }

    #[test]
    fn meters_commutativity() {
        let a = Meters(7.0);
        let b = Meters(11.0);
        
        // Addition commutativity: a + b = b + a
        assert_eq!(a + b, b + a);
        
        // Multiplication commutativity with dimensionality
        let area1: MetersSquared = a * b;
        let area2: MetersSquared = b * a;
        assert_eq!(area1, area2);
    }

    #[test]
    fn meters_distributivity() {
        let a = Meters(3.0);
        let b = Meters(4.0);
        let scalar = 2.0;
        
        // Scalar distributivity: k(a + b) = ka + kb
        assert_eq!(scalar * (a + b), scalar * a + scalar * b);
    }

    // === Real-world Scale Tests ===
    
    #[test]
    fn orbital_scale_calculations() {
        // Earth's radius
        let earth_radius = Meters(6_371_000.0);
        // ISS altitude
        let iss_altitude = Meters(408_000.0);
        let iss_orbit_radius = earth_radius + iss_altitude;
        
        assert_relative_eq!(iss_orbit_radius.value(), 6_779_000.0, epsilon = 1.0);
        
        // Check that we can compute orbital circumference (2πr)
        let circumference = iss_orbit_radius * (2.0 * PI);
        assert!(circumference.value() > 42_000_000.0); // ~42.6M meters
    }

    #[test]
    fn astronomical_scale_precision() {
        // Earth-Sun distance (1 AU)
        let au = Meters(149_597_870_700.0);
        let half_au = au / 2.0;
        
        assert_relative_eq!(half_au.value(), 74_798_935_350.0, epsilon = 1.0);
    }

    // === Display Implementation Tests ===
    // Note: Display tests removed to maintain no_std compatibility
    // Display trait implementations are still available for debugging
}
