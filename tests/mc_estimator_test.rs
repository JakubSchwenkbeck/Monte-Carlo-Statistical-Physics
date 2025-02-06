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
#[test]
fn test_weighted_estimator() {
    // A simple 3-state system.
    let matrix = vec![
        vec![0.8, 0.1, 0.1],
        vec![0.2, 0.7, 0.1],
        vec![0.1, 0.2, 0.7],
    ];
    let transition_matrix = StochasticMatrix { matrix };

    // Assume the stationary distribution is known.
    let pi = vec![0.5, 0.3, 0.2];
    // Define an initial distribution that is uniform over the 3 states.
    let initial = ProbabilityDistribution::new(vec![1.0 / 3.0; 3]);

    let estimator = MonteCarloEstimator::new(transition_matrix);
    // Our observable f(i) = i (just the state index).
    let f = |state: usize| state as f64;

    // Run the simulation with a moderate number of steps and simulations.
    let estimate = estimator.estimate_weighted(&initial, 10, 10_000, f, &pi);
    println!("Weighted estimator value: {}", estimate);

    //toy test  check that the value lies within an expected range.
    assert!(estimate >= 0.0 && estimate <= 2.0);
}

#[test]
fn test_energy_estimator() {
    let matrix = vec![
        vec![0.8, 0.1, 0.1],
        vec![0.2, 0.7, 0.1],
        vec![0.1, 0.2, 0.7],
    ];
    let transition_matrix = StochasticMatrix { matrix };

    let pi = vec![0.5, 0.3, 0.2];
    let initial = ProbabilityDistribution::new(vec![1.0 / 3.0; 3]);

    let estimator = MonteCarloEstimator::new(transition_matrix);

    // Hamiltonian: H(i)=i.
    let h = |state: usize| state as f64;
    let beta = 1.0;

    let energy_estimate = estimator.estimate_energy(&initial, 10, 10_000, h, beta, &pi);
    println!("Estimated Energy: {}", energy_estimate);

    // Check that the energy estimate is within a plausible range.
    assert!(energy_estimate >= 0.0 && energy_estimate <= 2.0);
}
