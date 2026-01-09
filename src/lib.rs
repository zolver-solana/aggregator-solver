#[derive(Debug)]
pub struct Solver {
    name: String,
    version: String,
}

impl Solver {
    pub fn new(name: &str, version: &str) -> Self {
        Solver {
            name: name.to_string(),
            version: version.to_string(),
        }
    }

    pub fn solve(&self, intent: &str) -> String {
        format!("Solving '{}' using {} v{}", intent, self.name, self.version)
    }
}
