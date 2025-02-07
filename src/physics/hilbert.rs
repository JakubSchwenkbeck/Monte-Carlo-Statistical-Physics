/// Trait for Hamiltonian systems
/// - Provides a method to calculate the energy of a given state
///
/// Ising model Hamiltonian
/// - Models a system of spins interacting with each other and an external field
pub trait Hamiltonian<T> {
    /// Given a state, return its energy.
    fn energy(&self, state: &T) -> f64;
}
pub struct IsingHamiltonian {
    pub j: f64, // Coupling constant (J)
    pub h: f64, // External magnetic field (h)
}

impl Hamiltonian<Vec<i8>> for IsingHamiltonian {
    fn energy(&self, state: &Vec<i8>) -> f64 {
        let mut energy = 0.0;

        // Loop through the spins and calculate the Hamiltonian
        for i in 0..state.len() - 1 {
            energy -= self.j * state[i] as f64 * state[i + 1] as f64; // Interaction with the next spin
        }

        // Add the external field interaction
        for spin in state {
            energy -= self.h * *spin as f64; // Interaction with the external magnetic field
        }

        energy
    }
}
