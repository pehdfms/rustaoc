pub mod day;
pub mod year;

pub type Solution<T> = fn(data: T) -> String;
pub type PreProcessor<T> = fn(data: String) -> T;
