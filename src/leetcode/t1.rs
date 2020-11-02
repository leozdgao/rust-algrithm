// 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
//
// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。
//
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/two-sum

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let length = nums.len();

    // 两次遍历，时间复杂度 O(n^2)
    // for i in 0..length - 1 {
    //     for j in i + 1..length {
    //         let lval = nums[i];
    //         let rval = nums[j];
    //
    //         if lval + rval == target {
    //             return vec![i as i32, j as i32];
    //         }
    //     }
    // }

    // 缓存差值结果
    let mut map = HashMap::with_capacity(length);

    for i in 0..length {
        let current = nums[i];
        let delta = target - current;

        if let Some(k) = map.get(&current) {
            println!("{}", k);
            return vec![*k as i32, i as i32];
        }

        map.insert(delta, i);
    }

    return vec![];
}

#[test]
fn it_works() {
    let result = two_sum(vec![2,7,11,15], 9);
    println!("t1 result {:?}", result);
}
