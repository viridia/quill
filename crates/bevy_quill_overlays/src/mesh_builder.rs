use bevy::render::mesh::{Mesh, PrimitiveTopology};

/// Trait that abstracts the construction of a mesh.
pub trait MeshBuilder {
    fn topology() -> PrimitiveTopology;

    /// Build the mesh, consuming the builder.
    fn build(self, mesh: &mut Mesh);
}
