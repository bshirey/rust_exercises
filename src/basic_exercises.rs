use std::io;
use std::process::Command;

pub fn do_exercises() {
    println!("------- Basic Exercise #1 -------");
    exercise1();

    println!("------- Basic Exercise #2 -------");
    //exercise2();
    
    println!("------- Basic Exercise #3 -------");
    exercise3();
    
    println!("------- Basic Exercise #4 -------");
    exercise4();
    
    println!("------- Basic Exercise #5 -------");
    exercise5();
    
    println!("------- Basic Exercise #6 -------");
    //exercise6();
    
    println!("------- Basic Exercise #7 -------");
    exercise7();
    
    println!("------- Basic Exercise #8 -------");
    exercise8();
    
    println!("------- Basic Exercise #9 -------");
    exercise9();
    
    println!("------- Basic Exercise #10 -------");
    //exercise10();
    
    println!("------- Basic Exercise #11 -------");
    exercise11();
    
    println!("------- Basic Exercise #12 -------");
    exercise12();
    
    println!("------- Basic Exercise #13 -------");
    exercise13();
    
    println!("------- Basic Exercise #14 -------");
    exercise14();
    
    println!("------- Basic Exercise #15 -------");
    exercise15();
    
    println!("------- Basic Exercise #16 -------");
    exercise16();
    
    println!("------- Basic Exercise #17 -------");
    exercise17();
    
    println!("------- Basic Exercise #18 -------");
    exercise18();
    
    println!("------- Basic Exercise #19 -------");
    exercise19();
}

/// 1. Write a Rust program that gathers system information such as the Rust version, operating system details, and system architecture.
fn exercise1() {
    let version_output = Command::new("rustc").arg("--version").output().expect("Failed to exec rustc");
    let version = String::from_utf8_lossy(&version_output.stdout);
    println!("Rust version: {}", version.trim());

    let os_details_output = Command::new("uname").arg("-a").output().expect("Failed to exec uname -a");
    let os_details = String::from_utf8_lossy(&os_details_output.stdout);
    println!("OS Details: {}", os_details.trim());

    let arch_output = Command::new("uname").arg("-m").output().expect("Failed to exec uname -m");
    let arch = String::from_utf8_lossy(&arch_output.stdout);
    println!("Arch: {}", arch.trim());
}

/// 2. Write a Rust program that accepts two numbers from the user, adds them together, and displays the result.
#[allow(dead_code)]
fn exercise2() {
    println!("Enter the first number:");
    let mut first = String::new();
    io::stdin().read_line(&mut first).expect("Failed to read first number");
    let first_num: f64 = first.trim().parse().expect("Please enter a valid number");

    println!("Enter the second number:");
    let mut second = String::new();
    io::stdin().read_line(&mut second).expect("Failed to read second number");
    let second_num: f64 = second.trim().parse().expect("Please enter a valid number");

    let result = first_num + second_num;
    println!("The sum of {} and {} is {}.", first_num, second_num, result);
}

/// 3. Write a Rust program that declares a mutable variable counter and initializes it with 0. Then increase it by 1 and decrease it by 1. At the end, print the variable value for each stage.
fn exercise3() {
    let mut some_num: i32 = 0;
    println!("Number is currently {some_num}");
    some_num += 1;
    println!("Number is currently {some_num}");
    some_num -= 1;
    println!("Number is currently {some_num}");
}

/// 4. Write a Rust program that converts an integer to a string and vice versa and prints the result.
fn exercise4() {
    let num = 56789;
    let str_num: String = num.to_string();
    println!("Num val: {}. String val: {}", num, str_num);

    let str_to_parse = "8765";
    let parsed_num = match str_to_parse.parse::<i32>() {
        Ok(n) => n,
        Err(e) => { println!("Could not parse that number. {}", e); return },
    };
    println!("String val: {}. Num val: {}", str_to_parse, parsed_num);
}

/// 5. Write a Rust program that performs basic Math operations - addition, subtraction, multiplication, and division operations on two integers.
fn exercise5() {
    let num1: i32 = 120;
    let num2: i32 = 40;

    println!("The 2 numbers are {num1} and {num2}");

    println!("The sum is {}.", num1 + num2);
    println!("The difference is {}.", num1 - num2);
    println!("The product is {}.", num1 * num2);
    println!("The quotient is {}.", num1 / num2);
}

/// 6. Write a Rust program that checks if a number is even or odd and prints the result.
#[allow(dead_code)]
fn exercise6() {
    println!("Enter a whole number:");
    let mut num_str = String::new();
    io::stdin().read_line(&mut num_str).expect("Failed to read number");
    let num: i32 = num_str.trim().parse().expect("Please enter a valid integer number");
    println!("The number {} is {}.", num, if num % 2 == 0 { "even" } else { "odd" });
}

