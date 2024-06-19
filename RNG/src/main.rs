//Started using the command prompt because apparently you can do that with cargo, and it feels a bit nicer imo.

//IMPORTANT: REMEMBER TO INCLUDE RAND IN THE DEPENDENCIES SECTION OF YOUR CARGO.TOML FILE
//YOU NEED THIS TO BE ABLE TO USE RAND
//AND YOU'LL PROBABLY NEED TO USE THIS FOR OTHER THINGS TOO IN THE FUTURE
//DON'T FORGET HOW TO DO THIS
use rand::Rng;

fn main() {
    println!("Computer! What day of the month is it?");

    //Creating an instance of rand. I think. Needs more research.
    let mut rng = rand::thread_rng();

    //Using a variable to hold a value between 1 and 30.
    //We ended at 31 because the max value is exclusive in Rust.
    let day_of_month = rng.gen_range(1..31);

    //Showing the generated value that was assigned to the variable.
    println!("{}", day_of_month);
}