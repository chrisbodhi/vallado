# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Almagest is a `no_std` compatible Rust library for astrodynamics and orbital mechanics, implementing algorithms from Vallado's "Fundamentals of Astrodynamics and Applications". The library prioritizes type safety, mathematical precision, and dimensional analysis to prevent unit conversion errors in mission-critical aerospace applications.

## Architecture

### Core Design Principles
- **Type-Safe Units**: Extensive use of wrapper types (`Meters`, `MetersSquared`, `Kilometers`, `Eccentricity`) with compile-time dimensional analysis
- **no_std Compatibility**: Works in embedded and resource-constrained environments
- **Mathematical Precision**: Uses `f64` (`Real` type alias) for high-precision astronomical calculations
- **Validation**: Input validation for physical constraints (e.g., eccentricity bounds)

### Module Structure
- `utils.rs`: Type-safe unit system with dimensional analysis, mathematical constants
- `kepler.rs`: Elliptical orbit calculations, Kepler's laws implementation  
- `lib.rs`: Library root with feature flags (`std` feature available)

### Unit System Architecture
The heart of the library is the sophisticated unit system in `utils.rs`:

```rust
// Basic units with dimensional safety
Meters * Meters = MetersSquared     // Area calculations
Meters * MetersSquared = MetersCubed // Volume calculations
MetersSquared / Meters = Meters     // Dimensional reduction
Meters / Meters = Real              // Dimensionless ratios
```

Key features:
- Operator overloading for natural mathematical expressions
- Automatic dimensional analysis prevents unit mismatches at compile time
- Comprehensive trait implementations (Add, Sub, Mul, Div, Display, PartialOrd)
- Constants optimized for orbital mechanics (PI, TAU, E)

### Mathematical Foundations
- Implements elliptical orbit mechanics with periapsis/apoapsis calculations
- Supports highly eccentric orbits (comets) and circular orbits (satellites)
- Comprehensive test coverage using real orbital data (Earth's orbit, ISS, GTO)
- Handles edge cases (parabolic trajectories, infinity, NaN propagation)

## Common Development Commands

### Building and Testing
```bash
# Build the library
cargo build

# Run all tests (40+ comprehensive unit tests)
cargo test 

# Run specific test module
cargo test utils::units
cargo test kepler::tests

# Run single test
cargo test meters_addition

# Build with no_std (default)
cargo build --no-default-features

# Build with std support
cargo build --features std
```

### Development Environment
```bash
# The project uses Nix for reproducible development environments
nix develop  # If flake.nix is present

# Work with Jupyter notebooks for exploration
# (Check ../Chapter01.ipynb for interactive examples)
```

## Usage Patterns

### Creating and Using Units
```rust
use almagest::utils::{Meters, Kilometers, MetersSquared, Eccentricity, Real};

// Basic unit creation
let distance = Meters(1000.0);
let area = Meters(10.0) * Meters(20.0); // MetersSquared(200.0)

// Unit conversions
let km = distance.to_km(); // Kilometers(1.0)

// Validated construction
let ecc = Eccentricity::new(0.5)?; // Validates non-negative
```

### Orbital Mechanics
```rust
use almagest::kepler::{Ellipse, Point};

// Create orbital ellipse
let ellipse = Ellipse::from_periapsis_apoapsis(
    Meters(6_571_000.0),  // 200km above Earth
    Meters(42_157_000.0), // Geostationary altitude  
    Point { x: Meters(0.0), y: Meters(0.0) }
);

// Calculate orbital properties
let semi_major = ellipse.semi_major_axis();
let eccentricity = ellipse.eccentricity();
```

## Testing Philosophy

The test suite emphasizes safety-critical validation:
- **Dimensional Analysis**: Ensures compile-time unit safety
- **Edge Cases**: NaN, infinity, division by zero handling
- **Mathematical Properties**: Associativity, commutativity verification
- **Real-World Data**: Tests using actual orbital parameters
- **Precision**: Uses `approx` crate for floating-point comparisons

## Key Dependencies
- `libm`: Mathematical functions for `no_std` compatibility
- `approx`: Floating-point comparison in tests

## Development Notes
- Always use the type-safe unit system rather than raw `f64` values
- Add comprehensive tests when extending the unit system
- Maintain `no_std` compatibility - avoid `std` dependencies
- Use `Real` type alias instead of `f64` for consistency
- Validate inputs for physical constraints (eccentricity bounds, etc.)