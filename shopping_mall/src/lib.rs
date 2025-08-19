/* biggest_store: receives a Mall and returns the Store with the most square_meters.

highest_paid_employee: receives a Mall and returns a vector containing the Employee(s) with the highest salary.

nbr_of_employees: receives a Mall and returns the number of employees and guards as a usize.

check_for_securities: receives a Mall and a map of Guards keyed by their names as a String. If there is not at least 1 guard for every 200 square meters of total floor size, a guard should be added to the Mall.guards.

cut_or_raise: receives a Mall. For each employee, the salary will be raised by 10% if they work for 10 hours or more, else their salary will be decreased by 10%. You can consider that guards are not employees of the mall.
 */

mod mall;
use std::collections::HashMap;

pub use mall::*;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    todo!()
}

pub fn highest_paid_employee(mall: &Mall) ->Vec<(&String, &Employee)> {
    todo!()
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    todo!()
}

pub fn check_for_securities(mall: &mut Mall, applicants: HashMap<String, Guard>) {
    todo!()
}

pub fn cut_or_raise(mall: &mut Mall) {}

#[cfg(test)]
mod tests;
