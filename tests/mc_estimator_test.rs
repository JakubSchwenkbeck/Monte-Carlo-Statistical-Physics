use monte_carlo::algorithms::distributions::ProbabilityDistribution;
use monte_carlo::algorithms::mc_estimator::MonteCarloEstimator;
use monte_carlo::algorithms::stochastic_matrix::StochasticMatrix;

#[test]
fn test_monte_carlo_estimator() {
    let matrix = StochasticMatrix::new(vec![vec![0.5, 0.5], vec![0.3, 0.7]]);
    let estimator = MonteCarloEstimator::new(matrix);
    let initial = ProbabilityDistribution::new(vec![1.0, 0.0]);

    let result = estimator.estimate(&initial, 10, 100_000);

    // Should approximate [0.375, 0.625] - exact stationary distribution
    assert!((result.values[0] - 0.375).abs() < 0.01);
    assert!((result.values[1] - 0.625).abs() < 0.01);
}
