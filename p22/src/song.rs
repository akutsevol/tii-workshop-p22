pub fn twelve_days_of_christmas(mut n: usize) -> String {
    if n > 12 {
        n = 12;
    }

    let mut result = String::new();

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
        result.push_str(
            &format!(
                "On the {} day of Christmas, my true love gave to me:\n",
                day
            )
            .to_string(),
        );
        for (j, gift) in gifts.iter().enumerate().take(i + 1).rev() {
            if j == 0 && i != 0 {
                result.push_str("And ");
            }
            result.push_str(&format!("{}\n", gift).to_string());
        }
        result.push('\n');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twelve_days_of_christmas_1() {
        let sample_1 = b"On the first day of Christmas, my true love gave to me:\nA partridge in a pear tree\n\n";
        assert_eq!(twelve_days_of_christmas(1).as_bytes(), sample_1);
    }

    #[test]
    fn test_twelve_days_of_christmas_2() {
        let sample_2 = b"On the first day of Christmas, my true love gave to me:\nA partridge in a pear tree\n\nOn the second day of Christmas, my true love gave to me:\nTwo turtle doves, and\nAnd A partridge in a pear tree\n\n";
        assert_eq!(twelve_days_of_christmas(2).as_bytes(), sample_2);
    }
}
