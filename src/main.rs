use aggregator_solver::Solver;

fn main() {
    let solver = Solver::new("AggregatorSolver", "1.0");
    let result = solver.solve("Example Intent");
    println!("{}", result);
}
