use rand::Rng;

pub struct BrownianMotion {
    pub num_particles: usize,
    pub num_steps: usize,
    pub step_size: f64,
    pub positions: Vec<(f64, f64)>,
}

impl BrownianMotion {
    pub fn new(num_particles: usize, num_steps: usize, step_size: f64) -> Self {
        let positions = vec![(0.0, 0.0); num_particles];
        Self {
            num_particles,
            num_steps,
            step_size,
            positions,
        }
    }

    pub fn simulate(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..self.num_steps {
            for i in 0..self.num_particles {
                let angle = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
                let dx = self.step_size * angle.cos();
                let dy = self.step_size * angle.sin();
                self.positions[i].0 += dx;
                self.positions[i].1 += dy;
            }
        }
    }

    pub fn get_positions(&self) -> &[(f64, f64)] {
        &self.positions
    }
}

pub fn brownian_motion_example() {
    let mut bm = BrownianMotion::new(100, 1000, 1.0);
    bm.simulate();
    for (i, pos) in bm.get_positions().iter().enumerate() {
        println!("Particle {}: ({}, {})", i, pos.0, pos.1);
    }
}
#[test]
fn test_brownian_motion() {
    let mut bm = BrownianMotion::new(100, 1000, 1.0);
    bm.simulate();
    let positions = bm.get_positions();

    // Check that the number of positions matches the number of particles
    assert_eq!(
        positions.len(),
        100,
        "Number of positions does not match number of particles"
    );

    // Check that the positions are not all at the origin
    let all_at_origin = positions.iter().all(|&(x, y)| x == 0.0 && y == 0.0);
    assert!(!all_at_origin, "All particles are still at the origin");
}
