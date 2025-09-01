use roman_numbers_iter::RomanNumber;

fn main() {
    /* let number2 = RomanNumber::from(989);
    println!("{:?}", number2);
    println!("{:?}", u32::from(number2)); */

    let mut number = RomanNumber::from(15);

    println!("{:?}", number);
    println!("{:?}", number.next());
}
