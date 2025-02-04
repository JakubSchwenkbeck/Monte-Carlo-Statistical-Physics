use crate::algorithms::distributions::ProbabilityDistribution;
use crate::algorithms::stochastic_matrix::StochasticMatrix;
use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;

pub struct MarkovChain<T> {
    pub states: Vec<T>,
    pub transition_matrix: StochasticMatrix,
    pub current_state_index: usize,
}

impl<T: Clone> MarkovChain<T> {
    pub fn new(
        states: Vec<T>,
        transition_matrix: StochasticMatrix,
        initial_distribution: ProbabilityDistribution,
    ) -> Self {
        let initial_index = initial_distribution
            .values
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();

        Self {
            states,
            transition_matrix,
            current_state_index: initial_index,
        }
    }

    pub fn step<R: rand::Rng>(&mut self, rng: &mut R) {
        let dist = ProbabilityDistribution {
            values: self.transition_matrix.matrix[self.current_state_index].clone(),
        };
        let new_index = WeightedIndex::new(&dist.values).unwrap().sample(rng);
        self.current_state_index = new_index;
    }

    pub fn get_current_state(&self) -> &T {
        &self.states[self.current_state_index]
    }
}
