#[derive(Debug, Clone)]
pub struct Person<'a> {
    pub name: &'a str,      // reference needs lifetime
    pub age: u8,
}

impl Person<'_> {
    pub fn new(name: &str) -> Person {
        Person {
            name: name,
            age: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests;
