// Static Composite

trait Valuable<T> {
    fn get_value(&self) -> T;
}

struct GoldenEgg {
    value: u32,
}

impl GoldenEgg {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl Valuable<u32> for GoldenEgg {
    fn get_value(&self) -> u32 {
        self.value
    }
}

struct CustomCart {
    goods: Vec<Box<dyn Valuable<u32>>>,
}

impl CustomCart {
    pub fn new(goods: Vec<Box<dyn Valuable<u32>>>) -> Self {
        Self { goods }
    }
}

impl Valuable<u32> for CustomCart {
    fn get_value(&self) -> u32 {
        self.goods.iter().map(|x| x.get_value()).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::composite::{GoldenEgg, Valuable};

    use super::CustomCart;

    #[test]
    fn test_composite() {
        let goods: Vec<Box<dyn Valuable<u32>>> = vec![
            Box::new(GoldenEgg::new(10)),
            Box::new(GoldenEgg::new(20)),
            Box::new(GoldenEgg::new(30)),
        ];
        let cart1 = CustomCart::new(goods);

        let cart2 = CustomCart::new(vec![Box::new(GoldenEgg::new(10)), Box::new(cart1)]);
        assert_eq!(cart2.get_value(), 70);
    }
}
