use crate::assets::{Vector, PicoFace, PicoMesh};

/// Builder for `PicoMesh`
#[derive(Debug, PartialEq)]
pub struct PicoMeshBuilder {
    name: String,
    pos: Vector,
    rot: Vector,
    vertices: Vec<Vector>,
    faces: Vec<PicoFace>,
}

impl PicoMeshBuilder {
    /// Returns a new builder containing the `PicoFace::default()` values.
    pub fn new() -> Self {
        let obj = PicoMesh::default();
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
    pub fn build(self) -> PicoMesh {
        PicoMesh {
            name: self.name,
            pos: self.pos,
            rot: self.rot,
            vertices: self.vertices,
            faces: self.faces,
        }
    }
}