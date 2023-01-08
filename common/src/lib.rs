use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Markup {
    pub markup: String,
}

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
