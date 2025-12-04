trait UIComponent {
    fn render(&self) {
        println!("");
    }
}

struct Button {
    text: String,
}

impl UIComponent for Button {}

struct Container {
    name: String,
    child: Box<Container>,
}

impl UIComponent for Container {}

fn main() {
    println!("Hello, world!");
    let button_a = Button {
        text: "b".to_owned(),
    };
    // like unique_ptr in c++
    // gives single ownership on something stored on the heap
    // 1. use case: avoid to copy large amount of data when transferring ownership
    let button_b = Box::new(Button {
        text: "b".to_owned(),
    });

    // here entire button is copied
    let button_c = button_a;
    // here only the box smart contract is copied cause the rest is stored on the heap
    let button_d = button_b;

    let components: Vec<Box<dyn UIComponent>> = vec![Box::new(button_c), button_d];
}
