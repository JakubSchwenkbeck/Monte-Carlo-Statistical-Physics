use rand::Rng;

pub fn estimate_pi(samples: usize) -> f64 {
    let mut rng = rand::rng();
    let mut inside_circle = 0;

    for _ in 0..samples {
        let x: f64 = rng.random_range(-1.0..1.0);
        let y: f64 = rng.random_range(-1.0..1.0);
        if x * x + y * y <= 1.0 {
            inside_circle += 1;
        }
    }

    4.0 * (inside_circle as f64) / (samples as f64)
}
