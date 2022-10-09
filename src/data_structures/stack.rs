pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::stack::Stack;

    #[test]
    fn normal() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());

        stack.push(10);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.is_empty(), false);

        stack.push(20);
        stack.push(30);

        let mut val = stack.pop();
        assert_eq!(val, Some(30));

        val = stack.pop();
        assert_eq!(val, Some(20));

        stack.pop();
        val = stack.pop();
        assert_eq!(val, None);
        assert_eq!(stack.is_empty(), true);
    }
}
