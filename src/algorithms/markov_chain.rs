use crate::algorithms::distributions::Distribution;

// A generic Markov chain, where T is the type representing a state,
// and D is a distribution that given a state returns a new candidate state.
pub struct MarkovChain<T, D>
where
    D: Distribution<T>,
{
    pub current_state: T,
    pub transition_distribution: D,
}

impl<T, D> MarkovChain<T, D>
where
    D: Distribution<T>,
{
    pub fn new(initial_state: T, transition_distribution: D) -> Self {
        Self {
            current_state: initial_state,
            transition_distribution,
        }
    }

    /// Advance the chain by one step.
    pub fn step(&mut self) {
        // In a simple Markov chain, we sample a new state.
        let new_state = self.transition_distribution.sample(&self.current_state);
        // CAN include acceptance criteria here
        // (for Metropolis-Hastings algorithm)
        self.current_state = new_state;
    }
}
