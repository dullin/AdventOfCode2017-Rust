use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Read file into a string
    let mut file = File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //Convert string into array of digits
    let num_array:Vec<u32> = contents.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect();

    //Check for consequent digits
    let mut last = num_array.last().unwrap();
    let mut sum = 0;
    for i in &num_array {
        if i == last {
            sum += i;
        }
        last = i;
    }

    println!("{}", sum);
}
