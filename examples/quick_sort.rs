/*
 * @Copyright: xvsos
 * @Author: xvs
 * @Date: 2022-02-02 15:05:19
 * @LastEditTime: 2022-02-02 16:11:56
 * @LastEditors: OsenbergQu
 * @FilePath: /algorithm/examples/quick_sort.rs
 * @Description:
 */
use algorithm::quick_sort;
use algorithm::random_numbers_to_array;

fn main() {
    let mut need_to_sort: [i32; 10] = [0; 10];
    random_numbers_to_array(&mut need_to_sort);

    let mut need_to_sort_vec: Vec<i32> = Vec::new();
    for i in 0..need_to_sort.len() {
        need_to_sort_vec.push(need_to_sort[i]);
    }

    println!("Row vector: {:?}", need_to_sort_vec);
    let sorted_vec = quick_sort(&need_to_sort_vec);
    println!("Sorted vector: {:?}", sorted_vec);
}
