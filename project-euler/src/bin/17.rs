fn main() {
    let ones = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    let teens = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];

    let mut letter_count = "".to_owned();
    // let num = 115;
    for num in 1..1001 {
        if num < 10 {
            letter_count.push_str(ones[num]);
        }
        if num >= 10 && num < 20 {
            letter_count.push_str(teens[num - 10]);
        }
        if num >= 20 && num < 100 {
            letter_count.push_str(tens[num / 10]);
            letter_count.push_str(ones[num % 10]);
        }
        if num >= 100 && num < 1000 {
            letter_count.push_str(ones[num / 100]);
            letter_count.push_str("hundred");
            if num / 10 % 10 == 0 {
                if num % 10 == 0 {
                    // 300
                } else {
                    // 101
                    letter_count.push_str("and");
                    letter_count.push_str(ones[num % 10]);
                }
            } else if num / 10 % 10 == 1 {
                // 211
                letter_count.push_str("and");
                letter_count.push_str(teens[num % 10]);
            } else {
                // 573
                letter_count.push_str("and");
                letter_count.push_str(tens[num / 10 % 10]);
                letter_count.push_str(ones[num % 10]);
            }
        }
        if num == 1000 {
            letter_count.push_str("onethousand")
        }
    }
    println!("{}", letter_count.len());
}
