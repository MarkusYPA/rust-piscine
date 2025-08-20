use std::collections::HashMap;

pub fn spell(n: u64) -> String {
    if n == 1000000 {
        return "one million".to_owned();
    }

    if n == 0 {
        return "zero".to_owned();
    }

    let names: HashMap<u64, &str> = HashMap::from([
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "fourty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
    ]);

    let mut result = String::new();
    let mut thousands = n / 1000;
    let mut rest = n % 1000;

    if thousands > 0 {
        result.push_str(&hundreds(&mut thousands, &names));
        result.push_str(" thousand ");
    }

    if rest > 0 {
        result.push_str(&hundreds(&mut rest, &names));
    }

    result.trim().to_owned()
}

fn hundreds(n: &mut u64, names: &HashMap<u64, &str>) -> String {
    let mut res = String::new();
    while *n > 0 {
        if *n > 99 {
            let digit = *n / 100;
            *n = *n % 100;
            res.push_str(&format!("{}", &names[&digit]));
            res.push_str(" hundred ");
        } else if *n > 19 {
            let digit = *n / 10;
            *n = *n % 10;
            res.push_str(&format!("{}", &names[&(digit * 10)]));
            if *n % 10 != 0 {
                res.push('-');
            } else {
                res.push(' ');
            }
        } else {
            res.push_str(&format!("{}", &names[n]));
            break;
        }
    }
    res.trim().to_owned()
}

#[cfg(test)]
mod tests;
