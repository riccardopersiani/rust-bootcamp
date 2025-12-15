fn main() {
    let greaten_than = |x: &i32| *x > 10;
    // let less_than = |x: &i32| *x < 20;

    let res = are_both_true(greaten_than, less_than, &15);
    println!("{res}");
}

fn less_than(x: &i32) -> bool {
    *x < 20
}

fn are_both_true<V>(f1: T, f2: U, item: &V) -> bool
where
    T: Fn(&V) -> bool,
    U: Fn(&V) -> bool,
{
    f1(item) && f2(item)
}
