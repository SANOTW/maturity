use maturity_macro::maturity;

#[maturity]
pub struct MetricCount {
    total: u64,
}

impl MetricCount {
    pub fn total(&self) -> u64 {
        self.total
    }
}
