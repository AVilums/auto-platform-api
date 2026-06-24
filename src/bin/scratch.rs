// src/bin/scratch.rs
// Playground file. Run with cargo run --bin scratch

fn main() {
    variables();
    control_flow(10);
    ownership_borrowing();
    struct_enums();
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