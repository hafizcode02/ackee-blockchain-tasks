fn data_type() {
    let string_type = "Solana";
    let float_type = 4.5;
    let boolean_type = true;
    let char_type = '*';

    println!("Winter School of {}", string_type);
    println!("Rating of the School is {}", float_type);
    println!("Like Rust {}", boolean_type);
    println!("Winter School of {}", char_type);
}

fn test_print() {
    let name = "Hafiz Caniago";
    let age = 21;

    println!("Your Name is {} and age is {}", name, age);
}

fn know_string_func() {
    let empty_string = String::new();
    println!("length is {}", empty_string.len());

    let content_string = String::from("HafigoBlockchain");
    println!("length is {}", content_string.len());
}

fn sum(x: u64, y: u64) -> u64 {
    return x + y;
}

fn running_loop() {
    let mut counter = 0;
    while counter <= 10 {
        counter = counter + 1;
        println!("Hello");
    }
}

fn condition() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}

// borrow checker
fn owner1(name: &String) {
    println!("Owner 1 using this name, {}", name);
}

fn owner2(name: &String) {
    println!("Owner 2 using this name, {}", name);
}

fn mutate(greet: &mut String) {
    greet.push_str("Hafiz");
}

fn mutate2(greet: &mut String) {
    greet.push_str(" Bjir");
}

// arr slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

// struct
struct Unit;

// a Tuple struct
struct Pair(i32, f32);

// a struct with fields
struct Point {
    x: f32,
    y: f32,
}

fn main() {
    println!("Hello, world!");

    data_type();
    test_print();
    know_string_func();

    condition();
    running_loop();

    let result = sum(10, 20);
    println!("Result is : {}", result);

    let owner = String::from("Hafiz");
    owner1(&owner);
    owner2(&owner); // borrow checking, must add & in front of the variable to be used.

    let mut greetings = String::from("Hi!, my name is ");
    mutate(&mut greetings);
    mutate2(&mut greetings);
    println!("{}", greetings);

    let numbers = [1, 2, 3, 4, 5];
    println!("first element of the slice {}", numbers[0]);
    println!("the slice has {} elements", numbers.len());

    println!("borrow the whole array as a slice");
    analyze_slice(&numbers[1..4]);

    // struct testing
    let point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates = ({}, {})", point.x, point.y);
}
