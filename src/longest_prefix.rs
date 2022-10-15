fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut res = String::from("");
    if strs.len() == 0 {
        println!("returning early");
        return res;
    }
    res = strs[0].clone();
    for i in 1..strs.len() {
        while !(strs[i].as_str().starts_with(res.as_str())) {
            res.pop();
        }
    }
    res
}
pub fn run() {
    println!(
        "{}",
        longest_common_prefix(vec![
            "dog".to_string(),
            "doog".to_string(),
        ])
    );
}
