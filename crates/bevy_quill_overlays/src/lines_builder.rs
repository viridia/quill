use bevy::{
    math::Vec3,
    render::mesh::{Indices, Mesh, PrimitiveTopology},
};

use super::mesh_builder::MeshBuilder;

/// A builder for creating two-dimensional shapes.
#[derive(Clone, Debug, Default)]
pub struct LinesBuilder {
    vertices: Vec<Vec3>,
    indices: Vec<u32>,
}

impl LinesBuilder {
    /// Create a new `ShapeBuilder`.
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Reserve space for vertices and indices.
    pub fn reserve(&mut self, vertices: usize, indices: usize) -> &mut Self {
        self.vertices.reserve(vertices);
        self.indices.reserve(indices);
        self
    }

    /// Add a vertex to the shape.
    #[inline]
    pub fn push_vertex(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.vertices.push(Vec3::new(x, y, z));
        self
    }

    /// Add an index to the shape.
    #[inline]
    pub fn push_index(&mut self, index: u32) -> &mut Self {
        self.indices.push(index);
        self
    }

    /// Add an index to the shape.
    #[inline]
    pub fn push_indices(&mut self, indices: &[u32]) -> &mut Self {
        self.indices.extend(indices);
        self
    }

    /// Draw a line segment.
    ///
    /// Arguments:
    /// `a` - The start of the line segment.
    /// `b` - The end of the line segment.
    pub fn line(&mut self, a: Vec3, b: Vec3) -> &mut Self {
        let i1 = self.push_vec3_index(a);
        let i2 = self.push_vec3_index(b);
        self.push_index(i1);
        self.push_index(i2);
        self
    }

    /// Draw a sequence of connected line segments.
    ///
    /// Arguments:
    /// `vertices` - The list of vertices.
    pub fn polyline(&mut self, vertices: &[Vec3]) -> &mut Self {
        for i in 0..(vertices.len() - 1) {
            let i1 = self.push_vec3_index(vertices[i]);
            let i2 = self.push_vec3_index(vertices[i + 1]);
            self.push_index(i1);
            self.push_index(i2);
        }
        self
    }

    #[inline]
    fn push_vec3_index(&mut self, v: Vec3) -> u32 {
        let index = self.vertices.len() as u32;
        self.vertices.push(v);
        index
    }
}

impl MeshBuilder for LinesBuilder {
    fn topology() -> PrimitiveTopology {
        PrimitiveTopology::LineList
    }

    /// Copy the shape into a [`Mesh`]. This will consume the builder and return a mesh.
    fn build(self, mesh: &mut Mesh) {
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
        mesh.insert_indices(Indices::U32(self.indices));
    }
}
