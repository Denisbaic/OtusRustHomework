pub trait Reporter {
    fn create_report(&self) -> String;
}
