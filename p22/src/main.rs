mod calc;
use crate::calc::{celsius2farenheit, farenheit2celsius, fibonacci_loop, fibonacci_rec};

mod song;
use crate::song::twelve_days_of_christmas;

fn main() {
    println!("celsius2farenheit: {}", celsius2farenheit(36));
    println!("farenheit2celsius: {}", farenheit2celsius(90));
    println!("fibonacci_loop:    {}", fibonacci_loop(5));
    println!("fibonacci_rec:     {}", fibonacci_rec(10));

    twelve_days_of_christmas(12);
}
