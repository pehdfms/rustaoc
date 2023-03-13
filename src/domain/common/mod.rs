use std::time::Instant;

pub fn timed<C, T>(closure: C) -> (u128, T)
where
    C: FnOnce() -> T,
{
    let start = Instant::now();
    let result = closure();
    (start.elapsed().as_millis(), result)
}
