// Make the code compile by implementing the Iterator trait for `Queue`.

struct Queue {
    items: Vec<i32>,
}

impl Queue {
    fn new(items: Vec<i32>) -> Self {
        Self { items }
    }
}

impl Iterator for Queue {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        // self.items[];
        if self.items.len() > 0 {
            Some(self.items.remove(0))
        } else {
            None
        }
    }
}

fn main() {
    let mut queue = Queue::new(vec![3, 2, 1]);
    assert!(matches!(queue.next(), Some(3)));
    assert!(matches!(queue.next(), Some(2)));
    assert!(matches!(queue.next(), Some(1)));
    assert!(matches!(queue.next(), None));
}
