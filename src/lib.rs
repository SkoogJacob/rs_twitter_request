pub mod twitter;
use reqwest::*;

#[cfg(test)]
mod tests {
    use crate::twitter;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
