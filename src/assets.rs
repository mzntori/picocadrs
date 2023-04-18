use rlua::{Lua, Table, Value};
use rlua::prelude::LuaError;

/// A vector containing 3 float values representing x, y and z
#[derive(Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    /// Serializes the Vector into a String representing that Vector as a lua table
    pub fn serialize(&self) -> String {
        format!("{}{},{},{}{}", "{", &self.x, &self.y, &self.z, "}")
    }
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


/// Enum that represents colors in the pico-8 color palette.
#[derive(Debug, Clone, Copy)]
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

impl PicoColor {
    /// Returns the Color represented as an integer between 0 and 15.
    /// Returns -1 if its not a valid color.
    pub fn to_i32(&self) -> i32 {
        return match self {
            Self::Black => 0,
            Self::DarkBlue => 1,
            Self::DarkPurple => 2,
            Self::DarkGreen => 3,
            Self::Brown => 4,
            Self::DarkGrey => 5,
            Self::LightGrey => 6,
            Self::White => 7,
            Self::Red => 8,
            Self::Orange => 9,
            Self::Yellow => 10,
            Self::Green => 11,
            Self::Blue => 12,
            Self::Lavender => 13,
            Self::Pink => 14,
            Self::LightPeach => 15,
            _ => -1
        };
    }
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

impl From<char> for PicoColor {
    fn from(c: char) -> Self {
        return match c {
            '0' => Self::Black,
            '1' => Self::DarkBlue,
            '2' => Self::DarkPurple,
            '3' => Self::DarkGreen,
            '4' => Self::Brown,
            '5' => Self::DarkGrey,
            '6' => Self::LightGrey,
            '7' => Self::White,
            '8' => Self::Red,
            '9' => Self::Orange,
            'a' => Self::Yellow,
            'b' => Self::Green,
            'c' => Self::Blue,
            'd' => Self::Lavender,
            'e' => Self::Pink,
            'f' => Self::LightPeach,
            _ => Self::None
        };
    }
}

/// Builder for `PicoFace`.
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
    /// Returns a new builder containing the `PicoFace::default()` values.
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

    /// Sets the faces vertices indexes to the ones provided as a parameter in the provided order.
    pub fn vertices_index(mut self, vertices_index: Vec<i32>) -> Self {
        self.vertices_index = vertices_index;
        self
    }

    /// Sets the faces color to the provided color.
    pub fn color(mut self, color: PicoColor) -> Self {
        self.color = color;
        self
    }

    /// Sets the uv coordinates to the ones provided as a parameter in the provided order.
    pub fn uvs(mut self, uvs: Vec<Vector>) -> Self {
        self.uvs = uvs;
        self
    }

    /// Sets the face's property to render textures on both sides to the provided value.
    pub fn double_sided(mut self, double_sided: bool) -> Self {
        self.double_sided = double_sided;
        self
    }

    /// Sets the face's property to not have shadows to the provided value.
    pub fn no_shading(mut self, no_shading: bool) -> Self {
        self.no_shading = no_shading;
        self
    }

    /// Sets the face's property to render first to the provided value.
    pub fn render_priority(mut self, render_priority: bool) -> Self {
        self.render_priority = render_priority;
        self
    }

    /// Sets the face's property to have no texture to the provided value.
    pub fn no_texture(mut self, texture_disabled: bool) -> Self {
        self.no_texture = texture_disabled;
        self
    }

    /// Builds the `PicoFace` instance.
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

/// Represents a Face as stored by picoCAD
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

impl PicoFace {
    /// Serializes the face into a string in form ofa lua table, that picoCAD can read.
    pub fn serialize(&self) -> String {
        let mut s: String = String::new();

        // start
        s.push('{');

        // vertices
        for index in &self.vertices_index {
            s.push_str(format!("{},", index).as_str());
        }
        // color
        s.push_str(format!(" c={},", self.color.to_i32()).as_str());

        // bools
        if self.double_sided { s.push_str(" dbl=1,") }
        if self.no_shading { s.push_str(" noshade=1,") }
        if self.no_texture { s.push_str(" notex=1,") }
        if self.render_priority { s.push_str(" prio=1,") }

        // uvs
        s.push_str(" uv={");
        for uv_vector in &self.uvs {
            s.push_str(format!("{},{},", uv_vector.x, uv_vector.y).as_str());
        }
        s = match s.strip_suffix(',') {
            Some(str) => { str }
            None => { "" }
        }.to_string();
        s.push_str("} }");

        s
    }
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

        let mut c: PicoColor = PicoColor::Black;
        let mut uv: Vec<Vector> = vec![];
        let mut dbl: bool = false;
        let mut noshade: bool = false;
        let mut notex: bool = false;
        let mut prio: bool = false;

