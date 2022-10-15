fn reverse(x: i32) -> i32 {
    let mut rev: i32 = 0;
    let mut temp = x;
    let mut digit;
    while temp != 0 {
        digit = temp % 10;
        if let None = rev.checked_mul(10){
            println!("problem here finding mul");
            return 0; 
        };
        let rev_ten = rev*10;
        rev = rev_ten + digit;
        println!("{}", rev);
        temp = temp / 10;
    }
    rev
}

pub fn run() {
    println!("{}", reverse(-2147483412));
}
