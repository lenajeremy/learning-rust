/// A Queue data structure. It follows the FIFO (first in, first out) order. Used extensively
/// in algorithms throughout computer science.
#[derive(Debug)]
pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    /// Initializes an empty queue;
    pub fn new() -> Self {
        Queue { items: vec![] }
    }

    /// Initializes a queue from a list of vectors
    pub fn from(items: Vec<T>) -> Self {
        Queue { items }
    }

    /// Add a value to the end of the queue.
    pub fn enqueue(&mut self, val: T) -> usize {
        self.items.push(val);
        self.items.len()
    }

    /// Remove a value from the end of the queue.
    pub fn dequeue(&mut self) -> Option<T> {
        if self.items.len() == 0 {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    /// Returns a boolean to indicate whether or not the queue is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_queue() {
        let q: Queue<i32> = Queue::new();
        assert!(q.items.len() == 0)
    }

    #[test]
    fn test_create_queue_from_vector() {
        let q = Queue::from(vec![1, 2, 3, 4, 5]);
        assert!(q.items.len() == 5);
        assert_eq!(q.items, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_enqueing() {
        let mut q = Queue::from(vec![1, 2, 3, 4, 5]);
        q.enqueue(50);
        q.enqueue(500);

        assert!(q.items.len() == 7);
        assert_eq!(q.items, vec![1, 2, 3, 4, 5, 50, 500]);
    }

    #[test]
    fn test_dequeing() {
        let mut q = Queue::from(vec![1, 2, 3, 4, 5]);
        let first = q
            .dequeue()
            .unwrap_or_else(|| panic!("dequeing an empty list"));
        let second = q
            .dequeue()
            .unwrap_or_else(|| panic!("dequeing an empty list"));

        assert!(q.items.len() == 3);
        assert_eq!(q.items, vec![3, 4, 5]);
        assert_eq!(first, 1);
        assert_eq!(second, 2)
    }

    #[test]
    #[should_panic]
    fn test_dequeing_an_empty_queue() {
        let mut q: Queue<i32> = Queue::new();
        let first = q.dequeue();
        let second = q.dequeue();

        assert!(q.items.len() == 3);
        assert_eq!(q.items, vec![3, 4, 5]);
        assert_eq!(first, None);
        assert_eq!(second, None)
    }
}
