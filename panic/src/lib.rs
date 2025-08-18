use std::fs::File;

pub fn open_file(s: &str) -> File {
    // expect
    let f = File::open(s).expect(&format!("failed to open {}", s));
    f
}
