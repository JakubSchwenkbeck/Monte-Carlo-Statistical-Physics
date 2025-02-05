use monte_carlo::algorithms::metropolis::Metropolis;

#[test]
fn test_metropolis_gaussian() {
    // Target distribution: Standard Gaussian (up to a normalizing constant)
    fn target_distribution(x: f64) -> f64 {
        (-x * x / 2.0).exp()
    }

    let metropolis = Metropolis::new(target_distribution, 1.0);
    let samples = metropolis.sample(10_000, 0.0);

    // Compute mean and variance of the samples
    let mean: f64 = samples.iter().sum::<f64>() / samples.len() as f64;
    let variance: f64 =
        samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / samples.len() as f64;

    // Check that the mean is close to 0 and variance is close to 1
    assert!((mean.abs() - 0.0).abs() < 0.1, "Mean is not close to 0");
    assert!((variance - 1.0).abs() < 0.1, "Variance is not close to 1");
}
