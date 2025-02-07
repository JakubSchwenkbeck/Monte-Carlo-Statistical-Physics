use rand::Rng;
/// Quantum Monte Carlo simulation for estimating ground state energy
/// - Uses wave function and local energy to estimate the quantum system's energy.
pub struct QuantumMonteCarlo {
    pub num_samples: usize,
    pub alpha: f64,
    pub positions: Vec<f64>,
}

impl QuantumMonteCarlo {
    pub fn new(num_samples: usize, alpha: f64) -> Self {
        let positions = vec![0.0; num_samples];
        Self {
            num_samples,
            alpha,
            positions,
        }
    }

    pub fn wave_function(&self, x: f64) -> f64 {
        (-self.alpha * x.powi(2)).exp()
    }

    pub fn local_energy(&self, x: f64) -> f64 {
        -0.5 * (self.wave_function(x).ln().powi(2) + 2.0 * self.alpha)
    }

    pub fn simulate(&mut self) {
        let mut rng = rand::rng();
        for i in 0..self.num_samples {
            let x = rng.random_range(-2.0..2.0);
            self.positions[i] = x;
        }
    }

    pub fn estimate_energy(&self) -> f64 {
        let mut energy_sum = 0.0;
        for &x in &self.positions {
            energy_sum += self.local_energy(x);
        }
        energy_sum / self.num_samples as f64
    }
}

pub fn quantum_monte_carlo_example() {
    let mut qmc = QuantumMonteCarlo::new(1000, 0.5);
    qmc.simulate();
    let estimated_energy = qmc.estimate_energy();
    println!("Estimated ground state energy: {}", estimated_energy);
}

#[test]
fn test_quantum_monte_carlo() {
    let mut qmc = QuantumMonteCarlo::new(1000, 0.5);
    qmc.simulate();
    let estimated_energy = qmc.estimate_energy();

    // Check that the estimated energy is within a reasonable range
    assert!(
        estimated_energy < 0.0,
        "Estimated energy should be negative"
    );
    assert!(
        estimated_energy > -1.0,
        "Estimated energy should be greater than -1.0"
    );
}
