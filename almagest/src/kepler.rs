use libm::sqrt;

use crate::utils::{Eccentricity, Meters, Real};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    x: Meters,
    y: Meters,
}
// impl Point {
//     pub fn new(x: Meters, y: Meters) -> Self {
//         Point(x, y)
//     }
// }

pub struct Ellipse {
    // Eccentricity
    e: Eccentricity,
    // Primary focus
    f: Point,
    // Radius of periapsis
    r_p: Meters,
}

impl Ellipse {
    pub fn new(e: Eccentricity, f: Point, r_p: Meters) -> Self {
        Ellipse { e, f, r_p }
    }

    /// Construct an ellipse from periapsis and apoapsis distances.
    pub fn from_periapsis_apoapsis(r_p: Meters, r_a: Meters, f: Point) -> Self {
        let e = (r_a.value() - r_p.value()) / (r_a.value() + r_p.value());
        Ellipse {
            e: Eccentricity::new(e).unwrap(),
            f,
            r_p,
        }
    }

    pub fn eccentricity(&self) -> Eccentricity {
        self.e
    }

    /// The gravitational center of attraction
    pub fn primary_focus(&self) -> Point {
        self.f
    }

    /// The distance from the primary focus to the
    /// nearest edge of the ellipse, along the
    /// semi-major axis
    pub fn periapsis(&self) -> Meters {
        self.r_p
    }

    /// Half of the long axis of the ellipse,
    /// denoted in formula by `a`
    pub fn semi_major_axis(&self) -> Meters {
        self.periapsis() / (1.0 - self.eccentricity().value())
    }

    /// Half of the short axis of the ellipse,
    /// denoted in formula by `b`
    pub fn semi_minor_axis(&self) -> Meters {
        // b = a * sqrt(1 - ecc^2)
        Meters(
            self.semi_major_axis().value()
                * sqrt(1.0 - (self.eccentricity().value() * self.eccentricity().value())),
        )
    }

    /// Describe the shape of the ellipse;
    /// an alternative to using the eccentricity
    pub fn flattening(&self) -> Real {
        let a = self.semi_major_axis() - self.semi_minor_axis();
        a / self.semi_major_axis()
    }

    /// The distance from the primary focus to the
    /// far edge of the ellipse, along the major axis
    pub fn apoapsis(&self) -> Meters {
        Meters(self.semi_major_axis().value() * (1.0 + self.eccentricity().value()))
    }

    /// The distance between foci;
    /// half of the this value is denoted in formula
    /// by `c`
    pub fn focal_distance(&self) -> Meters {
        Meters(self.eccentricity().value() * self.semi_major_axis().value())
    }
}

/// Calculate double the length of the semimajor axis,
/// using the distance from the primary focus to a point
/// on the orbit as well as the distance from the secondary
/// focus to the same point.
pub fn calc_2a(r_f: Meters, r_f_p: Meters) -> Meters {
    r_f + r_f_p
}

/// Calculate the distance between the two foci,
/// using the distance from the primary focus to a point
/// on the orbit as well as the distance from the secondary
/// focus to the same point.
pub fn calc_2c(r_f: Meters, r_f_p: Meters) -> Meters {
    if r_f.value() > r_f_p.value() {
        r_f - r_f_p
    } else {
        r_f_p - r_f
    }
}

