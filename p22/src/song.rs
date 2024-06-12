pub fn twelve_days_of_christmas(mut n: usize) {
    if n > 12 {
        n = 12;
    }

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for (i, day) in days.iter().enumerate().take(n) {
        println!("On the {} day of Christmas, my true love gave to me:", day);
        for (j, gift) in gifts.iter().enumerate().take(i + 1).rev() {
            if j == 0 && i != 0 {
                print!("And ");
            }
            println!("{}", gift);
        }
        println!();
    }
}
