use core::cell::RefCell;

use alloc::rc::Rc;

pub struct Splitter<T, U>
where
    T: FnMut(&U::Item) -> bool,
    U: Iterator,
{
    splitter: Rc<RefCell<T>>,
    iterator: Rc<RefCell<core::iter::Peekable<U>>>,
}

impl<T, U> Splitter<T, U>
where
    T: FnMut(&U::Item) -> bool,
    U: Iterator,
{
    pub fn new(splitter: T, iterator: U) -> Self {
        Self {
            splitter: Rc::new(RefCell::new(splitter)),
            iterator: Rc::new(RefCell::new(iterator.peekable())),
        }
    }
}

impl<T, U> Iterator for Splitter<T, U>
where
    T: FnMut(&U::Item) -> bool,
    U: Iterator,
{
    type Item = Splitted<T, U>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iterator.borrow_mut().peek() {
                Some(item) => {
                    if (self.splitter.borrow_mut())(&item) {
                        return Some(Splitted {
                            splitter: Rc::clone(&self.splitter),
                            iterator: Rc::clone(&self.iterator),
                        });
                    }
                }
                None => return None,
            }
        }
    }
}

pub struct Splitted<T, U>
where
    T: FnMut(&U::Item) -> bool,
    U: Iterator,
{
    splitter: Rc<RefCell<T>>,
    iterator: Rc<RefCell<core::iter::Peekable<U>>>,
}

impl<T, U> Iterator for Splitted<T, U>
where
    T: FnMut(&U::Item) -> bool,
    U: Iterator,
{
    type Item = U::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.borrow_mut().next() {
            Some(item) => {
                if (self.splitter.borrow_mut())(&item) {
                    Some(item)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
