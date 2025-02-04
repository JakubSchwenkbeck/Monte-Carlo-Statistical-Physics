use monte_carlo::algorithms::distributions::ProbabilityDistribution;
use monte_carlo::algorithms::markov_chain::MarkovChain;
use monte_carlo::algorithms::stochastic_matrix::StochasticMatrix;

#[test]
fn test_markov_chain() {
    let states = vec!["A", "B"];
    let matrix = StochasticMatrix::new(vec![vec![0.8, 0.2], vec![0.4, 0.6]]);
    let initial_dist = ProbabilityDistribution::new(vec![1.0, 0.0]);
    let mut chain = MarkovChain::new(states.clone(), matrix, initial_dist);

    let mut rng = rand::rng();
    chain.step(&mut rng);
    assert!(states.contains(chain.get_current_state()));
}
