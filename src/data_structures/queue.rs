use std::collections::LinkedList;

pub struct Queue<T> {
    data: LinkedList<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            data: LinkedList::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn push(&mut self, item: T) {
        self.data.push_back(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::queue::Queue;

    #[test]
    fn normal() {
        let mut queue = Queue::new();
        assert!(queue.is_empty());

        queue.push(10);
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.is_empty(), false);

        queue.push(20);
        queue.push(30);

        let mut val = queue.pop();
        assert_eq!(val, Some(10));

        val = queue.pop();
        assert_eq!(val, Some(20));

        val = queue.pop();
        assert_eq!(val, Some(30));
        val = queue.pop();
        assert_eq!(val, None);
        assert_eq!(queue.is_empty(), true);
    }
}
