struct Tweet<'a> {
    content: &'a str,
}

impl<'a> Tweet<'a> {
    fn change_content(&mut self, new_content: &'a str) {
        self.content = new_content;
    }
}

fn main() {
    println!("Hello, world!");
    let mut tweet = Tweet { content: "ciao" };
    println!("Content: {}", tweet.content);
    tweet.change_content("bella");
    println!("Content: {}", tweet.content);
}

// We don't need to add lifetime annotation, because the Rust compiler follow the 3 lifetime elision rules:
// After applying the 3 rules and the lifetime is still ambiguous it will throw an error and will require explicit lifetime
//
// Lifetime of function or method parameters are called "input lifetimes"
// Lifetimes of return values are called "output lifetimes"
//
// 1. each parameter that is a reference gets its own lifetime parameter;
// 2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters;
// 3. (applies to methods only?) If there are multiple inputs lifetime parameters, but one of them is &self or &mut self,
// the lifetime of self is assigned to all output lifetime parameters.
//
// these ' remeber that describe the relation not concrete lifetimes
fn take_and_return_content<'a>(content: &'a str, content2: &'a str) -> &'a str {
    content
}
