


pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_num : HashMap<i32, usize> =HashMap::new();
        for (i, numb_r) in nums.iter().enumerate(){
            let complement = target - numb_r;
            if let Some(complement_index) = hash_num.get(&complement){
                return vec![*complement_index as i32, i as i32]
            }
            hash_num.insert(*numb_r, i);
        }
        vec![]
    }
}


fn main(){
    assert_eq!(Solution::two_sum(vec![2,7,11,15],9), vec![0,1]);
}