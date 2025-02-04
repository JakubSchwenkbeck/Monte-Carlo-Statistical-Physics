use monte_carlo::algorithms::distributions::Distribution;
use monte_carlo::algorithms::markov_chain::MarkovChain;

pub struct DummyDistribution<T: Clone> {
    pub new_state: T,
}

impl<T: Clone> Distribution<T> for DummyDistribution<T> {
    fn sample(&self, _current_state: &T) -> T {
        self.new_state.clone()
    }
}

#[test]
fn test_markov_chain_single_step() {
    let initial_state = 0;
    let dummy_distribution = DummyDistribution { new_state: 42 };
    let mut chain = MarkovChain::new(initial_state, dummy_distribution);
    chain.step();
    assert_eq!(chain.current_state, 42);
}

#[derive(Clone, Debug, PartialEq)]
struct IncrementState(i32);

struct IncrementDistribution;

impl Distribution<IncrementState> for IncrementDistribution {
    fn sample(&self, current_state: &IncrementState) -> IncrementState {
        IncrementState(current_state.0 + 1)
    }
}

#[test]
fn test_markov_chain_multiple_steps() {
    let initial_state = IncrementState(0);
    let increment_distribution = IncrementDistribution;
    let mut chain = MarkovChain::new(initial_state.clone(), increment_distribution);
    for _ in 0..5 {
        chain.step();
    }
    assert_eq!(chain.current_state, IncrementState(initial_state.0 + 5));
}
