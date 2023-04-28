pub use pico_color::PicoColor;
pub use pico_face::PicoFace;
pub use pico_face_builder::PicoFaceBuilder;
pub use pico_footer::PicoFooter;
pub use pico_header::PicoHeader;
pub use pico_mesh::PicoMesh;
pub use pico_mesh_builder::PicoMeshBuilder;
pub use serialize::Serialize;
pub use vector::Vector;

mod vector;
mod pico_color;
mod pico_face_builder;
mod pico_face;
mod pico_mesh;
mod pico_mesh_builder;
mod pico_header;
mod pico_footer;
mod serialize;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn mesh_parsing_test() {
        let mesh = PicoMesh::from(r#"
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
        print!("{:#?}", mesh);
    }

    #[test]
    #[ignore]
    fn color_conversion() {
        assert_eq!('8', PicoColor::Red.to_char());
    }

    #[test]
    #[ignore]
    fn serialization() {
        println!("{}", PicoHeader::default().serialize());
        println!("{}", Vector::default().serialize());
        println!("{}", PicoFace::default().serialize());
        println!("{}", PicoMesh::default().serialize());
    }

    #[test]
    #[ignore]
    fn vector_deserialization() {
        assert_eq!(Vector::from("{0.0,0.0,0,0}"), Vector::from("{0.0,0.0,0,0}".to_string()))
    }

    #[test]
    #[ignore]
    fn face_deserialization() {
        let face = PicoFace::default().serialize();
        assert_eq!(PicoFace::from(face.clone()), PicoFace::from(face.as_str()))
    }

    #[test]
    #[ignore]
    fn obj_deserialization() {
        let obj = PicoMesh::default().serialize();
        assert_eq!(PicoMesh::from(obj.clone()), PicoMesh::from(obj.as_str()))
    }

    #[test]
    #[ignore]
    fn header_deserialization() {
        let obj = PicoHeader::default().serialize();
        assert_eq!(PicoHeader::from(obj.clone()), PicoHeader::from(obj.as_str()))
    }

    #[test]
    #[ignore]
    fn footer_deserialization() {
        let footer = PicoFooter::default().serialize();
        assert_eq!(PicoFooter::from(footer.clone()), PicoFooter::from(footer.as_str()))
    }

    #[test]
    #[ignore]
    fn vector_implementations() {
        let mut vector = Vector::default();
        vector.add(0.0, 0.0, 2.3);
        assert_eq!(Vector::new(0.0, 0.0, 2.3), vector);

        let mut vector = Vector::default();
        vector.add_vector(&Vector::new(0.0, 0.0, 2.3));
        assert_eq!(Vector::new(0.0, 0.0, 2.3), vector);

        let mut vector = Vector::new(1.0, 1.0, 1.0);
        vector.scale(2.0);
        assert_eq!(Vector::new(2.0, 2.0, 2.0), vector);

        let mut vector = Vector::new(1.0, 1.5, 0.5);
        vector.scale_with(&Vector::new(2.0, 2.0, 8.0));
        assert_eq!(Vector::new(2.0, 3.0, 4.0), vector);

        let mut vector = Vector::new(1.0, 1.0, 1.0);
        vector.flatten();
        assert_eq!(Vector::new(1.0, 1.0, 0.0), vector);

        assert_eq!(Vector::new(3.0, 4.0, 0.0).amount(), 5.0);

        let mut vector = Vector::new(3.0, 4.0, 0.0);
        vector.normalize();
        assert_eq!(Vector::new(0.6, 0.8, 0.0), vector);
    }

    #[test]
    #[ignore]
    fn vector_rotation() {
        let mut v1 = Vector::new(3.0, 3.0, 2.0);
        let v2 = Vector::new(3.0, -2.0, 3.0);

        v1.rotate(&Vector::new(0.25, 0.0, 0.0));
        v1.round();
        assert_eq!(v1, v2)
    }
}