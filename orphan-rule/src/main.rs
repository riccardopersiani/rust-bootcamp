use orphan_rule::Point;

//TUple struct that wraps the Point struct
struct PointWrapper(Point);

// we are braking rule, ordre to implement a trait on given type, it needs to be defined in the same crate so that rust know what rust should use.
// but you can get around with a wrapper type
impl PartialEq for PointWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}

fn main() {
    let p1 = PointWrapper(Point { x: 1, y: 2 });
    let p2 = PointWrapper(Point { x: 1, y: 2 });
    let p3 = PointWrapper(Point { x: 1, y: 2 });
    println!("{}", p1 == p2);
}
