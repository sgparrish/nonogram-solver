use nono_solver::Nonogram;
use std::time::Instant;

fn main() {
    let x_hints: Vec<Vec<u8>> = vec![
        vec![2, 1, 2],
        vec![1, 2, 3, 1],
        vec![2, 1, 5],
        vec![2, 2, 4],
        vec![5, 2],
        vec![1, 1, 2],
        vec![1, 2, 4, 2],
        vec![8, 1, 2],
        vec![7, 4],
        vec![7, 3],
        vec![3, 8],
        vec![3, 6],
        vec![4, 6],
        vec![11],
        vec![7],
    ];

    let y_hints: Vec<Vec<u8>> = vec![
        vec![9],
        vec![5, 7],
        vec![1, 1, 1, 8],
        vec![7, 3],
        vec![4, 3, 3],
        vec![2, 4, 2],
        vec![5, 2],
        vec![2, 2, 2],
        vec![1, 2, 2],
        vec![2, 1, 5],
        vec![3, 4],
        vec![3, 6],
        vec![11],
        vec![9, 1],
        vec![3, 1, 1],
    ];

    let mut ng = Nonogram::new(x_hints, y_hints);

    let now = Instant::now();
    {
        ng.solve(30 * 15);
    }

    let elapsed = now.elapsed();
    println!("{}", ng);
    println!("Solved in: {:.2?}", elapsed);
}
