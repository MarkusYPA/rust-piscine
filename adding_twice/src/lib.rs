pub fn add_curry(v: i32) -> impl Fn(i32) -> i32 {
    move |x| x + v
}

// take a function f(x) as parameter, and return a function f(f(x))
// add two times the value in add_curry to the original value
pub fn twice<T>(f: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |x| f(f(x))
}

#[cfg(test)]
mod tests;
