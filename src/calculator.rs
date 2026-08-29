/// Summary statistics computed from a set of scores.
///
/// Both a *sample* and a *population* standard deviation are reported:
/// - **sample** divides the sum of squared deviations by `n - 1`,
/// - **population** divides it by `n`.
#[derive(Debug, Clone, PartialEq)]
pub struct Statistics {
    pub count: usize,
    pub sum: f64,
    pub minimum: f64,
    pub maximum: f64,
    pub average: f64,
    pub variance_sample: f64,
    pub std_dev_sample: f64,
    pub variance_population: f64,
    pub std_dev_population: f64,
}

/// Computes the statistics for a slice of scores.
///
/// Returns `None` when the slice is empty, since there is nothing to measure.
/// For a single value, the sample standard deviation is mathematically
/// undefined, so it is reported as `0.0`.
pub fn calculate(scores: &[f64]) -> Option<Statistics> {
    if scores.is_empty() {
        return None;
    }

    let count = scores.len();
    let sum: f64 = scores.iter().sum();
    let minimum = scores.iter().cloned().fold(f64::INFINITY, f64::min);
    let maximum = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let average = sum / count as f64;

    let squared_deviations: f64 = scores.iter().map(|value| (value - average).powi(2)).sum();

    let variance_population = squared_deviations / count as f64;
    let variance_sample = if count > 1 {
        squared_deviations / (count as f64 - 1.0)
    } else {
        0.0
    };

    Some(Statistics {
        count,
        sum,
        minimum,
        maximum,
        average,
        variance_sample,
        std_dev_sample: variance_sample.sqrt(),
        variance_population,
        std_dev_population: variance_population.sqrt(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(left: f64, right: f64, tolerance: f64) -> bool {
        (left - right).abs() <= tolerance
    }

    #[test]
    fn computes_basic_metrics_for_a_known_dataset() {
        let scores = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let stats = calculate(&scores).expect("dataset is not empty");

        assert_eq!(stats.count, 8);
        assert_eq!(stats.sum, 40.0);
        assert_eq!(stats.minimum, 2.0);
        assert_eq!(stats.maximum, 9.0);
        assert_eq!(stats.average, 5.0);
        assert!(approx(stats.variance_sample, 32.0 / 7.0, 1e-9));
        assert!(approx(stats.std_dev_sample, (32.0_f64 / 7.0).sqrt(), 1e-9));
        assert_eq!(stats.variance_population, 4.0);
        assert_eq!(stats.std_dev_population, 2.0);
    }

    #[test]
    fn returns_none_for_an_empty_dataset() {
        assert!(calculate(&[]).is_none());
    }

    #[test]
    fn handles_negative_values() {
        let scores = [-4.0, -2.0, 0.0, 2.0, 4.0];
        let stats = calculate(&scores).expect("dataset is not empty");

        assert_eq!(stats.minimum, -4.0);
        assert_eq!(stats.maximum, 4.0);
        assert_eq!(stats.average, 0.0);
        assert_eq!(stats.sum, 0.0);
        assert!(approx(stats.variance_sample, 10.0, 1e-9));
        assert!(approx(stats.std_dev_sample, 10.0_f64.sqrt(), 1e-9));
        assert_eq!(stats.variance_population, 8.0);
        assert!(approx(stats.std_dev_population, 8.0_f64.sqrt(), 1e-9));
    }

    #[test]
    fn handles_a_single_value() {
        let stats = calculate(&[42.0]).expect("dataset is not empty");

        assert_eq!(stats.count, 1);
        assert_eq!(stats.sum, 42.0);
        assert_eq!(stats.minimum, 42.0);
        assert_eq!(stats.maximum, 42.0);
        assert_eq!(stats.average, 42.0);
        assert_eq!(stats.std_dev_sample, 0.0);
        assert_eq!(stats.std_dev_population, 0.0);
    }

    #[test]
    fn handles_identical_values_with_zero_spread() {
        let stats = calculate(&[7.0, 7.0, 7.0, 7.0]).expect("dataset is not empty");

        assert_eq!(stats.average, 7.0);
        assert_eq!(stats.minimum, 7.0);
        assert_eq!(stats.maximum, 7.0);
        assert_eq!(stats.variance_sample, 0.0);
        assert_eq!(stats.variance_population, 0.0);
        assert_eq!(stats.std_dev_sample, 0.0);
        assert_eq!(stats.std_dev_population, 0.0);
    }

    #[test]
    fn handles_fractional_scores() {
        let scores = [1.5, 2.5, 3.5, 4.5];
        let stats = calculate(&scores).expect("dataset is not empty");

        assert_eq!(stats.average, 3.0);
        assert_eq!(stats.minimum, 1.5);
        assert_eq!(stats.maximum, 4.5);
        assert!(approx(stats.variance_sample, 5.0 / 3.0, 1e-9));
        assert!(approx(stats.variance_population, 1.25, 1e-9));
    }
}
