// Specifying Placeholder Types in Trait Definitions with Associated Types
// ================================================================
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(50)
    }
}

impl Iterator for Counter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        Some(50)
    }
}

pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(50)
    }
}

impl Iterator<u8> for Counter {
    fn next(&mut self) -> Option<u8> {
        Some(50)
    }
}