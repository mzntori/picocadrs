use rlua::{Lua, Table, Value};
use rlua::prelude::LuaError;

use crate::assets::{PicoColor, Vector, PicoFaceBuilder, Serialize, PicoFaceTags};


/// Represents a Face as stored by picoCAD
#[derive(Debug, PartialEq)]
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
    pub fn set_tag(&mut self, tag: PicoFaceTags) {
        match tag {
            PicoFaceTags::DoubleSided => { self.double_sided = true }
            PicoFaceTags::NoShading => { self.no_shading = true }
            PicoFaceTags::RenderPriority => { self.render_priority = true }
            PicoFaceTags::NoTexture => { self.no_texture = true }
        }
    }

    pub fn unset_tag(&mut self, tag: PicoFaceTags) {
        match tag {
            PicoFaceTags::DoubleSided => { self.double_sided = false }
            PicoFaceTags::NoShading => { self.no_shading = false }
            PicoFaceTags::RenderPriority => { self.render_priority = false }
            PicoFaceTags::NoTexture => { self.no_texture = false }
        };
    }

    pub fn set_color(&mut self, color: PicoColor) {
        match color {
            PicoColor::None => { self.color = PicoColor::Black; }
            _ => { self.color = color; }
        }
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

impl From<String> for PicoFace {
    fn from(s: String) -> Self {
        let mut v: PicoFace = PicoFace::default();
        let lua = Lua::new();
        lua.context(|ctx| {
            let table: Table = ctx.load(s.as_str()).eval().expect("Failed to parse Face");
            v = PicoFace::from(table);
        });
        v
    }
}

impl From<&str> for PicoFace {
    fn from(s: &str) -> Self {
        PicoFace::from(s.to_string())
    }
}

impl Serialize for PicoFace {
    fn serialize(&self) -> String {
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
            None => { s.as_str() }
        }.to_string();
        s.push_str("} }");

        s
    }
}

#[cfg(test)]
mod tests {
    use crate::assets::{PicoColor, PicoFace, PicoFaceBuilder, PicoFaceTags};

    #[test]
    fn face_properties() {
        let mut face = PicoFace::default();
        face.set_tag(PicoFaceTags::DoubleSided);
        face.set_tag(PicoFaceTags::NoShading);
        face.set_tag(PicoFaceTags::NoTexture);
        face.unset_tag(PicoFaceTags::NoTexture);

        assert_eq!(
            face,
            PicoFaceBuilder::new()
                .no_shading(true)
                .double_sided(true)
                .build()
        );
    }

    #[test]
    fn face_color() {
        let mut face = PicoFace::default();
        face.set_color(PicoColor::DarkBlue);

        assert_eq!(face, PicoFaceBuilder::new().color(PicoColor::DarkBlue).build());
    }
}
