use std::io::Write;
use std::path::Path;
use std::fs::OpenOptions;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    // Open the file in append mode if it exists, or create it
    let file = OpenOptions::new()        
        .append(true)
        .create(true)
        .open(path);

    file.unwrap().write(content.as_bytes()).unwrap();

    /* match file {
        Ok(mut f) => {
            if let Err(e) = f.write_all(content.as_bytes()) {
                panic!("Failed to write to file: {}", e);
            }
        }
        Err(e) => {
            panic!("Failed to open or create file: {}", e);     // unwrap would do this
        }
    } */
}