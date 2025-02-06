use rand::Rng;

pub struct Percolation {
    pub size: usize,
    pub grid: Vec<Vec<bool>>,
}

impl Percolation {
    pub fn new(size: usize) -> Self {
        let grid = vec![vec![false; size]; size];
        Self { size, grid }
    }

    pub fn open(&mut self, row: usize, col: usize) {
        self.grid[row][col] = true;
    }

    pub fn is_open(&self, row: usize, col: usize) -> bool {
        self.grid[row][col]
    }

    pub fn percolates(&self) -> bool {
        let mut visited = vec![vec![false; self.size]; self.size];
        for col in 0..self.size {
            if self.dfs(0, col, &mut visited) {
                return true;
            }
        }
        false
    }

    fn dfs(&self, row: usize, col: usize, visited: &mut Vec<Vec<bool>>) -> bool {
        if row >= self.size || col >= self.size || visited[row][col] || !self.is_open(row, col) {
            return false;
        }
        visited[row][col] = true;
        if row == self.size - 1 {
            return true;
        }
        let directions = [(1, 0), (0, 1), (0, -1), (-1, 0)];
        for &(dr, dc) in &directions {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;
            if new_row >= 0
                && new_row < self.size as isize
                && new_col >= 0
                && new_col < self.size as isize
                && self.dfs(new_row as usize, new_col as usize, visited)
            {
                return true;
            }
        }
        false
    }
}

pub fn percolation_example() {
    let mut percolation = Percolation::new(20);
    let mut rng = rand::rng();
    for _ in 0..400 {
        let row = rng.random_range(0..20);
        let col = rng.random_range(0..20);
        percolation.open(row, col);
    }
    println!("Percolates: {}", percolation.percolates());
}

#[test]
fn test_no_percolation() {
    let mut percolation = Percolation::new(20);
    for i in 0..19 {
        percolation.open(i, i);
    }
    assert!(!percolation.percolates(), "The system should not percolate");
}

#[test]
fn test_percolation_with_multiple_paths() {
    let mut percolation = Percolation::new(20);
    for i in 0..20 {
        percolation.open(i, 0);
        percolation.open(i, 1);
    }
    assert!(percolation.percolates(), "The system should percolate");
}