/// 7. Write a Rust program that counts from 1 to 10 using a loop and prints each number.
fn exercise7() {
    for i in 1..=10 {
        println!("{}", i);
    }
}

/// 8. Write a Rust program that prints all even numbers from 1 to 20 using a while loop.
fn exercise8() {
    let mut i = 1;
    while i <= 20 {
        if i % 2 == 0 {
            println!("{i}");
        }
        i += 1;
    }
}

/// 9. Write a Rust program that prints all elements of an array using a for loop.
fn exercise9() {
    let array: [i32; 5] = [52, 88888, -4, 1234, 999999];
    for item in array {
        println!("{item}");
    }
}

fn factorial(num: u32) -> u64 {
    if num == 1 {
        return 1;
    }

    num as u64 * factorial(num - 1)
}

/// 10. Write a Rust program that defines a function that calculates the factorial of a given number and returns the result.
#[allow(dead_code)]
fn exercise10() {
    println!("Enter a whole number:");
    let mut num_str = String::new();
    io::stdin().read_line(&mut num_str).expect("Failed to read number");
    let num: u32 = num_str.trim().parse().expect("Please enter a valid integer number");
    println!("The factorial of {} is {}.", num, factorial(num));
}

/// 11. Write a Rust program that creates two variables p and q, assigns a value to p, then assigns p to q and try to use p again.
fn exercise11() {
    let p = 50;
    let _q: i32;

    _q = p;
    println!("p is {p}.");
}

fn add100(num: &mut i32) {
    *num += 100;
}

/// 12. Write a Rust function that takes a reference to a variable as a parameter and modifies its value.
fn exercise12() {
    let mut num = 50;
    println!("Num before calling add100() is {num}");

    add100(&mut num);
    println!("Num before calling add100() is {}", num);
}

fn smallest_str<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1 <= str2 {
        return str1;
    }

    str2
}

/// 13. Write a Rust function that takes two string references and returns the smallest one.
fn exercise13() {
    let s1 = "abcd";
    let s2 = "abcde";
    println!("Smallest of \"{s1}\" and \"{s2}\" is \"{}\".", smallest_str(s1, s2));
}

struct Person {
    name: String,
    age: u16,
}

/// 14. Write a Rust program that defines a struct Person with fields like name and age.
fn exercise14() {
    let p: Person = Person { name: "Brian".into(), age: 44, };
    println!("{} is a Person - age {}.", p.name, p.age);
}

/// 15. Write a Rust program that creates an instance of the Person struct and prints its fields.
fn exercise15() {
    exercise14(); // lol same thing
}

#[allow(dead_code)]
#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Yellow,
    Green,
    Black,
    White,
}

/// 16. Write a Rust program that defines an enum Color with variants representing different colors.
fn exercise16() {
    let c: Color = Color::Black;
    println!("Color is {:?}", c);
}

fn maybe_num(n: Option<i32>) {
    let num = match n {
        Some(n) => n,
        None => return,
    };

    println!("Optional num found: {num}.");
}

/// 17. Write a Rust function that takes an optional integer and prints its value if it exists.
fn exercise17() {
    let n1 = Some(5543);
    let n2 = None;

    maybe_num(n1);
    maybe_num(n2);
}

fn pos_is_good(num: i32) -> Result<i32, &'static str> {
    if num >= 0 {
        return Ok(num);
    }

    Err("Negative")
}

/// 18. Write a Rust function that returns a success value for positive numbers and an error value for negative numbers.
fn exercise18() {
    let num1 = 55;
    let num2 = -1234;

    match pos_is_good(num1) {
        Ok(_) => println!("Success: {num1} is Positive"),
        Err(_) => println!("Error: {num1} is Negative")
    }

    match pos_is_good(num2) {
        Ok(_) => println!("Success: {num2} is Positive"),
        Err(_) => println!("Error: {num2} is Negative")
    }
}

fn divide(num1: i32, num2: i32) -> Result<i32, &'static str> {
    if num2 == 0 {
        return Err("Divide by zero.");
    }
    Ok(num1 / num2)
}

/// 19. Write a Rust program that handles the result of a division operation, using pattern matching to distinguish between success and failure.
fn exercise19() {
    let num1 = 50;
    let num2 = 0;

    match divide(num1, num2) {
        Ok(n) => println!("Successful divide: {num1} / {num2} = {n}."),
        Err(e) => println!("Error on {num1} / {num2}: {e}"),
    };
}