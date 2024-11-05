fn get_d(champernowne: &str, n: usize) -> u32 {
    champernowne
        .chars()
        .nth(n - 1)
        .unwrap_or('0')
        .to_digit(10)
        .unwrap_or(0)
}

fn main() {
    let mut champernowne = String::new();
    for i in 1..185185 {
        champernowne.push_str(&i.to_string());
    }

    println!(
        "{}",
        get_d(&champernowne, 1)
            * get_d(&champernowne, 10)
            * get_d(&champernowne, 100)
            * get_d(&champernowne, 1000)
            * get_d(&champernowne, 10000)
            * get_d(&champernowne, 100000)
    );
}
