use crate::domain::entities::year::{AocYear, Year};

use self::{y2019::get_2019_solutions, y2022::get_2022_solutions};

pub mod y2019;
pub mod y2022;

pub fn get_year(year: AocYear) -> Year {
    match year {
        AocYear::Year2019 => get_2019_solutions(),
        AocYear::Year2022 => get_2022_solutions(),
    }
}
