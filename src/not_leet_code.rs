
use std::collections::HashMap;

fn cope (word_1: String, word_2: String, target: String) -> bool{
    let mut flag = true;
    let mut word_hash: HashMap<char, usize> = HashMap::new();
    let mut target_hash: HashMap<char, usize> = HashMap::new();
    for c in (word_1 + &word_2).chars() {
        let count = word_hash.entry(c).or_insert(0);
        *count += 1;
    }
    for c in target.chars() {
        let count = target_hash.entry(c).or_insert(0);
        *count += 1;
    }
    for (c, i) in target_hash{
        if let Some(&count) = word_hash.get(&c) {
            if count != i {
                flag = false;
            }
        }
    }
    flag
}







pub fn run() {
    // let mut word_1= String::new();
    // let mut word_2= String::new();
    // let mut target = String::new();
    // io::stdin().read_to_string(&mut word_1).expect("cannot read: word1");
    // io::stdin().read_to_string(&mut word_2).expect("cannot read: word2");
    // io::stdin().read_to_string(&mut target).expect("cannot read: target");
    println!("{}", cope("hello".to_string(), "world".to_string(), "hello world".to_string()));
}
