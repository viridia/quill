use bevy::prelude::*;

use crate::{
    gen::{DataType, Expr, ShaderAssembly, TerminalReader},
    operator::{
        DisplayName, Operator, OperatorCategory, OperatorClass, OperatorDescription,
        OperatorOutput, ReflectOperator,
    },
};

#[derive(Debug, Reflect, Clone, Default)]
#[reflect(
    Operator,
    Default,
    @OperatorClass(OperatorCategory::Input),
    @OperatorDescription("
Returns attributes of the input mesh.
"))]
pub struct Geometry {
    /// World position
    #[reflect(@OperatorOutput, @DisplayName("Position"))]
    pub position: Vec3,

    /// World normal
    #[reflect(@OperatorOutput, @DisplayName("Normal"))]
    pub normal: Vec3,

    /// Output texture coordinate
    #[reflect(@OperatorOutput, @DisplayName("UV"))]
    pub uv: Vec2,
}

impl Operator for Geometry {
    fn to_boxed_clone(&self) -> Box<dyn Operator> {
        Box::new(self.clone())
    }

    fn gen(
        &self,
        assembly: &mut ShaderAssembly,
        _reader: &TerminalReader,
        _node_id: Entity,
        out_id: &str,
    ) -> Expr {
        match out_id {
            "position" => {
                assembly.needs_position = true;
                Expr::RefLocal(DataType::Vec3, "mesh.position.xyz".to_string())
            }
            "normal" => {
                assembly.needs_normal = true;
                Expr::RefLocal(DataType::Vec3, "mesh.world_normal".to_string())
            }
            "uv" => {
                assembly.needs_uv = true;
                Expr::RefLocal(DataType::Vec2, "mesh.uv".to_string())
            }
            _ => panic!("Unknown output ID: {}", out_id),
        }
    }
}
