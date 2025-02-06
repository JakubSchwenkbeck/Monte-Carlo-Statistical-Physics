pub mod algorithms {
    pub mod distributions;
    pub mod gibbs;
    pub mod markov_chain;
    pub mod mc_estimator;
    pub mod metropolis;
    pub mod stochastic_matrix;
}

pub mod models {
    pub mod state;
}
pub mod applications {
    pub mod percolation;
    pub mod pi_estimation;
    pub mod random_walk;
    pub mod web_graph;
}
pub mod physics {
    pub mod hilbert;
    pub mod ising_model;
}
pub mod diagnostics;
