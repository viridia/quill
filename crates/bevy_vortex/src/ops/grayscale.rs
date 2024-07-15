use bevy::prelude::*;

use crate::{
    gen::Expr,
    operator::{
        DisplayName, OpValuePrecision, OpValueRange, Operator, OperatorCategory, OperatorClass,
        OperatorDescription, OperatorInput, OperatorOutput, ReflectOperator,
    },
};

#[derive(Debug, Reflect, Clone, Default)]
#[reflect(Operator, Default, @OperatorClass(OperatorCategory::Filter), @OperatorDescription("
Converts the input Linear RGBA color to grayscale.
"))]
pub struct Grayscale {
    /// Output color
    #[reflect(@OperatorOutput, @DisplayName("Out"))]
    pub output: LinearRgba,

    /// Input color
    #[reflect(@OperatorInput, @DisplayName("In"))]
    pub input: LinearRgba,

    /// Strength of the grayscale effect, from 0 to 1.
    #[reflect(@OpValueRange::<f32>(0.0..=1.0), @OpValuePrecision(3), @DisplayName("Strength"))]
    pub strength: f32,
}

impl Operator for Grayscale {
    fn to_boxed_clone(&self) -> Box<dyn Operator> {
        Box::new(self.clone())
    }

    fn gen(&self) -> Expr {
        // todo!()
        Expr::ConstColor(LinearRgba::RED)
    }
}
