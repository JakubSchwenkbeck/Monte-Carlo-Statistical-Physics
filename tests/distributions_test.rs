use monte_carlo::algorithms::distributions::ProbabilityDistribution;

#[test]
fn test_probability_distribution() {
    let distribution = ProbabilityDistribution::new(vec![0.5, 0.5]);
    assert_eq!(distribution.values.len(), 2);
}
