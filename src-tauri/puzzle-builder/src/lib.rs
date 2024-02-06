pub mod game;
pub mod image_splitter;
pub mod error;
pub(crate) mod utils;

pub(crate) type Result<T, E = error::Error> = std::result::Result<T, E>;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
