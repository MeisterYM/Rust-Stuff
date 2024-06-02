fn main() {
    //Commented line to test variables and methods.
    //println!("{}", numbers[2]);

    //The value the algorithm will be looking for.
    let lookup = 69;

    //Declared and initialized array.
    let numbers = [15, 23, 24, 25, 69];

    //Look at each value in the array. If found, mention that it was found and where. Othwerise say nothing.
    for i in 0..numbers.len() {
        if lookup == numbers[i] {
            println!("Found it! It's in index {}", i);
        }
    }
}