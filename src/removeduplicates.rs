fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup(); 
    nums.len() as i32
}

pub fn run() {
    let mut myvec = vec![1, 1, 2];
    let k = remove_duplicates(&mut myvec);
    println!("{:?}", myvec);
}
