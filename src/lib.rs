use core::num;
use rand::Rng;
use std::io;
use std::time::{self, Duration, Instant};

/// 随机的生成 i32 类型的整数， 生成的个数为传入数组的元素个数
pub fn random_numbers_to_array(array: &mut [i32]) {
    for index in 0..array.len() {
        let number: i32 = rand::thread_rng().gen_range(i32::MIN..i32::MAX);
        array[index] = number;
    }
}

/// 让用户输入指定数目的 i32 类型整数， 生成的个数为传入数组的元素个数
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

/// 插入排序
pub fn insertion_sort(array: &mut [i32]) -> Duration {
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

/// 二分查找
///
/// 返回查找目标在数组中的下标
///
/// 算法复杂度: T(log2^n)
///
/// `注意: 查找的数组需要是排序好的`
/// `排序算法用的插入排序，可自行替换其他排序方法`
pub fn binary_search(array: &[i32], number: i32) -> Option<i32> {
    let mut min_size = 0;
    let mut max_size = array.len();
    while min_size <= max_size {
        let mid = (min_size + max_size) / 2;
        if array[mid] == number {
            return Some(mid as i32);
        } else if array[mid] > number {
            max_size = mid - 1;
        } else if array[mid] < number {
            min_size = mid + 1;
        }
    }
    return None;
}
