use rand::prelude::*;

// Static Visitor

#[derive(Clone, Copy, PartialEq, Eq)]
struct BookPage(u32);

struct Book {
    pub pages: Vec<BookPage>,
}

impl Book {
    fn accept(&mut self, visitor: &mut impl Visitor) {
        visitor.do_book_work(self);
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Good(String);

struct CustomCart {
    pub goods: Vec<Good>,
}

impl CustomCart {
    fn accept(&mut self, visitor: &mut impl Visitor) {
        visitor.do_cart_work(self);
    }
}

trait Visitor {
    fn do_book_work(&self, book: &mut Book);

    fn do_cart_work(&self, cart: &mut CustomCart);
}

struct AddRandomElement;

impl Visitor for AddRandomElement {
    fn do_book_work(&self, book: &mut Book) {
        book.pages.push(BookPage(random()));
    }

    fn do_cart_work(&self, cart: &mut CustomCart) {
        cart.goods.push(Good(random::<u32>().to_string()));
    }
}

struct AddElement(BookPage, Good);

impl AddElement {
    fn new(page_to_add: BookPage, good: Good) -> Self {
        Self(page_to_add, good)
    }
}

impl Visitor for AddElement {
    fn do_book_work(&self, book: &mut Book) {
        let AddElement(page, _) = self;
        book.pages.push(page.clone());
    }

    fn do_cart_work(&self, cart: &mut CustomCart) {
        let AddElement(_, good) = self;
        cart.goods.push(good.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut book = Book { pages: vec![] };
        let mut cart = CustomCart { goods: vec![] };

        let mut add_random_element = AddRandomElement;

        book.accept(&mut add_random_element);
        cart.accept(&mut add_random_element);

        let mut add_element = AddElement(BookPage(1), Good("1".to_string()));

        book.accept(&mut add_element);
        cart.accept(&mut add_element);

        assert_eq!(book.pages.contains(&BookPage(1)), true);
        assert_eq!(cart.goods.contains(&Good("1".to_string())), true);
    }
}
