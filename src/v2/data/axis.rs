use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Axis {
    X,
    Y,
    Z,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Foo {
            axis: Axis,
        }

        let x: Foo = serde_json::from_str("{\"axis\": \"x\"}").unwrap();
        let y: Foo = serde_json::from_str("{\"axis\": \"y\"}").unwrap();
        let z: Foo = serde_json::from_str("{\"axis\": \"z\"}").unwrap();

        assert_eq!(x, Foo { axis: Axis::X });
        assert_eq!(y, Foo { axis: Axis::Y });
        assert_eq!(z, Foo { axis: Axis::Z });
    }
}
