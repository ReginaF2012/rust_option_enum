enum Option<T> {
    //<T> means the Some variant of the Option enum can hold one piece of data of any type.
    Some(T),
    None,
}

let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;

fn main() {
    println!("Hello, world!");
}