        let mut vertices_indexes: Vec<i32> = vec![];
        for v in t.clone().sequence_values::<i32>() {
            vertices_indexes.push(v.expect("Failed to parse Vertex Index"))
        }

        for pair in t.pairs::<String, Value>() {
            let (key, value) = pair.unwrap();

            match key.as_str() {
                "c" => {
                    c = match value {
                        Value::Integer(i) => { PicoColor::from(i as i32) }
                        _ => { PicoColor::Black }
                    };
                }
                "uv" => {
                    uv = match value {
                        Value::Table(t) => {
                            let mut uvs: Vec<Vector> = vec![];
                            let raw_uvs: Vec<Result<f32, LuaError>> = t.sequence_values::<f32>().collect();

                            for uv_index in 0..(raw_uvs.len() as f32 / 2f32).floor() as usize {
                                uvs.push(Vector {
                                    x: raw_uvs.get(2 * uv_index).unwrap().clone().expect("Failed to read UV Coordinates"),
                                    y: raw_uvs.get(2 * uv_index + 1).unwrap().clone().expect("Failed to read UV Coordinates"),
                                    z: 0f32,
                                })
                            }

                            uvs
                        }
                        _ => { vec![] }
                    };
                }
                "dbl" => {
                    dbl = match value {
                        Value::Integer(i) => { if i == 1 { true } else { false } }
                        _ => { false }
                    };
                }
                "noshade" => {
                    noshade = match value {
                        Value::Integer(i) => { if i == 1 { true } else { false } }
                        _ => { false }
                    };
                }
                "notex" => {
                    notex = match value {
                        Value::Integer(i) => { if i == 1 { true } else { false } }
                        _ => { false }
                    };
                }
                "prio" => {
                    prio = match value {
                        Value::Integer(i) => { if i == 1 { true } else { false } }
                        _ => { false }
                    };
                }
                _ => {}
            }
        }

        builder.vertices_index(vertices_indexes)
            .color(c)
            .uvs(uv)
            .double_sided(dbl)
            .no_shading(noshade)
            .no_texture(notex)
            .render_priority(prio)
            .build()
    }
}

/// Builder for `PicoObject`
#[derive(Debug)]
pub struct PicoObjectBuilder {
    name: String,
    pos: Vector,
    rot: Vector,
    vertices: Vec<Vector>,
    faces: Vec<PicoFace>,
}

impl PicoObjectBuilder {
    /// Returns a new builder containing the `PicoFace::default()` values.
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

    /// Sets the objects name to the provided value.
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// Sets the objects origin to the provided coordinates.
    pub fn pos(mut self, pos: Vector) -> Self {
        self.pos = pos;
        self
    }

    /// Sets the objects lightsource rotation to the provided value.
    pub fn rot(mut self, rot: Vector) -> Self {
        self.rot = rot;
        self
    }

    /// Sets the objects vertices to the provided positions in the provided order.
    pub fn vertices(mut self, vertices: Vec<Vector>) -> Self {
        self.vertices = vertices;
        self
    }

    /// Sets the objects faces to the provided values in the provided order.
    pub fn faces(mut self, faces: Vec<PicoFace>) -> Self {
        self.faces = faces;
        self
    }

    /// Builds the `PicoObject` instance.
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

/// Represents an object as stored by picoCAD
#[derive(Debug)]
pub struct PicoObject {
    pub name: String,
    pub pos: Vector,
    pub rot: Vector,
    pub vertices: Vec<Vector>,
    pub faces: Vec<PicoFace>,
}

impl PicoObject {
    /// Serializes the face into a string in form ofa lua table, that picoCAD can read.
    pub fn serialize(&self) -> String {
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
            None => { "" }
        }.to_string();
        s.push_str("\n },\n f={");

        // faces
        for face in &self.faces {
            s.push_str(format!("\n  {},", face.serialize()).as_str())
        }
        s = match s.strip_suffix(',') {
            Some(str) => { str }
            None => { "" }
        }.to_string();
        s.push_str("\n }\n}");

        s
    }
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

        PicoObjectBuilder::new()
            .name(name)
            .pos(pos)
            .rot(rot)
            .vertices(v)
            .faces(f)
            .build()
    }
}

/// Represents the header of a picoCAD savefile.
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
            identifier: header_data.get(0).unwrap().to_string(),
            name: header_data.get(1).unwrap().to_string(),
            zoom: header_data.get(2).unwrap().parse::<i32>().unwrap(),
            bg_color: PicoColor::from(header_data.get(3).unwrap().parse::<i32>().unwrap()),
            alpha_color: PicoColor::from(header_data.get(4).unwrap().parse::<i32>().unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_parsing_test() {
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
    }
}