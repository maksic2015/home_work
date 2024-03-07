fn main() {
    let mut sausage:i32;
    sausage = add(957, 3);
    sausage = sausage + 2;
    let ages= vec![9,1,1,8,10,58,48];
    print_fibonacci(11);
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
fn sum(items:Vec<i32>) -> i32 {
    let mut result:i32 = 0;
    for item in items.iter() {
        result= result+item;
        println!("{}", result);
    }
    result
}

fn print_fibonacci(n:i32) {
    for i in 0..n {
        println!("{}", fibonacci_number(i));
    }
}

fn fibonacci_number(i:i32) -> i32 {
    return match i {
        0 => 0,
        1 => 1,
        _ => fibonacci_number(i-1) + fibonacci_number(i-2)
    }
}
fn minus(x: i32, y:i32) -> i32 {
    x-y
}