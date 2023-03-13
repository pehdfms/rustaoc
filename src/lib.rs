#![feature(array_chunks, result_option_inspect)]

// there's no real implementation, we just swap type T into ()
// so we don't have to worry about it anymore
trait Ignorable {
    fn ignore(&self) {}
}

impl<R, E> Ignorable for Result<R, E> {}
impl<T> Ignorable for Option<T> {}

pub mod domain;
pub mod solutions;
