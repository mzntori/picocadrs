use crate::assets::{PicoColor, Vector, PicoFace};

/// Builder for `PicoFace`.
#[derive(Debug, PartialEq)]
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
