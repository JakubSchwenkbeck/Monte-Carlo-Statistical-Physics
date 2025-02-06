pub fn autocorrelation(samples: &[f64], lag: usize) -> f64 {
    let n = samples.len();
    let mean: f64 = samples.iter().sum::<f64>() / n as f64;
    let var: f64 = samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;
    if var.abs() < 1e-12 {
        return 0.0;
    }
    let mut cov = 0.0;
    for i in 0..n - lag {
        cov += (samples[i] - mean) * (samples[i + lag] - mean);
    }
    cov / ((n - lag) as f64 * var)
}
#[test]
fn test_autocorrelation() {
    let samples = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    // With a linear series the autocorrelation at lag=1 should be positive.
    let ac = autocorrelation(&samples, 1);
    println!("Autocorrelation lag 1: {}", ac);
    assert!(ac > 0.0);
}
