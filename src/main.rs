use monte_carlo::algorithms::distributions::{ProbabilityDistribution, RandomVariable};

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
}
