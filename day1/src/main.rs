use std::io;
use std::env;
use std::fs;

fn main() {
    // Sort the left and right list and compute the distance for each pair
    // Then, add up all the distances
    let args: Vec<String> = env::args().collect();
    let n = args.len();
    println!("args: {:?}", args);
    let filename = &args[n - 1];
    println!("Reading from file: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in contents.lines() {
        let nums: Vec<&str> = line.split_whitespace().collect();
        if nums.len() == 2 {
            if let (Ok(left), Ok(right)) = (nums[0].parse::<i32>(), nums[1].parse::<i32>()) {
                left_numbers.push(left);
                right_numbers.push(right);
            }
        }
    }
    left_numbers.sort_by(|a, b| b.cmp(a));
    right_numbers.sort_by(|a, b| b.cmp(a));

    dbg!(&left_numbers);
    dbg!(&right_numbers);

    let mut total_distance = 0;
    for i in 0..left_numbers.len() {
        total_distance += (left_numbers[i] - right_numbers[i]).abs();
    }
    println!("Total distance: {}", total_distance);
}