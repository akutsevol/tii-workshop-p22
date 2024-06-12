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
        _ => fibonacci_rec(n - 1) + fibonacci_rec(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius2farenheit() {
        assert_eq!(celsius2farenheit(0), 32);
        assert_eq!(celsius2farenheit(36), 96);
        assert_eq!(celsius2farenheit(-10), 14);
    }

    #[test]
    fn test_farenheit2celsius() {
        assert_eq!(farenheit2celsius(32), 0);
        assert_eq!(farenheit2celsius(90), 32);
        assert_eq!(farenheit2celsius(-1), -18);
    }

    #[test]
    fn test_fibonacci_loop() {
        assert_eq!(fibonacci_loop(0), 0);
        assert_eq!(fibonacci_loop(1), 1);
        assert_eq!(fibonacci_loop(10), 55);
    }

    #[test]
    fn test_fibonacci_rec() {
        assert_eq!(fibonacci_rec(0), 0);
        assert_eq!(fibonacci_rec(1), 1);
        assert_eq!(fibonacci_rec(10), 55);
    }
}
