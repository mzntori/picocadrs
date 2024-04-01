use rlua::{Lua, Table, Value};

use crate::assets::{Vector, PicoFace, Serialize, PicoMeshBuilder};

/// Represents a mesh as stored by picoCAD
#[derive(Debug, PartialEq)]
pub struct PicoMesh {
    pub name: String,
    pub pos: Vector,
    pub rot: Vector,
    pub vertices: Vec<Vector>,
    pub faces: Vec<PicoFace>,
}

impl Default for PicoMesh {
    fn default() -> Self {
        Self {
            name: "object".to_string(),
            pos: Vector::default(),
            rot: Vector::default(),
            vertices: vec![],
            faces: vec![],
        }
    }
}

impl From<String> for PicoMesh {
    fn from(s: String) -> Self {
        let mut obj: PicoMesh = PicoMesh::default();

        let lua = Lua::new();
        lua.context(|ctx| {
            let table: Table = ctx.load(s.as_str()).eval().expect("Failed loading lua table");

            obj = PicoMesh::from(table);
        });

        obj
    }
}

impl From<&str> for PicoMesh {
    fn from(s: &str) -> Self {
        PicoMesh::from(s.to_string())
    }
}

impl From<Table<'_>> for PicoMesh {
    fn from(table: Table) -> Self {
        let mut name: String = String::new();
        let mut pos: Vector = Vector::default();
        let mut rot: Vector = Vector::default();
        let mut v: Vec<Vector> = vec![];
        let mut f: Vec<PicoFace> = vec![];

        for pair in table.pairs::<String, Value>() {
            let (key, value) = pair.unwrap();

            match key.as_str() {
                "name" => {
                    name = match value {
                        Value::String(s) => { s.to_str().unwrap().to_string() }
                        _ => { "object".to_string() }
                    };
                }
                "pos" => {
                    pos = match value {
                        Value::Table(t) => { Vector::from(t) }
                        _ => { Vector::default() }
                    };
                }
                "rot" => {
                    rot = match value {
                        Value::Table(t) => { Vector::from(t) }
                        _ => { Vector::default() }
                    };
                }
                "v" => {
                    v = match value {
                        Value::Table(t) => {
                            let mut tempv = vec![];

                            for vertex in t.sequence_values::<Table>() {
                                tempv.push(Vector::from(vertex.expect("Failed to load Vertex")))
                            }

                            tempv
                        }
                        _ => { vec![] }
                    };
                }
                "f" => {
                    f = match value {
                        Value::Table(t) => {
                            let mut faces: Vec<PicoFace> = vec![];

                            for face_table in t.sequence_values::<Table>() {
                                faces.push(PicoFace::from(face_table.expect("Failed to parse Face")))
                            }

                            faces
                        }
                        _ => { vec![] }
                    };
                }
                _ => {}
            }
        }

        PicoMeshBuilder::new()
            .name(name)
            .pos(pos)
            .rot(rot)
            .vertices(v)
            .faces(f)
            .build()
    }
}

impl Serialize for PicoMesh {
    fn serialize(&self) -> String {
        let mut s: String = String::new();

        s.push_str("{\n");
        s.push_str(format!(" name='{}', pos={}, rot={},\n", &self.name, &self.pos.serialize(), &self.rot.serialize()).as_str());

        // vertices
        s.push_str(" v={");
        for vertex in &self.vertices {
            s.push_str(format!("\n  {},", vertex.serialize()).as_str())
        }
        s = match s.strip_suffix(',') {
            Some(str) => { str }
            None => { s.as_str() }
        }.to_string();
        s.push_str("\n },\n f={");

        // faces
        for face in &self.faces {
            s.push_str(format!("\n  {},", face.serialize()).as_str())
        }
        s = match s.strip_suffix(',') {
            Some(str) => { str }
            None => { s.as_str() }
        }.to_string();
        s.push_str("\n }\n}");

        s
    }
}
