use algorithm::{self, input_numbers_to_array, random_numbers_to_array};

fn insertion_sort(array: &mut [i32]) {
    let size = array.len();
}

fn main() {
    println!("Insertion Sort");
    let mut need_sort_array: [i32; 10] = [0; 10];

    // input_numbers_to_array(&mut need_sort_array);
    random_numbers_to_array(&mut need_sort_array);

    println!("Your input is arrays : {:?}", need_sort_array);
}
