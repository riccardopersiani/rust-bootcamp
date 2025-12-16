// Make the code compile by implementing the Iterator trait for `Queue`.

struct Queue {
    items: Vec<i32>,
}

struct Person {
    first_name: String,
    last_name: String,
    occupation: String,
}

struct PersonIterator {
    values: Vec<String>,
}

impl Iterator for PersonIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.values.is_empty() {
            return None;
        }
        Some(self.values.remove(0))
    }
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

impl IntoIterator for Person {
    type Item = String;
    //type IntoIter = PersonIterator;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // PersonIterator {
        //     values: vec![self.first_name, self.last_name, self.occupation],
        // }
        vec![self.first_name, self.last_name, self.occupation].into_iter()
    }
}

fn main() {
    let mut queue = Queue::new(vec![3, 2, 1]);
    assert!(matches!(queue.next(), Some(3)));
    assert!(matches!(queue.next(), Some(2)));
    assert!(matches!(queue.next(), Some(1)));
    assert!(matches!(queue.next(), None));
    let p = Person {
        first_name: "Riccardo".to_owned(),
        last_name: "Persiani".to_owned(),
        occupation: "Software Engineer".to_owned(),
    };

    // for loop know how to handle iterator
    // will print only the string but not None.
    for item in p {
        println!("{item}");
    }
}
