use crate::domain::entities::year::Year;

use self::days::get_solutions;

#[allow(unused_variables, dead_code)]
mod days;

pub fn get_2022_solutions() -> Year {
    Year::new(get_solutions())
}