/// Calculate the eccentricity of an orbit from the lengths
/// of both foci to a single point on the orbit.
// TODO: lots of tests to ensure the returned value is never negative
pub fn calc_ecc(r_f: Meters, r_f_p: Meters) -> Eccentricity {
    let two_a = calc_2a(r_f, r_f_p);
    let two_c = calc_2c(r_f, r_f_p);
    let a = two_a.value() / 2.0;
    let c = two_c.value() / 2.0;
    Eccentricity::new(c / a).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn has_focus() {
        let f = Point {
            x: Meters(1.0),
            y: Meters(1.0),
        };
        let e = Ellipse {
            e: Eccentricity::new(1.0).unwrap(),
            f: f,
            r_p: Meters(1.0),
        };
        assert_eq!(e.f, f);
    }

    #[test]
    fn calcs_semi_major_axis() {
        let f = Point {
            x: Meters(1.0),
            y: Meters(1.0),
        };
        let e = Ellipse {
            e: Eccentricity::new(0.5).unwrap(),
            f: f,
            r_p: Meters(1.0),
        };
        let expected = Meters(2.0);
        assert_eq!(e.semi_major_axis(), expected);
    }

    // Test case 1: Circle (e = 0)
    #[test]
    fn test_circle() {
        let ellipse = Ellipse {
            e: Eccentricity::new(0.0).unwrap(),
            f: Point {
                x: Meters(0.0),
                y: Meters(0.0),
            },
            r_p: Meters(1000.0),
        };

        // For a circle: r_p = r_a = a = b
        assert_relative_eq!(ellipse.semi_major_axis().0, 1000.0, epsilon = 1e-10);
        assert_relative_eq!(ellipse.semi_minor_axis().0, 1000.0, epsilon = 1e-10);
        assert_relative_eq!(ellipse.apoapsis().0, 1000.0, epsilon = 1e-10);
        assert_relative_eq!(ellipse.focal_distance().0, 0.0, epsilon = 1e-10);
    }

    // Test case 2: Earth's orbit (approximately)
    #[test]
    fn test_earth_orbit() {
        let e_val = 0.0167; // Earth's orbital eccentricity
        let r_p_val = 147_097_000_000.0; // Perihelion in meters

        let ellipse = Ellipse {
            e: Eccentricity::new(e_val).unwrap(),
            f: Point {
                x: Meters(0.0),
                y: Meters(0.0),
            },
            r_p: Meters(r_p_val),
        };

        let expected_a = 149_595_240_516.6277;
        let expected_r_a = 152_093_481_033.25537;

        assert_relative_eq!(ellipse.semi_major_axis().0, expected_a, epsilon = 1e-6);
        assert_relative_eq!(ellipse.apoapsis().0, expected_r_a, epsilon = 1e-6);
    }

    // Test case 3: Highly eccentric orbit (comet-like)
    #[test]
    fn test_highly_eccentric_orbit() {
        let ellipse = Ellipse {
            e: Eccentricity::new(0.9).unwrap(),
            f: Point {
                x: Meters(0.0),
                y: Meters(0.0),
            },
            r_p: Meters(1000.0),
        };

        // For e = 0.9, r_p = 1000:
        // a = r_p / (1 - e) = 1000 / 0.1 = 10000
        // r_a = a(1 + e) = 10000 * 1.9 = 19000
        // b = a * sqrt(1 - e²) = 10000 * sqrt(0.19) ≈ 4358.9
        // c = a * e = 9000

        assert_relative_eq!(ellipse.semi_major_axis().0, 10000.0, epsilon = 1e-10);
        assert_relative_eq!(ellipse.apoapsis().0, 19000.0, epsilon = 1e-10);
        assert_relative_eq!(
            ellipse.semi_minor_axis().0,
            4358.898943540674,
            epsilon = 1e-6
        );
        assert_relative_eq!(ellipse.focal_distance().0, 9000.0, epsilon = 1e-6);
    }

    // Test case 4: Low Earth Orbit (ISS-like)
    #[test]
    fn test_low_earth_orbit() {
        let earth_radius = 6_371_000.0; // Earth radius in meters
        let ellipse = Ellipse {
            e: Eccentricity::new(0.0002).unwrap(), // Very low eccentricity
            f: Point {
                x: Meters(0.0),
                y: Meters(0.0),
            },
            r_p: Meters(earth_radius + 408_000.0), // ~408 km altitude at perigee
        };

        // For nearly circular LEO
        let expected_a = (earth_radius + 408_000.0) / (1.0 - 0.0002);
        let expected_r_a = expected_a * (1.0 + 0.0002);

        assert_relative_eq!(ellipse.semi_major_axis().0, expected_a, epsilon = 1.0);
        assert_relative_eq!(ellipse.apoapsis().0, expected_r_a, epsilon = 1.0);
    }

    // Test case 5: Geostationary Transfer Orbit (GTO)
    #[test]
    fn test_geostationary_transfer_orbit() {
        let earth_radius = 6_371_000.0;
        let ellipse = Ellipse {
            e: Eccentricity::new(0.7308).unwrap(), // Typical GTO eccentricity
            f: Point {
                x: Meters(0.0),
                y: Meters(0.0),
            },
            r_p: Meters(earth_radius + 200_000.0), // 200 km perigee
        };

        // Calculate expected values using orbital mechanics formulas
        let expected_a = ellipse.r_p.0 / (1.0 - ellipse.e.value());
        let expected_r_a = 2.0 * expected_a - ellipse.r_p.0;

        assert_relative_eq!(ellipse.semi_major_axis().0, expected_a, epsilon = 1e3);
        assert_relative_eq!(ellipse.apoapsis().0, expected_r_a, epsilon = 1e5);
    }

    // Test case 6: Parabolic trajectory (e = 1.0)
    #[test]
    fn test_parabolic_trajectory() {
        let ellipse = Ellipse {
            e: Eccentricity::new(1.0).unwrap(),
            f: Point {
                x: Meters(0.0),
                y: Meters(0.0),
            },
            r_p: Meters(1000.0),
        };

        // For parabolic orbit: a approaches infinity, r_a approaches infinity
        // This is a degenerate case that might need special handling
        assert!(ellipse.semi_major_axis().0.is_infinite());
        assert!(ellipse.apoapsis().0.is_infinite());
    }

    // Test case 6b: Ellipse constructor from periapsis and apoapsis
    #[test]
    fn test_from_periapsis_apoapsis_constructor() {
        let r_p = Meters(6_571_000.0); // Example: 200 km above Earth's surface
        let r_a = Meters(42_157_000.0); // Example: geostationary altitude
        let f = Point {
            x: Meters(0.0),
            y: Meters(0.0),
        };

        let ellipse = Ellipse::from_periapsis_apoapsis(r_p, r_a, f);

        let expected_a = (r_p.0 + r_a.0) / 2.0;
        let expected_e = (r_a.0 - r_p.0) / (r_a.0 + r_p.0);

        assert_relative_eq!(ellipse.semi_major_axis().0, expected_a, epsilon = 1e-6);
        assert_relative_eq!(ellipse.e.value(), expected_e, epsilon = 1e-6);
        assert_relative_eq!(ellipse.apoapsis().0, r_a.0, epsilon = 1e-6);
        assert_relative_eq!(ellipse.periapsis().0, r_p.0, epsilon = 1e-6);
    }

    // Test case 7: Edge case - very small periapsis
    #[test]
    fn test_small_periapsis() {
        let ellipse = Ellipse {
            e: Eccentricity::new(0.5).unwrap(),
            f: Point {
                x: Meters(0.0),
                y: Meters(0.0),
            },
            r_p: Meters(1.0), // 1 meter periapsis
        };

        // a = r_p / (1 - e) = 1 / 0.5 = 2
        // r_a = a(1 + e) = 2 * 1.5 = 3
        assert_relative_eq!(ellipse.semi_major_axis().0, 2.0, epsilon = 1e-10);
        assert_relative_eq!(ellipse.apoapsis().0, 3.0, epsilon = 1e-10);
    }

    // Test case 8: Mathematical precision test
    #[test]
    fn test_mathematical_relationships() {
        let ellipse = Ellipse {
            e: Eccentricity::new(0.3).unwrap(),
            f: Point {
                x: Meters(0.0),
                y: Meters(0.0),
            },
            r_p: Meters(7000.0),
        };

        let a = ellipse.semi_major_axis().0;
        let b = ellipse.semi_minor_axis().0;
        let c = ellipse.focal_distance().0;
        let r_a = ellipse.apoapsis().0;
        let e = ellipse.e.value();

        // Test fundamental ellipse relationships
        assert_relative_eq!(c * c + b * b, a * a, epsilon = 1e-6); // c² + b² = a²
        assert_relative_eq!(c, a * e, epsilon = 1e-10); // c = ae
        assert_relative_eq!(ellipse.r_p.0, a * (1.0 - e), epsilon = 1e-10); // r_p = a(1-e)
        assert_relative_eq!(r_a, a * (1.0 + e), epsilon = 1e-10); // r_a = a(1+e)
        assert_relative_eq!(b, a * (1.0 - e * e).sqrt(), epsilon = 1e-10); // b = a√(1-e²)
    }

    // Property-based test helper
    #[test]
    fn test_eccentricity_bounds() {
        // Test that calculations work for valid eccentricity range [0, 1)
        // First, for a circle:
        let ellipse = Ellipse {
            e: Eccentricity::new(0.0).unwrap(),
            f: Point {
                x: Meters(0.0),
                y: Meters(0.0),
            },
            r_p: Meters(1000.0),
        };

        // All calculated values should be positive and finite
        assert!(ellipse.semi_major_axis().0 > 0.0);
        assert!(ellipse.semi_major_axis().0.is_finite());
        assert!(ellipse.semi_minor_axis().0 > 0.0);
        assert!(ellipse.semi_minor_axis().0.is_finite());
        assert!(ellipse.apoapsis().0 == ellipse.r_p.0);

        // Then, for the other ranges:
        for e_val in [0.1, 0.25, 0.5, 0.75, 0.9, 0.95, 0.99, 0.999] {
            let ellipse = Ellipse {
                e: Eccentricity::new(e_val).unwrap(),
                f: Point {
                    x: Meters(0.0),
                    y: Meters(0.0),
                },
                r_p: Meters(1000.0),
            };

            // All calculated values should be positive and finite
            assert!(ellipse.semi_major_axis().0 > 0.0);
            assert!(ellipse.semi_major_axis().0.is_finite());
            assert!(ellipse.semi_minor_axis().0 > 0.0);
            assert!(ellipse.semi_minor_axis().0.is_finite());
            assert!(ellipse.apoapsis().0 > ellipse.r_p.0);

            if e_val < 1.0 {
                assert!(ellipse.apoapsis().0.is_finite());
            }
        }
    }
}
