fn next_permutation<T: Ord>(array: &mut [T]) -> bool {
    // Find non-increasing suffix
    if array.is_empty() {
        return false;
    }
    let mut i: usize = array.len() - 1;
    while i > 0 && array[i - 1] >= array[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }

    // Find successor to pivot
    let mut j: usize = array.len() - 1;
    while array[j] <= array[i - 1] {
        j -= 1;
    }
    array.swap(i - 1, j);

    // Reverse suffix
    array[i..].reverse();
    true
}

fn is_valid(num: &[u16]) -> bool {
    let mut d2d3d4 = 0;
    let mut d3d4d5 = 0;
    let mut d4d5d6 = 0;
    let mut d5d6d7 = 0;
    let mut d6d7d8 = 0;
    let mut d7d8d9 = 0;
    let mut d8d9d10 = 0;

    for i in 0..10 {
        let n = i + 1;

        match n {
            1 => (),
            2 => {
                d2d3d4 += num[i] * 100;
            }
            3 => {
                d3d4d5 += num[i] * 100;
                d2d3d4 += num[i] * 10;
            }
            4 => {
                d4d5d6 += num[i] * 100;
                d3d4d5 += num[i] * 10;
                d2d3d4 += num[i];
            }
            5 => {
                d5d6d7 += num[i] * 100;
                d4d5d6 += num[i] * 10;
                d3d4d5 += num[i];
            }
            6 => {
                d6d7d8 += num[i] * 100;
                d5d6d7 += num[i] * 10;
                d4d5d6 += num[i];
            }
            7 => {
                d7d8d9 += num[i] * 100;
                d6d7d8 += num[i] * 10;
                d5d6d7 += num[i];
            }
            8 => {
                d8d9d10 += num[i] * 100;
                d7d8d9 += num[i] * 10;
                d6d7d8 += num[i];
            }
            9 => {
                d8d9d10 += num[i] * 10;
                d7d8d9 += num[i];
            }
            _ => {
                // 10
                d8d9d10 += num[i];
            }
        }
    }

    return d2d3d4 % 2 == 0
        && d3d4d5 % 3 == 0
        && d4d5d6 % 5 == 0
        && d5d6d7 % 7 == 0
        && d6d7d8 % 11 == 0
        && d7d8d9 % 13 == 0
        && d8d9d10 % 17 == 0;
}

fn main() {
    let mut arr = [1, 0, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut sum = 0;

    while next_permutation(&mut arr) {
        if is_valid(&arr) {
            sum += arr
                .map(|x| x.to_string())
                .join("")
                .parse::<u64>()
                .unwrap_or(0);
        }
    }

    println!("{}", sum);
}
