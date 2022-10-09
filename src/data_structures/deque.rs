use std::collections::LinkedList;

pub struct Deque<T> {
    data: LinkedList<T>,
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Deque {
            data: LinkedList::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn push_back(&mut self, item: T) {
        self.data.push_back(item);
    }

    pub fn push_front(&mut self, item: T) {
        self.data.push_front(item);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Deque::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::deque::Deque;

    #[test]
    fn normal() {
        let mut deque = Deque::new();
        assert!(deque.is_empty());

        deque.push_front(10);
        assert_eq!(deque.len(), 1);
        assert_eq!(deque.is_empty(), false);

        deque.push_back(20);
        deque.push_front(30);

        let mut val = deque.pop_back();
        assert_eq!(val, Some(20));

        val = deque.pop_front();
        assert_eq!(val, Some(30));

        val = deque.pop_front();
        assert_eq!(val, Some(10));
        val = deque.pop_back();
        assert_eq!(val, None);
        val = deque.pop_back();
        assert_eq!(val, None);
        assert_eq!(deque.is_empty(), true);
    }
}
