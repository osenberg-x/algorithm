/*
 * @Copyright: 2020 RDXC Corporation
 * @Author: OsenbergQu
 * @Date: 2022-02-01 20:14:47
 * @LastEditTime: 2022-02-01 21:50:27
 * @LastEditors: OsenbergQu
 * @FilePath: /algorithm/examples/selection_sort.rs
 * @Description:
 */
use algorithm::input_numbers_to_array;
use algorithm::random_numbers_to_array;
use algorithm::selection_sort;

fn main() {
    println!("Selection Sort!");

    let mut need_to_sort: [i32; 10] = [0; 10];
    let mut result: [i32; 10] = [0; 10];

    // random_numbers_to_array(&mut need_to_sort);
    input_numbers_to_array(&mut need_to_sort);

    let elapsed_time = selection_sort(&need_to_sort, &mut result);
    println!("Result: {:?}", result);
    println!("\nElapsed time: {}", elapsed_time.as_micros());
}
