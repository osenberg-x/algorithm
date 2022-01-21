use algorithm::{self, input_numbers_to_array, random_numbers_to_array};
use std::time::{self, Duration, Instant};

fn sort(array: &mut [i32]) -> Duration {
    let now = Instant::now();
    let size = array.len();
    for i in 1..size {
        for j in 0..i {
            // if array[i] > array[j] && array[i] < array[j + 1] {
            if array[i] < array[j] {
                let tmp = array[i];
                array[i] = array[j];
                array[j] = tmp;
            }
        }
    }
    return now.elapsed();
}

fn main() {
    println!("Insertion Sort");
    let mut need_sort_array: [i32; 1000] = [0; 1000];

    // input_numbers_to_array(&mut need_sort_array);
    random_numbers_to_array(&mut need_sort_array);
    // println!("Your input is arrays : {:?}", need_sort_array);

    let elapsed_time = sort(&mut need_sort_array);
    // println!("Sorted array: {:?}", need_sort_array);
    println!("Elased time: {}", elapsed_time.as_micros());
}
