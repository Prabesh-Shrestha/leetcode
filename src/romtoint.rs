
fn val(c: &char) -> i32 {
    match &c {
        'I' => return 1,
        'V' => return 5,
        'X' => return 10,
        'L' => return 50,
        'C' => return 100,
        'D' => return 500,
        'M' => return 1000,
        _ => println!("sus char"),
    };
    0
}

fn roman_to_int(s: String) -> i32 {
    let mut total: i32 = 0;
    let mut prev: i32 = 0;
    let mut now: i32;
    for c in s.chars().rev() {
        now = val(&c);
        if prev > now {
            total = total - now;
        } else {
            total = total + now;
        }
        prev = now;
    }
    total
}

pub fn run() {
    println!("{}", roman_to_int("MCMXCIV".to_string()));
}

