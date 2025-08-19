mod mall;
use std::collections::HashMap;

pub use mall::*;

pub fn biggest_store(mall: &Mall) -> (&String, &Store) {
    let all_stores_with_name = mall.floors.values().flat_map(|f| &f.stores);
    all_stores_with_name
        .max_by_key(|(_, s)| s.square_meters)
        .unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let all_employees_with_name = mall
        .floors
        .values()
        .flat_map(|f| &f.stores)
        .flat_map(|s| &s.1.employees);

    let mut highest_salary = 0.0;
    for (_, e) in all_employees_with_name.clone() {
        if e.salary > highest_salary {
            highest_salary = e.salary;
        }
    }

    let mut highest_paid = Vec::new();
    for (n, e) in all_employees_with_name {
        if e.salary == highest_salary {
            highest_paid.push((n, e));
        }
    }

    highest_paid
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut count = mall.guards.len();

    let all_stores = mall.floors.values().flat_map(|f| f.stores.values());
    for s in all_stores {
        count += s.employees.len();
    }

    count
}

pub fn check_for_securities(mall: &mut Mall, applicants: HashMap<String, Guard>) {
    let area = mall.floors.values().map(|f| f.size_limit).sum::<u64>();
    for (n, g) in applicants {
        if area as usize / mall.guards.len() > 200 {
            mall.hire_guard(n, g);
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    let employees = mall
        .floors
        .values_mut()
        .flat_map(|f| f.stores.values_mut().flat_map(|s| s.employees.values_mut()));

    for e in employees {
        if e.working_hours.1 - e.working_hours.0 < 10 {
            e.cut(e.salary * 0.1);
        } else {
            e.raise(e.salary * 0.1);
        }
    }
}

#[cfg(test)]
mod tests;
