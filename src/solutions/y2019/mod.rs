use crate::domain::entities::year::Year;

use self::days::get_solutions;

mod days;

pub fn get_2019_solutions() -> Year {
    Year::new(get_solutions())
}
