use bevy::reflect::Reflect;

#[derive(Debug, Reflect, Clone, Default)]
pub struct MinMaxCurrent<T> {
    min: T,
    max: T,
    current: T,
}

impl<T: PartialOrd + Clone> MinMaxCurrent<T> {
    pub fn new(min: T, max: T, current: T) -> Self {
        assert!(min <= max);
        let mut min_max_current = MinMaxCurrent { min, max, current };
        min_max_current.clamp_to_borders();
        min_max_current
    }

    pub fn get_current_ref(&self) -> &T {
        &self.current
    }

    pub fn get_current_copy(&self) -> T {
        self.current.clone()
    }

    pub fn set_current(&mut self, current: T) {
        self.current = current;
        self.clamp_to_borders();
    }

    pub fn is_current_equal_to_max(&self) -> bool {
        self.current == self.max
    }

    pub fn is_current_equal_to_min(&self) -> bool {
        self.current == self.min
    }

    fn clamp_to_borders(&mut self) {
        if self.current < self.min {
            self.current = self.min.clone();
        }
        if self.current > self.max {
            self.current = self.max.clone();
        }
    }

    pub fn get_max_ref(&self) -> &T {
        &self.max
    }

    pub fn get_max_copy(&self) -> T {
        self.max.clone()
    }

    pub fn get_min_ref(&self) -> &T {
        &self.min
    }

    pub fn get_min_copy(&self) -> T {
        self.min.clone()
    }
}
