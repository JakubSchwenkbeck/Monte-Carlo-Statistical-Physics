use monte_carlo::algorithms::distributions::UniformDistribution;
use monte_carlo::algorithms::markov_chain::MarkovChain;

fn main() {
    let initial_state: f64 = 0.0;

    // Define a uniform distribution for transitions.
    let distribution = UniformDistribution {
        lower: -1.0,
        upper: 1.0,
    };

    let mut chain = MarkovChain::new(initial_state, distribution);

    // Advance the Markov chain a few steps.
    for _ in 0..10 {
        chain.step();
        println!("New state: {}", chain.current_state);
    }
}
