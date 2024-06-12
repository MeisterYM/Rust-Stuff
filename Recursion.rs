fn main() {
    //Calling the countdown function.
    countdown(-5);
}

//Function will call itself and decrease given number by 1 so long as said number is above 0.
//Unlike JS, PHP, and Python, this seems to need a return statement for every case so long as you define the return type.
fn countdown(number: i32) -> i32 {
    if number < 0 {
        println!("I can't do a countdown with that number!");
        return 0;
    } else if number > 0 {
        println!("{}", number);
        return countdown(number - 1);
    } else {
        println!("Liftoff!");
        return 0;
    }
}