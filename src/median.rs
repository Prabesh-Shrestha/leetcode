
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut merged: Vec<i32> = nums1.clone();
    merged.append(&mut nums2.clone());
    merged.sort();
    let mid = (merged.len() as f64 + 1.0) / 2.0 - 1.0;
    if (mid as i32) as f64 == mid {
        // doesnt have a decimal
        return merged[mid as usize] as f64;
    } else {
        // has decimal
        let mid_value = (merged[mid as usize] as f64 + merged[mid as usize + 1] as f64) / 2.0;
        return mid_value;
    }
    // 0.0
}
pub fn run() {
    println!("{}", find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
}
