const ONE_DIGITS: [&str; 10] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const TEN_DIGITS: [&str; 8] = [
    "twenty", "thirty", "fourty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const THOUSANDS: [&str; 11] = [
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
    "septillion",
    "octillion",
    "nonillion",
    "decillion",
];

const TEN_WORDS: [&str; 10] = [
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

/// Turns a number from a series of digits into words
///
/// This function is almost pure functional and almost completely uses the monadic design pattern,
/// there is just one closure that doesn't do either
pub fn num_as_words(num: impl Into<i128>) -> String {
    let num: i128 = num.into();
    if num == 0 {
        return "zero".to_string();
    }
    let neg = num.is_negative();
    return neg.then_some("negative").unwrap_or("").to_owned()
        + &Some(num.to_string())
            .map(|item| {
                item.chars()
                    .skip(neg.then(|| 1).unwrap_or(0))
                    .collect::<Vec<char>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<char>>()
                    .chunks(3)
                    .rev()
                    .map(|item| item.iter().collect::<String>())
                    .enumerate()
                    .map(|val| (item.len(), val))
                    .map(|val| ((val.0 / 3) - (val.0 % 3 == 0) as usize - val.1 .0, val.1 .1))
                    .collect::<Vec<(usize, String)>>()
            })
            .unwrap()
            .into_iter()
            .filter(|item| item.1.chars().any(|item| item != '0'))
            .map(|item| {
                // This entire closure is not pure functional
                // Can you turn it to pure functional
                // As an extra, limit yourself to monadic functions
                let mut res = String::new();
                let (thous, nums) = item;
                let tens_resolver = |tens: usize, units: usize| -> String {
                    if tens == 0 {
                        return ONE_DIGITS[units].to_string();
                    }
                    if tens == 1 {
                        return TEN_WORDS[units].to_string();
                    }
                    let mut res = String::new();
                    res.push_str(TEN_DIGITS[tens - 2]);
                    if units != 0 {
                        res.push(' ');
                        res.push_str(ONE_DIGITS[units])
                    }
                    return res;
                };
                match nums
                    .chars()
                    .map(|item| (item as u8 - '0' as u8) as usize)
                    .collect::<Vec<usize>>()[..]
                {
                    [units] => res.push_str(ONE_DIGITS[units]),
                    [units, tens] => res.push_str(tens_resolver(tens, units).as_str()),
                    [units, tens, hundreds] => {
                        if hundreds != 0 {
                            res.push_str(ONE_DIGITS[hundreds]);
                            res.push_str(" hundred");
                            if tens != 0 || units != 0 {
                                res.push_str(" and ");
                            }
                        }
                        res.push_str(tens_resolver(tens, units).as_str());
                    }
                    _ => unreachable!(),
                }

                if thous != 0 {
                    res.push(' ');
                    res.push_str(THOUSANDS[thous - 1])
                }
                return res;
            })
            .filter(|item| !item.is_empty())
            .collect::<Vec<String>>()
            .join(", ");
}
