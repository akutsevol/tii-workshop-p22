pub fn celsius2farenheit(celsius: i32) -> i32 {
    (celsius * 9 / 5) + 32
}

pub fn farenheit2celsius(farenheit: i32) -> i32 {
    (farenheit - 32) * 5 / 9
}

pub fn fibonacci_loop(n: u32) -> u64 {
    match n {
        u32::MIN..=0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0, 1);
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

pub fn fibonacci_rec(n: u32) -> u64 {
    match n {
        u32::MIN..=0 => 0,
        1 => 1,
        _ => fibonacci_rec (n-1)  + fibonacci_rec(n-2)
    }
}
