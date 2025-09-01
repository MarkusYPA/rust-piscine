use step_iterator::*;

fn main() {
    for v in StepIterator::new(0, 100, 10) {
        print!("{},", v);
    }
    println!();

    for v in StepIterator::new(0, 100, 12) {
        print!("{},", v)
    }
    println!();

    for (i, v) in StepIterator::new(0.3, 10.0, 0.5).enumerate() {
        println!("position: {}, value: {}, ", i, v);
    }
}
