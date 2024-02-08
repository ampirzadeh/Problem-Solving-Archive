fn main() {
    let mut sum = 0;

    for a in 0..10 {
        for b in 0..10 {
            for c in 0..10 {
                for d in 0..10 {
                    for e in 0..10 {
                        for f in 0..10 {
                            if 100000 * a + 10000 * b + 1000 * c + 100 * d + 10 * e + 1 * f
                                == a * a * a * a * a
                                    + b * b * b * b * b
                                    + c * c * c * c * c
                                    + d * d * d * d * d
                                    + e * e * e * e * e
                                    + f * f * f * f * f
                                && !(a == 0 && b == 0 && c == 0 && d == 0 && e == 0 && f == 1)
                            {
                                println!("{a}{b}{c}{d}{e}{f}");
                                sum += 100000 * a + 10000 * b + 1000 * c + 100 * d + 10 * e + 1 * f;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("sum: {sum}");
}
