use rand::{Rng, rng};

/// A custom-implemented ringbuffer to store data history
pub mod ringbuffer;
pub use crate::ringbuffer::RingBuffer;

#[cfg(test)]
mod tests;

/// Parameters for configuring simulation behaviour
#[derive(Copy, Clone)]
pub struct SimulationConfig {
    /// Where values with no history should randomly spawn
    pub starting_position: StartingPosition,
    /// The weight of drift on simulating values
    pub drift_scale: f64,
    /// The curve type used to calculate momentum
    pub momentum_curve: CurveType,
    /// The weight of momentum on simulating values
    pub momentum_scale: f64,
    /// The curve type used to calculate reversion to the centre
    pub reversion_curve: CurveType,
    /// The weight of reversion on simulating values
    pub reversion_scale: f64,
    /// Whether simulated values should be hard clamped to the range
    pub clamped: bool,
}

/// Simulates a next value from the given history
pub fn simulate<'a, I: Iterator<Item = &'a f64> + Clone>(
    min: f64,
    max: f64,
    history: I,
    len: usize,
    config: SimulationConfig,
) -> f64 {
    let mut rng = rng();
    let range = max - min;

    if len == 0 {
        let offset_modifier = match config.starting_position {
            StartingPosition::BottomThird => 0.00,
            StartingPosition::MiddleThird => 0.33,
            StartingPosition::TopThird => 0.66,
        };
        let output = min + range * offset_modifier + range * rng.random::<f64>() * 0.33;
        return output;
    }

    let drift_strength = range * config.drift_scale;
    let drift = rng.random_range(-drift_strength..drift_strength);

    let momentum = if len == 1 {
        0.0
    } else {
        let mut deltas = Vec::with_capacity(len - 1);
        let mut iter = history.clone();
        let mut prev = iter
            .next()
            .expect("Iterator should not be empty; this is a bug!");

        let iter = history.clone();
        iter.for_each(|cur| {
            deltas.push(cur - prev);
            prev = cur;
        });

        let weights: Vec<_> = (1..=deltas.len())
            .map(|i| {
                let i = i as f64;
                match config.momentum_curve {
                    CurveType::Linear => i,
                    CurveType::Quadratic => i.powi(2),
                    CurveType::Logarithmic => (i + 1.0).ln(),
                }
            })
            .collect();
        let average_momentum = deltas
            .iter()
            .zip(weights.iter())
            .map(|(d, w)| d * w)
            .sum::<f64>()
            / weights.iter().sum::<f64>();
        average_momentum * config.momentum_scale
    };

    let centre = (min + max) / 2.0;
    let current_value = history
        .last()
        .expect("Ringbuffer should not be empty; this is a bug!");
    let distance_from_centre = current_value - centre;
    let reversion_modifier: fn(f64) -> f64 = match config.reversion_curve {
        CurveType::Linear => |i| i,
        CurveType::Quadratic => |i| i.powi(2),
        CurveType::Logarithmic => |i| i.ln(),
    };
    let reversion = -distance_from_centre * reversion_modifier(config.reversion_scale);

    let mut next_value = current_value + drift + momentum + reversion;
    if config.clamped {
        next_value = next_value.clamp(min, max);
    }
    next_value
}

/// Constructs a [`SimulationConfig`] with the following defaults:
/// - `starting_position`: MiddleThird
/// - `drift_scale`: 0.05
/// - `momentum_curve`: Logarithmic
/// - `momentum_scale`: 0.3
/// - `reversion_curve`: Quadratic
/// - `reversion_scale`: 0.05
/// - `clamped`: `false`
impl Default for SimulationConfig {
    fn default() -> Self {
        SimulationConfig {
            starting_position: StartingPosition::MiddleThird,
            drift_scale: 0.05,
            momentum_curve: CurveType::Logarithmic,
            momentum_scale: 0.3,
            reversion_curve: CurveType::Quadratic,
            reversion_scale: 0.05,
            clamped: false,
        }
    }
}

/// A starting position for new simulations
#[derive(Copy, Clone)]
pub enum StartingPosition {
    BottomThird,
    MiddleThird,
    TopThird,
}

/// A curve type to be used
#[derive(Copy, Clone)]
pub enum CurveType {
    Linear,
    Quadratic,
    Logarithmic,
}
