use json::object;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {

    let (mut cals, mut carbs, mut proteins, mut fats) = (0.0, 0.0, 0.0, 0.0);
    for f in foods {
        let energy = f.calories.1.to_owned().strip_suffix("kcal").unwrap().parse::<f64>().unwrap();
        cals += energy * f.nbr_of_portions;
        carbs += f.carbs * f.nbr_of_portions;
        proteins += f.proteins * f.nbr_of_portions;
        fats += f.fats * f.nbr_of_portions;
    }

    // Doesn't work for 123.05, drops the 0:
    /* let cals = format!("{}.{}", cals.round() as i32, (cals * 100.0).round() as i32 % 100 );
    let carbs = format!("{}.{}", carbs.round() as i32, (carbs * 100.0).round() as i32 % 100 );
    let proteins = format!("{}.{}", proteins.round() as i32, (proteins * 100.0).round() as i32 % 100 );
    let fats = format!("{}.{}", fats.round() as i32, (fats * 100.0).round() as i32 % 100 ); */

    // Round to 0-2 decimal places
    let cals = (cals * 100.0).round() / 100.0;
    let carbs = (carbs * 100.0).round() / 100.0;
    let proteins = (proteins * 100.0).round() / 100.0;
    let fats = (fats * 100.0).round() / 100.0;
    
    object! {cals: cals, carbs: carbs, proteins: proteins, fats: fats}
}

#[cfg(test)]
mod tests;