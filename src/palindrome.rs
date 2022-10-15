fn is_palindrome(x: i32) -> bool { 
    let str_x = x.to_string(); 
    let mut temp = String::new();
    for c in str_x.chars() {
        temp = c.to_string() + &temp 
    }
    if temp == str_x {
        return true;
    }
    false
}
pub fn run () { 
    println!("{}", is_palindrome(1521));
}
