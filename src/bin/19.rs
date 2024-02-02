// This is not the most efficient solution, but I was excited to implement a (almost) complete calendar and I wanted to try it out!

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum WeekDay {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
}
impl WeekDay {
    fn next(&self) -> Self {
        let mut next_week_day_n = *self as u8 + 1;
        if next_week_day_n >= 7 {
            next_week_day_n -= 7
        }
        return unsafe { ::std::mem::transmute(next_week_day_n) };
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Month {
    JANUARY,
    FEBRUARY,
    MARCH,
    APRIL,
    MAY,
    JUNE,
    JULY,
    AUGUST,
    SEPTEMBER,
    OCTOBER,
    NOVEMBER,
    DECEMBER,
}
impl Month {
    fn next(&self) -> Self {
        let mut next_month_n = *self as u8 + 1;
        if next_month_n > 11 {
            next_month_n -= 12
        }
        return unsafe { ::std::mem::transmute(next_month_n) };
    }
}

#[derive(Debug)]
struct Date {
    year: u64,
    month: Month,
    day: u64,
    name: WeekDay,
}

fn main() {
    let mut calendar = vec![
        Date {
            year: 1901,
            month: Month::JANUARY,
            day: 1,
            name: WeekDay::TUESDAY,
        }
    ];
    let until_year = 2001;

    loop {
        let last_date = calendar.last().unwrap();
        let mut new_date: Date = Date {
            year: last_date.year,
            month: last_date.month,
            day: last_date.day,
            name: last_date.name,
        };

        new_date.name = last_date.name.next();
        new_date.month = last_date.month;
        new_date.year = last_date.year;

        if last_date.month == Month::SEPTEMBER
            || last_date.month == Month::APRIL
            || last_date.month == Month::JUNE
            || last_date.month == Month::NOVEMBER
        {
            // 30 dayed months
            if last_date.day == 30 {
                new_date.month = last_date.month.next();
                new_date.day = 1;
            } else {
                new_date.day = last_date.day + 1;
            }
        } else if last_date.month == Month::FEBRUARY {
            // if its february
            if last_date.year % 4 == 0 && last_date.year % 400 != 0 {
                // leap year (feb 29 days)
                if last_date.day == 29 {
                    new_date.month = last_date.month.next();
                    new_date.day = 1;
                } else {
                    new_date.day = last_date.day + 1;
                }
            } else {
                if last_date.day == 28 {
                    new_date.month = last_date.month.next();
                    new_date.day = 1;
                } else {
                    new_date.day = last_date.day + 1;
                }
            }
        } else if last_date.month == Month::DECEMBER {
            if last_date.day == 31 {
                new_date.year = last_date.year + 1;
                new_date.month = last_date.month.next();
                new_date.day = 1;
            } else {
                new_date.day = last_date.day + 1;
            }
        } else {
            // 31 dayed months
            if last_date.day == 31 {
                new_date.month = last_date.month.next();
                new_date.day = 1;
            } else {
                new_date.day = last_date.day + 1;
            }
        }

        if new_date.year == until_year {
            break;
        }

        calendar.push(new_date);
    }

    let mut sunday_the_first_cnt = 0;
    for date in &calendar {
        if date.name == WeekDay::SUNDAY && date.day == 1 {
            sunday_the_first_cnt += 1;
        }
    }

    println!("{:?}", sunday_the_first_cnt);
}
