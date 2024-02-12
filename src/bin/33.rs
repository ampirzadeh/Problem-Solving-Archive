fn cancel_wrongly(numerator: u32, denominator: u32) -> f64 {
    let numerator_str = numerator.to_string();
    let denominator_str = denominator.to_string();

    let new_numerator = numerator_str
        .chars()
        .filter(|&num| !denominator_str.contains(num))
        .collect::<String>();
    let new_denominator = denominator_str
        .chars()
        .filter(|&num| !numerator_str.contains(num))
        .collect::<String>();

    match new_numerator.parse::<f64>() {
        Ok(numer) => match new_denominator.parse::<f64>() {
            Ok(denom) => {
                if numer == numerator.into()
                    || denom == denominator.into()
                    || numerator % 10 == 0
                    || denominator % 10 == 0
                {
                    return 0f64;
                }
                return numer / denom;
            }
            Err(_) => 0f64,
        },
        Err(_) => 0f64,
    }
}

fn main() {
    let mut result_numerator = 1;
    let mut result_denominator = 1;

    for numerator in 10..100 {
        for denominator in numerator..100 {
            let result = numerator as f64 / denominator as f64;

            if cancel_wrongly(numerator, denominator) == result {
                result_numerator *= numerator;
                result_denominator *= denominator;

                println!("{}/{} = {}", numerator, denominator, result);
            }
        }
    }

    let gcd = amplib::gcd(result_numerator, result_denominator);
    println!("{}/{}", result_numerator / gcd, result_denominator / gcd);
}
