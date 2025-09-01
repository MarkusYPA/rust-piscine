use iterators::*;

fn main() {
/*     println!("{:?}", collatz(0));
    println!("{:?}", collatz(1));
    println!("{:?}", collatz(4));
    println!("{:?}", collatz(5));
    println!("{:?}", collatz(6));
    println!("{:?}", collatz(7));
    println!("{:?}", collatz(12)); */

    let test_value = vec![54, 888, 4372, 9999];
    for i in 0..test_value.len() {
        println!("{:?}",  collatz(test_value[i]));
    }
}