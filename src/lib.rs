use rand::Rng;
use std::io;
use std::time::{Duration, Instant};

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

/// 选择排序
///
/// 遍历所有元素，将符合条件的元素放入新的数组，形成有序的新数组
///
/// 算法复杂度: T(n^2)
///
/// `此示例中使用按照升序排序`
pub fn selection_sort(array: &[i32], result: &mut [i32]) -> Duration {
    let now = Instant::now();
    let mut min = array[0];
    let size = array.len();
    let mut indexs: Vec<usize> = Vec::new();
    let mut index = 0;
    for i in 0..size {
        for m in 0..size {
            if !indexs.contains(&m) {
                min = array[m];
                break;
            }
        }

        for j in 0..size {
            if indexs.contains(&j) {
                continue;
            }
            if min >= array[j] {
                min = array[j];
                index = j;
            }
        }
        result[i] = min;
        indexs.push(index);
    }

    return now.elapsed();
}

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
pub fn sum_by_recursion(array: &[i32]) -> i32 {
    if array.len() == 1 {
        return array[0];
    } else {
        return array[0] + sum_by_recursion(&array[1..array.len()]);
    }
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

/// 链表
mod linked_list {
    // https://zhuanlan.zhihu.com/p/341626700
    use std::rc::Rc;

    pub type Link<T> = Option<Rc<Node<T>>>;

    pub struct Node<T> {
        element: T,
        next: Link<T>,
    }
    /// 单链表实现
    pub struct SingleLinkedList<T> {
        head: Link<T>,
        tail: Link<T>,
    }

    impl<T> SingleLinkedList<T> {
        fn new() -> Self {
            SingleLinkedList {
                head: None,
                tail: None,
            }
        }

        /// 在末尾添加节点
        fn append(&mut self, element: T) -> bool {
            // if self.head == None {
            //     self.head =
            // }
            true
        }

        /// 在指定位置添加节点
        fn insert(location: i32, element: T) -> bool {
            true
        }

        /// 删除指定位置节点
        fn delete(element: T) -> bool {
            true
        }

        /// 反转整个链表
        fn reverse() -> bool {
            true
        }

        /// 返回第一个节点
        fn front() -> Link<T> {
            None
        }

        /// 返回最后一个节点
        fn back() -> Link<T> {
            None
        }

        /// 返回并删除第一个节点
        fn pop_front() -> Link<T> {
            None
        }

        /// 返回并删除最后一个节点
        fn pop_back() -> Link<T> {
            None
        }

        /// 清除整个链表
        fn clear() {}
    }
}
