// closure doesn't have proper type, so return something that implements that functionality
pub fn add_curry(v: i32) -> impl Fn(i32) -> i32 {
    // move makes reference into value
    move |x| x + v
}

#[cfg(test)]
mod tests;