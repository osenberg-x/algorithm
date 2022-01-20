use core::num;
use rand::Rng;
use std::io;

pub fn random_numbers_to_array(array: &mut [i32]) {
    for index in 0..array.len() {
        let number: i32 = rand::thread_rng().gen_range(i32::MIN..i32::MAX);
        array[index] = number;
    }
}

pub fn input_numbers_to_array(array: &mut [i32]) {
    let size = array.len();

    println!("Please enter {} numbers: ", size);
    for index in 0..array.len() {
        println!("Enter {}st number:", index + 1);

        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line.");
        array[index] = s.trim().parse::<i32>().unwrap();
    }
}
