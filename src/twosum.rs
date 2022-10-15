
use std::{collections::HashMap};
#[warn(dead_code)]
fn two_sum_square(nums: Vec<i32>, target: i32)-> Vec<i32>{
    for (i, val_i) in nums.iter().enumerate() {
        for (j, val_j) in nums.iter().enumerate() {
            if val_i + val_j == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![0,0]
}
fn two_sum_linear(nums: Vec<i32>, target: i32)-> Vec<i32>{
    let mut compliment: HashMap<i32,usize> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        match compliment.get(num){
            Some(&res) => return vec![res as i32, i as i32], 
            None => compliment.insert(target -num , i)
        };
    }
    vec![0,0]
}
pub fn run() { 
    println!("{:?}", two_sum_square(vec![2,7,12,34], 9));
    println!("{:?}", two_sum_linear(vec![2,7,12,34], 9));
}
