use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T,
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> MySmartPointer<T> {
        MySmartPointer { value }
    }
}

// taget is gonna be whatever is stored in value so when deref is called, we simple return
// a reference to self.value
impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

// return a mut reference.
impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

// When we create an instance of MySmartPointer we are passing in a Box that holds an owned string
//
fn main() {
    let mut s = MySmartPointer::new(Box::new("Heap string".to_owned()));

    // &MySmartPointer -> &Box ->(coherced) &String -> &str

    // derefs are resolved at compile time
    let s1: &MySmartPointer<Box<String>> = &s;
    let s2: &Box<String> = &(*s);
    let s3: &String = &(**s);
    let s4: &str = &(***s);
    print(&mut s);
}

fn print(s: &str) {
    println!("{}", s);
}

// Deref says how the deref works
// Target is associated type like a generic. In this case target can be anything that increments the Sized Trait.
// Types with a constant size known at compile time.
// All type parameters have an implicit bound of Sized.
// ?Sized, can be used to remove this bound! Special Syntax.
