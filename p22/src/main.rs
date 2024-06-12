mod song;

use crate::song::twelve_days_of_christmas;

fn main() {
    let lyrics = twelve_days_of_christmas(12);
    print!("{}", lyrics);
}
