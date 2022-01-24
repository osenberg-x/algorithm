use algorithm::binary_search;
use algorithm::insertion_sort;
use algorithm::random_numbers_to_array;
use std::time::Instant;

fn main() {
    println!("binary search!");
    let mut need_search_array: [i32; 10000] = [0; 10000];

    random_numbers_to_array(&mut need_search_array);

    let sort_elapsed_time = insertion_sort(&mut need_search_array);
    println!("sort elapsed time: {}", sort_elapsed_time.as_micros());

    let now = Instant::now();
    // binary_search(&need_search_array, 123);
    binary_search(&need_search_array, need_search_array[0]);
    let elapsed_time = now.elapsed();
    println!("elapsed time: {}", elapsed_time.as_micros());
}
