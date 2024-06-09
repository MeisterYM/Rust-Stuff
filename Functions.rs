fn main() {

    println!("Hello?");

    //Calling a greeting function.
    say_hi();

    println!("Oh, hi!");

    //Calling a subtraction function within a string.
    println!("Did you know that 169 minus 100 is {}?", subtract(169, 100));

    println!("Nice.");
}

//Greeting function.
fn say_hi() {
    println!("Hi!");
}

//Subtraction function.
fn subtract(x: i32,y: i32) -> i32 {
    let result = x - y;

    return result;
}