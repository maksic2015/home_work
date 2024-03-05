fn main() {
    let mut sausage:i32;
    sausage = add(957, 3);
    sausage = sausage + 2;
    println!("Hello, max! {}", is_sausage(String::from("sausage")));
}
fn add(x: i32, y:i32) -> i32 {
    x+y
}
fn multi(x: i32, y:i32) -> i32 {
    x*y
}
fn less(x: i32, y:i32) -> bool {
    if x < y { true } else { false }
}
fn less_or_equal(x: i32, y:i32) -> bool {
    if x <= y { true } else { false }
}
fn is_sausage(name: String) -> bool {
    if name == "sausage" { true } else { false }
}