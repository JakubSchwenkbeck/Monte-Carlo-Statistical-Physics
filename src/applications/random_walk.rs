use rand::Rng;

pub struct RandomWalk {
    pub steps: usize,
    pub position: i32,
}

impl RandomWalk {
    pub fn new(steps: usize) -> Self {
        Self { steps, position: 0 }
    }

    pub fn simulate(&mut self) {
        let mut rng = rand::rng();
        for _ in 0..self.steps {
            if rng.random_bool(0.5) {
                self.position += 1;
            } else {
                self.position -= 1;
            }
        }
    }

    pub fn get_position(&self) -> i32 {
        self.position
    }
}

pub fn random_walk_example() {
    let mut walk = RandomWalk::new(1000);
    walk.simulate();
    println!("Final position after 1000 steps: {}", walk.get_position());
}

#[test]
fn test_random_walk() {
    let mut walk = RandomWalk::new(1000);
    walk.simulate();
    let position = walk.get_position();
    assert!(position.abs() <= 1000, "Final position out of bounds");
}
