pub mod assets;
pub mod save;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::assets::PicoObject;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn main() {
        let obj = PicoObject::from(r#"
            {
                one="1",
                "two"
            }
        "#.to_string());

        assert_eq!(true, true)
    }
}
