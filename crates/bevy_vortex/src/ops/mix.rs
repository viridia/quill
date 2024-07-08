use bevy::prelude::*;

use crate::operator::{
    DisplayName, OpValuePrecision, OpValueRange, Operator, OperatorCategory, OperatorClass,
    OperatorDescription, OperatorInput, OperatorOutput, ReflectOperator,
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
    #[reflect(@OperatorInput, @DisplayName("In"))]
    pub input_a: LinearRgba,

    /// Input color B
    #[reflect(@OperatorInput, @DisplayName("A"))]
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

    fn gen(&self) {
        // todo!()
    }
}
