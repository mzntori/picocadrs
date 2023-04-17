#[cfg(test)]
mod tests {
    use crate::assets::{PicoColor, PicoObject};
    use crate::save::PicoSave;
    use super::*;

    use std::fs;

    #[test]
    fn object_parsing() {
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

        // print!("{:#?}", obj);

        assert_eq!(true, true)
    }

    #[test]
    fn parse_pico_save() {
        let save = PicoSave::from(fs::read_to_string("C:/Users/Jacob/AppData/Roaming/pico-8/appdata/picocad/plane.txt").expect("Failed to load File"));

        println!("{:#?}", save);
    }

    #[test]
    fn serialize_pico_save() {
        let save = PicoSave::from(fs::read_to_string("C:/Users/Jacob/AppData/Roaming/pico-8/appdata/picocad/plane.txt").expect("Failed to load File"));

        // println!("{}", save.serialize());
    }
}
