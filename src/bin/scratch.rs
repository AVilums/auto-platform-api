// src/bin/scratch.rs
// Playground file. Run with cargo run --bin scratch

fn main() {
    variables();
    control_flow(10);
    ownership_borrowing();
    struct_enums();
    pattern_matching();
    closures_iterations();
    error_handling();
    strings();
    collections();
}

// variables
fn variables() {
    println!("\n --- Variables ---");

    let immutable = 42; // immutable by default, can't change unless reassigned
    let mut mutable = 0; // opt-in mutability
    mutable += 1;

    // shadowing - same name, new binding and type
    let shadowed = "hello";
    let shadowed = shadowed.len();

    const MAX: u32 = 100; // compile-time const, must be typed
    println!("immutable={immutable}, mutable={mutable}, shadowed={shadowed}, MAX={MAX}");

    return;
}

// control flow (logic gates)
fn control_flow(x: u32) {
    println!("\n --- Control Flow ---");

    // if v1
    let label = if x % 2 == 0 { "even" } else { "odd" };
    println!("If v1. x is {}", label);

    // if v2
    let label_2: &str;
    if x % 2 == 0 { label_2 = "even"; } else { label_2 = "odd";  }
    println!("If v2. x is {}", label_2);

    // loop v1
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };
    println!("Loop v1. result={result}");

    // loop v2
    let mut counter = 0;
    let result_v2: u32;
    loop {
        counter += 1;
        if counter == 5 {
            result_v2 = counter * 2;
            break;
        }
    }
    println!("Loop v2. result={result_v2}");

    // while v1
    let mut n = 3;
    while n > 0 {
        print!("{n} ");
        n -= 1;
    }
    println!("while v1 done");

    // for v1
    for i in 0..4 {
        println!("{i}")
    }

    return;
}

// ownership and borrowing
fn ownership_borrowing() {

    fn calculate_length(s: &String) -> usize { return s.len(); }

    fn change(s: &mut String) -> () {
        s.push_str(", world");
        return;
    }

    println!("\n --- Ownership and Borrowing ---");

    // Move - s1 no longer valid after this
    let s1 = String::from("hello");
    let s2 = s1; // s1 moved into s2
    // println!("s1={s1}, s2={s2}");  // error: use of moved value: `s1`

    // Clone - explicit deep copy
    let s3 = s2.clone();
    println!("s2={s2}, s3={s3}");

    // Immutable borrow - many readers at once
    let len = calculate_length(&s3);
    println!("s3={s3}, len={len}");

    // Mutable borrow - only on at a time
    let mut s4 = String::from("hello");
    change(&mut s4);
    println!("after change: s4={s4}");

    return;
}

// struct and enums
fn struct_enums() {
    println!("\n --- Structs and Enums ---");

    // struct with data
    #[derive(Debug)]
    struct Point {
        x: f64, y: f64
    }

    impl Point {
        fn new(x: f64, y: f64) -> Self {
            return Self { x, y };
        }

        fn distance_from_origin(&self) -> f64 {
            let km: f64 = (self.x.powi(2) + self.y.powi(2)).sqrt();
            return km;
        }
    }

    let p = Point::new(3.0, 4.0);
    println!("{p:?} - distance from origin: {}", p.distance_from_origin());

    // enum with data
    #[derive(Debug)]
    enum Shape {
        Circle(f64),
        Rectangle(f64, f64)
    }

    impl Shape {
        fn area(&self) -> f64 {
            match self {
                Shape::Circle(r) => std::f64::consts::PI * r * r,
                Shape::Rectangle(w, h) => w * h,
            }
        }
    }

    let shapes = [Shape::Circle(2.0), Shape::Rectangle(3.0, 4.0)];
    for s in &shapes {
        println!("{s:?} area = {:.2}", s.area());
    }

    let circl = Shape::Circle(4.0);
    println!("Circle struct {}", circl.area());

    return;
}

// pattern matching
fn pattern_matching() {
    println!("\n --- Pattern matching ---");

    // Option<T>
    let maybe: Option<i32> = Some(42);
    println!("{maybe:?} value is {}", maybe.unwrap());
    match maybe {
        Some(n) => println!("Found a number: {n}"),
        None => println!("No number found"),
    }

    // if let — shorter, when error is irrelevant
    if let Some(n) = maybe {
        println!("if-let: {n}");
    }

    // while let
    let mut stack = vec![1, 2, 3];
    while let Some(n) = stack.pop() {
        print!("{n} ");
    }
    println!();

    // Destructuring tuples
    let (a, b, c) = (1, "two", 3.0_f32);
    println!("tuple: {a}, {b}, {c}");

    return;
}

// closures and iterations
fn closures_iterations() {
    println!("\n --- Closure and Iterations ---");

    let numbers = vec![1, 2, 3, 4, 5, 6];

    let evens_squared: Vec<i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("even squared: {evens_squared:?}");

    let sum: i32 = numbers.iter().sum();
    println!("sum = {sum}");

    // closure capturing environment
    let threshold = 3;
    let above: Vec<_> = numbers.iter().filter(|&&x| x > threshold).collect();
    println!("above {threshold}: {above:?}");

    return;
}

// error handling
fn error_handling() {
    println!("\n --- Error handling ---");

    // Result<T, E>
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            return Err("Cannot divide by zero".to_string());
        } else {
            return Ok(a / b);
        }
    }

    match divide(10.0, 2.0) {
        Ok(v) => println!("10/2 = {v}"),
        Err(e) => println!("Error: {e}"),
    }

    match divide(5.0, 0.0) {
        Ok(v) => println!("5/0 = {v}"),
        Err(e) => println!("Error: {e}"),
    }

    // uwrap_or, map, and_then
    let result = divide(9.0, 3.0).unwrap_or(0.0);
    println!("unwrap_or: {result}");

    fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
        let n: i32 = s.parse()?;
        return Ok(n * 2);
    }

    println!("parse '21' * 2 = {:?}", parse_and_double("21"));
    println!("parse 'abc' * 2 = {:?}", parse_and_double("abc"));

    return;
}

// Strings
fn strings() {
    println!("\n --- Strings ---");

    // &str - string slice, immutable view, lives in the binary
    let s: &str = "hello, world";

    // String - heal allocated, growable
    let mut owned = String::from("Hello");
    owned.push_str(", world");
    owned.push('!');

    println!("&str: {s}");
    println!("String: {owned}");
    println!("uppercase: {}", owned.to_uppercase());
    println!("contains 'world': {}", owned.contains("world"));
    println!("replace: {}", owned.replace("world", "Rust"));

    // format! - cheapest way to concatenate without moves
    let combined = format!("{} + {}", s, owned);
    println!("combined: {combined}");

    return;
}

// collections
fn collections() {
    println!("\n --- Collections ---");

    // Vec<T>
    let mut v: Vec<i32> = Vec::new();
    v.extend([1, 2, 3]);
    v.push(4);
    println!("vec: {v:?}, len={}", v.len());

    // HashMap
    use std::collections::HashMap;
    let mut scores: HashMap<&str, u32> = HashMap::new();
    scores.insert("Alice", 10);
    scores.insert("Bob", 20);
    scores.entry("Alice").and_modify(|s| *s += 5); //updates existing
    scores.entry("Carol").or_insert(15); // insert if missing
    println!("scores: {:?}", scores);

    // HashSet
    use std::collections::HashSet;
    let a: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let b: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
    let intersection: Vec<_> = a.intersection(&b).collect();
    println!("a ∩ b = {:?}", intersection);

    return;
}