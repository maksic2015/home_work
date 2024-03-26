fn main() {
    let mut sausage:i32;
    sausage = add(957, 3);
    sausage = sausage + 2;
    // let ages= vec![9,1,1,8,10,58,48,];
    print_power_of_5(11);

    let r1 = Rectangle{
        h:5,
        w:2,
    };
    let r2 = Rectangle{
        h:10,
        w:30,
    };

    println!("Rectangle Area: {} {}", r1.area(), r2.area());

    let mut maxic = Person{
        name:"max".to_string(),
        money:0,
        age:8
    };
    maxic.hello();

    maxic.add_money(100000);
    maxic.add_money(100000);
    maxic.add_money(100000);
    println!("Maxic rich {}", maxic.money)
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
fn all_6_friends(n:i32){
    for i in 1..n+1 {
        if i % 36 == 0 {
            println!("{}: I am your best friend", i);
        } else if i % 6 == 0{
            println!("{}: I am your friend", i);
        } else  {
            println!("{}: I am not your friend", i);
        }
    }
}
fn max(items:Vec<i32>) -> i32 {
    let mut result:i32 = items[0];
    for item in items.iter() {
        if result < *item {
            result = *item
        }
    }
    result
}
fn min(items:Vec<i32>) -> i32 {
    let mut result:i32 = items[0];
    for item in items.iter() {
        if result > *item {
            result = *item
        }
    }
    result
}
fn print_power_of_2(n:i32) {
    for i in 0..n {
        println!("{}", power_of_2(i));
    }
}

fn power_of_2(i:i32) -> i32 {
    return match i {
        0 => 1,
        1 => 2,
        _ => power_of_2(i-1)*2
    }
}
fn print_power_of_5(n:i32) {
    for i in 0..n {
        println!("{}", power_of_5(i));
    }
}

fn power_of_5(n:i32) -> i32 {
    return match n {
        0 => 1,
        1 => 5,
        _ => 5 * power_of_5(n -1)
    }
}
fn magic_number(n: i32) -> i32 {
    if n % 2 == 0 { n * 2} else { n+ 1 }
}

struct Rectangle {
    h:i32,
    w:i32
}

impl Rectangle {
    fn area(&self) -> i32{
        self.h*self.w
    }
}

struct Person {
    name: String,
    money: u32,
    age: u8
}

impl Person {
    fn hello(&self) {
        println!("Hello {}", self.name)
    }
    fn add_money(&mut self, amount: u32) {
        self.money = self.money + amount
    }
}