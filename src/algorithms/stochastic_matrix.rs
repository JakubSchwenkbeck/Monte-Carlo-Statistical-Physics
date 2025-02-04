use crate::algorithms::distributions::ProbabilityDistribution;

pub struct StochasticMatrix {
    pub matrix: Vec<Vec<f64>>,
}

impl StochasticMatrix {
    pub fn new(matrix: Vec<Vec<f64>>) -> Self {
        assert!(
            matrix.iter().all(|row| {
                row.iter().all(|&x| x >= 0.0) && (row.iter().sum::<f64>() - 1.0).abs() < 1e-9
            }),
            "Each row must be a valid probability distribution"
        );

        Self { matrix }
    }

    pub fn multiply_with_distribution(
        &self,
        distribution: &ProbabilityDistribution,
    ) -> ProbabilityDistribution {
        let num_states = distribution.values.len();
        let num_columns = self.matrix[0].len();
        let new_values: Vec<f64> = (0..num_columns)
            .map(|j| {
                (0..num_states)
                    .map(|i| self.matrix[i][j] * distribution.values[i])
                    .sum()
            })
            .collect();
        ProbabilityDistribution::new(new_values)
    }

    pub fn find_stationary_distribution(&self, iterations: usize) -> ProbabilityDistribution {
        let mut distribution =
            ProbabilityDistribution::new(vec![1.0 / self.matrix.len() as f64; self.matrix.len()]);

        for _ in 0..iterations {
            distribution = self.multiply_with_distribution(&distribution);
        }

        distribution
    }
}
