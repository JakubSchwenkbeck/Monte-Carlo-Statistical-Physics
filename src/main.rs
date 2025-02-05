use monte_carlo::algorithms::distributions::{ProbabilityDistribution, RandomVariable};
use monte_carlo::algorithms::metropolis::metropolis_example;
use monte_carlo::applications::pi_estimation::estimate_pi;
use monte_carlo::applications::web_graph::web_graph;

fn main() {
    // Create a random variable representing a coin toss.
    let state_space = vec!["heads", "tails"];
    let distribution = ProbabilityDistribution::new(vec![0.5, 0.5]);
    let coin = RandomVariable {
        state_space,
        distribution,
    };

    let mut rng = rand::rng();
    // Sample an outcome
    let outcome = coin.sample(&mut rng);
    println!("Coin toss outcome: {}", outcome);

    // Create a random variable for a six-sided die.
    let die_state_space = vec![1, 2, 3, 4, 5, 6];
    let die_distribution = ProbabilityDistribution::new(vec![1.0 / 6.0; 6]);
    let die = RandomVariable {
        state_space: die_state_space,
        distribution: die_distribution,
    };

    let outcome_die = die.sample(&mut rng);
    println!("Die roll outcome: {}", outcome_die);

    let pi_estimate = estimate_pi(1_000_000);
    println!("Estimated π: {}", pi_estimate);

    web_graph();

    metropolis_example()
}
