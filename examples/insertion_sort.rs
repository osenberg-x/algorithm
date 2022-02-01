/*
 * @Copyright: 2020 RDXC Corporation
 * @Author: OsenbergQu
 * @Date: 2022-02-01 11:23:32
 * @LastEditTime: 2022-02-01 20:19:49
 * @LastEditors: OsenbergQu
 * @FilePath: /algorithm/examples/insertion_sort.rs
 * @Description:
 */
use algorithm::insertion_sort;
use algorithm::{self, input_numbers_to_array, random_numbers_to_array};

fn main() {
    println!("Insertion Sort");
    let mut need_sort_array: [i32; 1000] = [0; 1000];

    // input_numbers_to_array(&mut need_sort_array);
    random_numbers_to_array(&mut need_sort_array);
    // println!("Your input is arrays : {:?}", need_sort_array);

    let elapsed_time = insertion_sort(&mut need_sort_array);
    // println!("Sorted array: {:?}", need_sort_array);
    println!("Elapsed time: {}", elapsed_time.as_micros());
}
