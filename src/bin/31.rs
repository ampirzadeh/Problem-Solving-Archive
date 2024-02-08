fn main() {
    let mut counter = 0;

    let max_two_hundred = 1;
    let max_hundred = 2;
    let max_fifty = 4;
    let max_twenty = 10;
    let max_ten = 20;
    let max_five = 40;
    let max_two = 100;
    let max_one = 200;

    for two_hundred in 0..(max_two_hundred + 1) {
        for hundred in 0..(max_hundred + 1) {
            for fifty in 0..(max_fifty + 1) {
                for twenty in 0..(max_twenty + 1) {
                    for ten in 0..(max_ten + 1) {
                        for five in 0..(max_five + 1) {
                            for two in 0..(max_two + 1) {
                                for one in 0..(max_one + 1) {
                                    if two_hundred * 200
                                        + hundred * 100
                                        + fifty * 50
                                        + twenty * 20
                                        + ten * 10
                                        + five * 5
                                        + two * 2
                                        + one
                                        == 200
                                    {
                                        // println!("200: {}, 100: {}, 50: {}, 20: {}, 10: {}, 5: {}, 2: {}, 1: {}", two_hundred, hundred, fifty, twenty, ten, five, two, one);
                                        counter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("cnt: {counter}");
}
