use std::fs::OpenOptions;
use std::sync::Mutex;
use rlua::{Lua, RegistryKey, Table, Value};
use rlua::prelude::LuaError;

#[derive(Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vector {
    fn default() -> Self {
        Self { x: 0f32, y: 0f32, z: 0f32 }
    }
}

impl From<Table<'_>> for Vector {
    fn from(t: Table) -> Self {
        let vector_values: Vec<Result<f32, LuaError>> = t.sequence_values::<f32>().collect();

        Self {
            x: vector_values.get(0).unwrap().clone().unwrap(),
            y: vector_values.get(1).unwrap().clone().unwrap(),
            z: vector_values.get(2).unwrap().clone().unwrap(),
        }
    }
}

#[derive(Debug)]
pub enum PicoColor {
    None = -1,
    Black = 0,
    DarkBlue = 1,
    DarkPurple = 2,
    DarkGreen = 3,
    Brown = 4,
    DarkGrey = 5,
    LightGrey = 6,
    White = 7,
    Red = 8,
    Orange = 9,
    Yellow = 10,
    Green = 11,
    Blue = 12,
    Lavender = 13,
    Pink = 14,
    LightPeach = 15,
}

impl From<i32> for PicoColor {
    fn from(i: i32) -> Self {
        return match i {
            0 => Self::Black,
            1 => Self::DarkBlue,
            2 => Self::DarkPurple,
            3 => Self::DarkGreen,
            4 => Self::Brown,
            5 => Self::DarkGrey,
            6 => Self::LightGrey,
            7 => Self::White,
            8 => Self::Red,
            9 => Self::Orange,
            10 => Self::Yellow,
            11 => Self::Green,
            12 => Self::Blue,
            13 => Self::Lavender,
            14 => Self::Pink,
            15 => Self::LightPeach,
            _ => Self::None
        };
    }
}

#[derive(Debug)]
pub struct PicoFaceBuilder {
    vertices_index: Vec<i32>,
    color: PicoColor,
    uvs: Vec<Vector>,
    double_sided: bool,
    no_shading: bool,
    render_priority: bool,
    no_texture: bool,
}

impl PicoFaceBuilder {
    pub fn new() -> Self {
        let obj = PicoFace::default();
        Self {
            vertices_index: obj.vertices_index,
            color: obj.color,
            uvs: obj.uvs,
            double_sided: obj.double_sided,
            no_shading: obj.no_shading,
            render_priority: obj.render_priority,
            no_texture: obj.no_texture,
        }
    }

    pub fn vertices_index(mut self, vertices_index: Vec<i32>) -> Self {
        self.vertices_index = vertices_index;
        self
    }

    pub fn color(mut self, color: PicoColor) -> Self {
        self.color = color;
        self
    }

    pub fn uvs(mut self, uvs: Vec<Vector>) -> Self {
        self.uvs = uvs;
        self
    }

    pub fn double_sided(mut self, double_sided: bool) -> Self {
        self.double_sided = double_sided;
        self
    }

    pub fn no_shading(mut self, no_shading: bool) -> Self {
        self.no_shading = no_shading;
        self
    }

    pub fn render_priority(mut self, render_priority: bool) -> Self {
        self.render_priority = render_priority;
        self
    }

    pub fn no_texture(mut self, texture_disabled: bool) -> Self {
        self.no_texture = texture_disabled;
        self
    }

    pub fn build(self) -> PicoFace {
        PicoFace {
            vertices_index: self.vertices_index,
            color: self.color,
            uvs: self.uvs,
            double_sided: self.double_sided,
            no_shading: self.no_shading,
            render_priority: self.render_priority,
            no_texture: self.no_texture,
        }
    }
}

#[derive(Debug)]
pub struct PicoFace {
    pub vertices_index: Vec<i32>,
    pub color: PicoColor,
    pub uvs: Vec<Vector>,
    pub double_sided: bool,
    pub no_shading: bool,
    pub render_priority: bool,
    pub no_texture: bool,
}

impl Default for PicoFace {
    fn default() -> Self {
        Self {
            vertices_index: vec![],
            color: PicoColor::Black,
            uvs: vec![],
            double_sided: false,
            no_shading: false,
            render_priority: false,
            no_texture: false,
        }
    }
}

