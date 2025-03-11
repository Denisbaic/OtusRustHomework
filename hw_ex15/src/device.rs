use crate::reporter::Reporter;

pub trait Device: Reporter {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn toggle(&mut self) {
        if self.is_on() {
            self.turn_off();
        } else {
            self.turn_on();
        }
    }
    fn is_on(&self) -> bool;
    fn is_off(&self) -> bool;
    fn get_device_name(&self) -> &str;
}
