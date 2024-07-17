use bevy::prelude::*;

use crate::{
    gen::{Expr, ShaderAssembly, TerminalReader},
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
    @DisplayName("Color"),
    @OperatorDescription("
Node that generartes a constant RGBA value.
"))]
pub struct ConstColor {
    /// Output color
    #[reflect(@OperatorOutput, @DisplayName("Out"))]
    pub output: LinearRgba,

    /// Input color A
    #[reflect(@DisplayName("Color"))]
    pub color: LinearRgba,
}

impl Operator for ConstColor {
    fn to_boxed_clone(&self) -> Box<dyn Operator> {
        Box::new(self.clone())
    }

    fn gen(
        &self,
        _assembly: &mut ShaderAssembly,
        _reader: &TerminalReader,
        _node_id: Entity,
        _out_id: &str,
    ) -> Expr {
        Expr::ConstColor(self.color)
    }
}