impl From<Table<'_>> for PicoFace {
    fn from(t: Table) -> Self {
        let builder = PicoFaceBuilder::new();

        builder.vertices_index(t.sequence_values::<i32>().collect());

        for pair in t.pairs::<String, Value>() {
            let (key, value) = pair.unwrap();

            match key.as_str() {
                "c" => {
                    builder.color(match value {
                        Value::Integer(i) => { PicoColor::from(i as i32) }
                        _ => { PicoColor::Black }
                    });
                }
                "uv" => {
                    builder.uvs(match value {
                        Value::Table(t) => {
                            let mut uvs: Vec<Vector> = vec![];
                            let raw_uvs: Vec<f32> = t.sequence_values::<f32>().collect();

                            for uv_index in 0..(raw_uvs.len() as f32 / 2f32).floor() as usize {
                                uvs.push(Vector {
                                    x: raw_uvs.get(2 * uv_index).unwrap().clone(),
                                    y: raw_uvs.get(2 * uv_index + 1).unwrap().clone(),
                                    z: 0f32,
                                })
                            }

                            uvs
                        }
                        _ => { vec![] }
                    });
                }
                "dbl" => {
                    builder.double_sided(match value {
                        Value::Integer(i) => { if i == 1 { true } else { false } }
                        _ => { false }
                    });
                }
                "noshade" => {
                    builder.no_shading(match value {
                        Value::Integer(i) => { if i == 1 { true } else { false } }
                        _ => { false }
                    });
                }
                "notex" => {
                    builder.no_texture(match value {
                        Value::Integer(i) => { if i == 1 { true } else { false } }
                        _ => { false }
                    });
                }
                "prio" => {
                    builder.render_priority(match value {
                        Value::Integer(i) => { if i == 1 { true } else { false } }
                        _ => { false }
                    });
                }
                _ => {}
            }
        }

        builder.build()
    }
}

#[derive(Debug)]
pub struct PicoObjectBuilder {
    name: String,
    pos: Vector,
    rot: Vector,
    vertices: Vec<Vector>,
    faces: Vec<PicoFace>,
}

impl PicoObjectBuilder {
    pub fn new() -> Self {
        let obj = PicoObject::default();
        Self {
            name: obj.name,
            pos: obj.pos,
            rot: obj.rot,
            vertices: obj.vertices,
            faces: obj.faces,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn pos(mut self, pos: Vector) -> Self {
        self.pos = pos;
        self
    }

    pub fn rot(mut self, rot: Vector) -> Self {
        self.rot = rot;
        self
    }

    pub fn vertices(mut self, vertices: Vec<Vector>) -> Self {
        self.vertices = vertices;
        self
    }

    pub fn faces(mut self, faces: Vec<PicoFace>) -> Self {
        self.faces = faces;
        self
    }

    pub fn build(self) -> PicoObject {
        PicoObject {
            name: self.name,
            pos: self.pos,
            rot: self.rot,
            vertices: self.vertices,
            faces: self.faces,
        }
    }
}

#[derive(Debug)]
pub struct PicoObject {
    pub name: String,
    pub pos: Vector,
    pub rot: Vector,
    pub vertices: Vec<Vector>,
    pub faces: Vec<PicoFace>,
}

impl Default for PicoObject {
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

impl From<String> for PicoObject {
    fn from(s: String) -> Self {
        let mut obj: PicoObject = PicoObject::default();

        let lua = Lua::new();
        lua.context(|ctx| {
            let table: Table = ctx.load(s.as_str()).eval().expect("Failed loading lua table");

            obj = PicoObject::from(table);
        });

        obj
    }
}

impl From<Table<'_>> for PicoObject {
    fn from(table: Table) -> Self {
        let builder = PicoObjectBuilder::new();

        for pair in table.pairs::<String, Value>() {
            let (key, value) = pair.unwrap();

            match key.as_str() {
                "name" => {
                    builder.name(match value {
                        Value::String(s) => { s.to_str().unwrap().to_string() }
                        _ => { "object".to_string() }
                    });
                }
                "pos" => {
                    builder.pos(match value {
                        Value::Table(t) => { Vector::from(t) }
                        _ => { Vector::default() }
                    });
                }
                "rot" => {
                    builder.rot(match value {
                        Value::Table(t) => { Vector::from(t) }
                        _ => { Vector::default() }
                    });
                }
                "v" => {
                    builder.vertices(match value {
                        Value::Table(t) => {
                            let mut tempv = vec![];

                            for vertex in t.sequence_values::<Table>() {
                                tempv.push(Vector::from(vertex.expect("Failed to load Vertex")))
                            }

                            tempv
                        }
                        _ => { vec![] }
                    });
                }
                "f" => {
                    builder.faces(match value {
                        Value::Table(t) => {
                            let mut faces: Vec<PicoFace> = vec![];

                            for face_table in t.sequence_values::<Table>() {
                                faces.push(PicoFace::from(face_table.expect("Failed to parse Face")))
                            }

                            faces
                        }
                        _ => { vec![] }
                    });
                }
                _ => {}
            }
        }

        builder.build()
    }
}

#[derive(Debug)]
pub struct PicoHeader {
    pub identifier: String,
    pub name: String,
    pub zoom: i32,
    pub bg_color: PicoColor,
    pub alpha_color: PicoColor,
}

impl From<&str> for PicoHeader {
    fn from(s: &str) -> Self {
        // split header into important data
        let header_data: Vec<&str> = s.split(';').collect();

        Self {
            identifier: header_data.get(0).cloned().unwrap().to_string(),
            name: header_data.get(1).cloned().unwrap().to_string(),
            zoom: header_data.get(2).cloned().unwrap().parse::<i32>().unwrap(),
            bg_color: PicoColor::from(header_data.get(3).cloned().unwrap().parse::<i32>().unwrap()),
            alpha_color: PicoColor::from(header_data.get(4).unwrap().parse::<i32>().unwrap()),
        }
    }
}