/**
 * Since for 7-and-above digit numbers the maximum sum 7*9^5=413,343 is smaller than 9,999,999
 * The largest possible value may be at most a 6 digit number
 */

fn main() {
    let mut sum = 0;
    for a in 0..10_u32 {
        for b in 0..10 {
            for c in 0..10 {
                for d in 0..10 {
                    for e in 0..10 {
                        for f in 0..10 {
                            let n = 100000 * a + 10000 * b + 1000 * c + 100 * d + 10 * e + 1 * f;

                            if n == a.pow(5) + b.pow(5) + c.pow(5) + d.pow(5) + e.pow(5) + f.pow(5)
                                && n != 1
                            {
                                println!("{a}{b}{c}{d}{e}{f}");
                                sum += n;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("sum: {sum}");
}
