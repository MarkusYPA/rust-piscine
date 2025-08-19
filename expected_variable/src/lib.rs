use convert_case::{Case, Casing};
pub use edit_distance::edit_distance;

pub fn expected_variable(comp: &str, exp: &str) -> Option<String> {
    if !comp.to_ascii_lowercase().is_case(Case::Camel)
        && !comp.to_ascii_lowercase().is_case(Case::Snake)
    {
        return None;
    }

    let likeness = 1.0
        - edit_distance(&comp.to_ascii_lowercase(), &exp.to_ascii_lowercase()) as f64
            / exp.len() as f64; // steps to modify

    if likeness < 0.5 {
        return None;
    }

    Some(format!("{}%", (likeness * 100.0).round()))
}

#[cfg(test)]
mod tests;
