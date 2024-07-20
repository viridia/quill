use std::sync::Arc;

use bevy::prelude::*;

use crate::{
    gen::{DataType, Expr, ShaderAssembly, TerminalReader},
    operator::{
        DisplayName, OpValuePrecision, OpValueRange, Operator, OperatorCategory, OperatorClass,
        OperatorDescription, OperatorInput, OperatorOutput, ReflectOperator,
    },
};

#[derive(Debug, Reflect, Clone, Default)]
#[reflect(Operator, Default, @OperatorClass(OperatorCategory::Filter), @OperatorDescription("
Interpolate between two colors by a mix factor.
"))]
pub struct Mix {
    /// Output color
    #[reflect(@OperatorOutput, @DisplayName("Out"))]
    pub output: LinearRgba,

    /// Input color A
    #[reflect(@OperatorInput, @DisplayName("A"))]
    pub input_a: LinearRgba,

    /// Input color B
    #[reflect(@OperatorInput, @DisplayName("B"))]
    pub input_b: LinearRgba,

    /// Mix factor, from 0 to 1.
    #[reflect(
        @OperatorInput,
        @DisplayName("Factor"),
        @OpValueRange::<f32>(0.0..=1.0),
        @OpValuePrecision(3))]
    pub factor: f32,
}

impl Operator for Mix {
    fn to_boxed_clone(&self) -> Box<dyn Operator> {
        Box::new(self.clone())
    }

    fn gen(
        &self,
        assembly: &mut ShaderAssembly,
        reader: &TerminalReader,
        node_id: Entity,
        _out_id: &str,
    ) -> Expr {
        let src_a = match reader.read_input_terminal(assembly, node_id, "input_a") {
            Some(expr) => expr.cast(DataType::LinearRgba),
            None => Expr::ConstColor(self.input_a),
        };
        let src_b = match reader.read_input_terminal(assembly, node_id, "input_b") {
            Some(expr) => expr.cast(DataType::LinearRgba),
            None => Expr::ConstColor(self.input_b),
        };
        let factor = match reader.read_input_terminal(assembly, node_id, "factor") {
            Some(expr) => expr.cast(DataType::F32),
            None => Expr::ConstF32(self.factor),
        };
        // TODO: Constant folding. Maybe this should be done as a post-process? We'll need
        // to make a 'mix' object that can evaluate it's arguments.

        Expr::FnCall(
            DataType::LinearRgba,
            "mix",
            vec![Arc::new(src_a), Arc::new(src_b), Arc::new(factor)],
        )
    }
}
