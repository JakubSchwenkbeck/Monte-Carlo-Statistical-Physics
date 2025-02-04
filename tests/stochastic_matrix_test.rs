use monte_carlo::algorithms::distributions::ProbabilityDistribution;
use monte_carlo::algorithms::stochastic_matrix::StochasticMatrix;

#[test]
fn test_stochastic_matrix() {
    let matrix = StochasticMatrix::new(vec![vec![0.5, 0.5], vec![0.3, 0.7]]);
    let initial_dist = ProbabilityDistribution::new(vec![1.0, 0.0]);
    println!("got here");
    let result = matrix.multiply_with_distribution(&initial_dist);
    assert_eq!(result.values, vec![0.5, 0.5]);
}
