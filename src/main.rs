fn main() {
    let mut sausage:i32;
    sausage = add(957, 3);
    sausage = sausage + 2;
    let ages= vec![9,1,1,8,10,58,48];
    all_5_friends(1000);
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
fn is_divided_by_3(i:i32) -> bool {
    i % 3 == 0
}
fn is_divided_by_5(i:i32) -> bool {
    i % 5 == 0
}
fn is_divided_by_15(i:i32) -> bool {
    i % 15 == 0
}

fn fizzbuzz(){
        for i in 1..17265 {
            if is_divided_by_15(i) {
                println!("fizzbuzz");
            } else if is_divided_by_5(i) {
                println!("buzz");
            } else if is_divided_by_3(i) {
                println!("fizz");
            } else {
                println!("{}", i);
            }
        }
}
fn is_divided_by_2(i:i32) -> bool {
    i % 2 == 0
}
fn pari_dispari(){
    for i in 1..17265 {
        if is_divided_by_2(i) {
            println!("pari");
        } else {
            println!("dispari");
        }
    }
}
fn all_5_friends(n:i32){
    for i in 1..n+1 {
        if i % 10 == 0 {
            println!("{}: I am your best friend", i);
        } else if is_divided_by_5(i) {
            println!("{}: I am your friend", i);
        } else  {
            println!("{}: I am not your friend", i);
        }
    }
}