use rand::Rng;

pub struct IsingModel {
    pub temperature: f64,
    pub j: f64,
    pub h: f64,
    pub grid: Vec<Vec<i8>>,
}

impl IsingModel {
    pub fn new(temperature: f64, j: f64, h: f64, grid_size: usize) -> Self {
        let grid = vec![vec![1; grid_size]; grid_size]; // All spins start as +1
        Self {
            temperature,
            j,
            h,
            grid,
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::rng();
        let n = self.grid.len();
        let m = self.grid[0].len();

        // Randomly select a spin
        let i = rng.random_range(0..n);
        let j = rng.random_range(0..m);

        let current_spin = self.grid[i][j];
        let neighbor_sum = self.get_neighbor_sum(i, j);

        let energy_diff = 2.0 * current_spin as f64 * neighbor_sum as f64 * self.j
            + 2.0 * self.h * current_spin as f64;

        // Decide whether to flip the spin
        if energy_diff <= 0.0 || rng.random::<f64>() < (-energy_diff / self.temperature).exp() {
            self.grid[i][j] = -current_spin; // Flip the spin
        }
    }

    fn get_neighbor_sum(&self, i: usize, j: usize) -> i8 {
        let n = self.grid.len();
        let m = self.grid[0].len();

        let mut sum = 0;
        sum += self.grid[(i + 1) % n][j]; // Right
        sum += self.grid[(i + n - 1) % n][j]; // Left
        sum += self.grid[i][(j + 1) % m]; // Up
        sum += self.grid[i][(j + m - 1) % m]; // Down

        sum
    }
}

pub fn ising_example() {
    let temperature = 2.0;
    let j = 1.0;
    let h = 0.0;
    let grid_size = 10;

    let mut ising_model = IsingModel::new(temperature, j, h, grid_size);

    // Run Metropolis updates and print grid
    for _ in 0..10000 {
        ising_model.update(); // Update grid using Metropolis
    }

    // Print final state of the grid
    println!("{:?}", ising_model.grid);
}
