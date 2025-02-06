use rand::Rng;

pub struct IsingModel {
    pub size: usize,
    pub temperature: f64,
    pub spins: Vec<Vec<i32>>,
}

impl IsingModel {
    pub fn new(size: usize, temperature: f64) -> Self {
        let mut rng = rand::rng();
        let spins = (0..size)
            .map(|_| {
                (0..size)
                    .map(|_| if rng.random_bool(0.5) { 1 } else { -1 })
                    .collect()
            })
            .collect();
        Self {
            size,
            temperature,
            spins,
        }
    }

    pub fn step(&mut self) {
        let mut rng = rand::rng();
        for _ in 0..self.size * self.size {
            let i = rng.random_range(0..self.size);
            let j = rng.random_range(0..self.size);
            let delta_e = 2
                * self.spins[i][j]
                * (self.spins[(i + 1) % self.size][j]
                    + self.spins[(i + self.size - 1) % self.size][j]
                    + self.spins[i][(j + 1) % self.size]
                    + self.spins[i][(j + self.size - 1) % self.size]);
            if delta_e <= 0 || rng.random_bool((-delta_e as f64 / self.temperature).exp()) {
                self.spins[i][j] = -self.spins[i][j];
            }
        }
    }

    pub fn magnetization(&self) -> f64 {
        self.spins.iter().flatten().map(|&s| s as f64).sum::<f64>() / (self.size * self.size) as f64
    }
}

pub fn ising_example() {
    let mut model = IsingModel::new(20, 2.0);
    for _ in 0..1000 {
        model.step();
    }
    println!("Magnetization: {}", model.magnetization());
}

#[test]
fn test_ising_model() {
    let mut model = IsingModel::new(20, 2.0);
    for _ in 0..1000 {
        model.step();
    }
    let magnetization = model.magnetization();
    assert!(magnetization.abs() <= 1.0, "Magnetization out of bounds");
}
