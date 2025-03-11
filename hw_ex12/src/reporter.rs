pub trait Reporter {
    fn create_report(&self) -> Result<String, Box<dyn std::error::Error>>;
}
