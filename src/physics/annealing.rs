use rand::Rng;
/// Simulated Annealing
/// - Optimizes by mimicking physical annealing process
/// - Adjusts state with temperature-dependent acceptance probability
/// - Cooling rate controls how quickly the temperature decreases
pub struct SimulatedAnnealing {
    pub initial_temp: f64,
    pub cooling_rate: f64,
    pub current_state: f64,
    pub best_state: f64,
    pub best_energy: f64,
}

impl SimulatedAnnealing {
    pub fn new(initial_temp: f64, cooling_rate: f64, initial_state: f64) -> Self {
        Self {
            initial_temp,
            cooling_rate,
            current_state: initial_state,
            best_state: initial_state,
            best_energy: Self::energy(initial_state),
        }
    }

    pub fn energy(state: f64) -> f64 {
        // Example Energy function (e.g, a simple quadratic function)
        state.powi(2)
    }

    pub fn neighbor(&self, state: f64) -> f64 {
        let mut rng = rand::rng();
        state + rng.random_range(-1.0..1.0)
    }

    pub fn accept_probability(&self, old_energy: f64, new_energy: f64, temperature: f64) -> f64 {
        if new_energy < old_energy {
            1.0
        } else {
            ((old_energy - new_energy) / temperature).exp()
        }
    }

    pub fn optimize(&mut self) {
        let mut temperature = self.initial_temp;
        while temperature > 1.0 {
            let new_state = self.neighbor(self.current_state);
            let current_energy = Self::energy(self.current_state);
            let new_energy = Self::energy(new_state);

            if self.accept_probability(current_energy, new_energy, temperature)
                > rand::rng().random::<f64>()
            {
                self.current_state = new_state;
            }

            if new_energy < self.best_energy {
                self.best_state = new_state;
                self.best_energy = new_energy;
            }

            temperature *= 1.0 - self.cooling_rate;
        }
    }

    pub fn get_best_state(&self) -> f64 {
        self.best_state
    }

    pub fn get_best_energy(&self) -> f64 {
        self.best_energy
    }
}

pub fn simulated_annealing_example() {
    let mut sa = SimulatedAnnealing::new(1000.0, 0.003, 10.0);
    sa.optimize();
    println!("Best state: {}", sa.get_best_state());
    println!("Best energy: {}", sa.get_best_energy());
}

#[test]
fn test_simulated_annealing() {
    let mut sa = SimulatedAnnealing::new(1000.0, 0.003, 10.0);
    sa.optimize();
    let best_state = sa.get_best_state();
    let best_energy = sa.get_best_energy();

    // Check that the best energy is close to 0 (global minimum for the quadratic function)
    assert!(best_energy.abs() < 1.0, "Best energy is not close to 0");
    // Check that the best state is close to 0
    assert!(best_state.abs() < 1.0, "Best state is not close to 0");
}
