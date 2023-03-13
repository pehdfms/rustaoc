use rustaoc::domain::{controllers::solution::SolutionController, entities::year::AocYear};

fn main() {
    println!("Hello, world!");

    dbg!(SolutionController::get(AocYear::Year2022, 4).unwrap());
}
