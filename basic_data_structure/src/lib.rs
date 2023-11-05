use std::rc::Rc;

pub struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

pub struct List<T> {
    size: usize,
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { size: 0, head: None, tail: None }
    }
    pub fn is_empty(&self) -> bool {
        0 == self.size
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn add(&mut self, elem: T) {
        let flag =
            if self.tail != None && self.head == None { 1 } else { 0 };
        let node = Box::new(Node {
            elem,
            next: self.tail.take(),
        });
        self.tail = Some(node);
        self.size += 1;
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
