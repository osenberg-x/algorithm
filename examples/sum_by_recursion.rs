/*
 * @Copyright: xvsos
 * @Author: xvs
 * @Date: 2022-02-02 09:40:09
 * @LastEditTime: 2022-02-02 09:42:30
 * @LastEditors: OsenbergQu
 * @FilePath: /algorithm/examples/sum_by_recursion.rs
 * @Description:
 */
use algorithm::input_numbers_to_array;
use algorithm::sum_by_recursion;

/// 分而治之(D&C, divide and conquer)思想
///
/// 递归式解决问题的方法
///
/// 步骤：
///
/// 1.找出基线条件，这种条件尽可能简单。
///
/// 2.不断将问题分解（或者说缩小规模），直到符合基线条件
///
/// 求数组所有元素的和
fn main() {
    let mut need_to_sum: [i32; 10] = [0; 10];
    input_numbers_to_array(&mut need_to_sum);

    println!("Sum = {}", sum_by_recursion(&need_to_sum));
}
