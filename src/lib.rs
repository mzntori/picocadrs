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
             name='cube', pos={0,0,0}, rot={0,0,0},
             v={
              {-0.5,-0.5,-0.5},
              {0.5,-0.5,-0.5},
              {0.5,0.5,-0.5},
              {-0.5,0.5,-0.5},
              {-0.5,-0.5,0.5},
              {0.5,-0.5,0.5},
              {0.5,0.5,0.5},
              {-0.5,0.5,0.5}
             },
             f={
              {1,2,3,4, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} },
              {6,5,8,7, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} },
              {5,6,2,1, c=11, dbl=1, noshade=1, notex=1, prio=1, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} },
              {5,1,4,8, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} },
              {2,6,7,3, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} },
              {4,3,7,8, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} }
             }
            }
        "#.to_string());

        print!("{:#?}", obj);

        assert_eq!(true, true)
    }
}
