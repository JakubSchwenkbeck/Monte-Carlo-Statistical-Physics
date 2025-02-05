use crate::algorithms::distributions::ProbabilityDistribution;
use crate::algorithms::mc_estimator::MonteCarloEstimator;
use crate::algorithms::stochastic_matrix::StochasticMatrix;

pub fn web_graph() {
    let web_graph = vec![
        vec![1, 2], // Page 0 links to pages 1 and 2
        vec![2],    // Page 1 links to page 2
        vec![0],    // Page 2 links to page 0
    ];

    let num_simulations = 100_000;
    let num_steps = 100;

    let ranks = pagerank(web_graph, num_simulations, num_steps);
    println!("PageRanks: {:?}", ranks.values);
}

fn pagerank(
    web_graph: Vec<Vec<usize>>,
    num_simulations: usize,
    num_steps: usize,
) -> ProbabilityDistribution {
    let num_pages = web_graph.len();
    let stochastic_matrix = build_stochastic_matrix(&web_graph, num_pages);
    let initial_distribution =
        ProbabilityDistribution::new(vec![1.0 / num_pages as f64; num_pages]);

    let estimator = MonteCarloEstimator::new(stochastic_matrix);
    estimator.estimate(&initial_distribution, num_steps, num_simulations)
}
fn build_stochastic_matrix(graph: &[Vec<usize>], num_pages: usize) -> StochasticMatrix {
    let mut matrix = vec![vec![0.0; num_pages]; num_pages];

    for (i, links) in graph.iter().enumerate() {
        if links.is_empty() {
            // Handle dead ends by linking to all pages
            for j in 0..num_pages {
                matrix[i][j] = 1.0 / num_pages as f64;
            }
        } else {
            let prob = 1.0 / links.len() as f64;
            for &j in links {
                matrix[i][j] = prob;
            }
        }
    }

    StochasticMatrix::new(matrix)
}
