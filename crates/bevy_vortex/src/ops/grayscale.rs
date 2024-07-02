use bevy::prelude::*;

use crate::operator::{
    DisplayName, Operator, OperatorCategory, OperatorClass, OperatorDescription, OperatorInput,
    OperatorOutput, ReflectOperator,
};

#[derive(Debug, Reflect, Clone, Default)]
#[reflect(Operator, Default, @OperatorClass(OperatorCategory::Filter), @OperatorDescription("
Converts the input Linear RGBA color to grayscale.
"))]
pub struct Grayscale {
    #[reflect(@OperatorInput, @DisplayName("in"))]
    pub input: LinearRgba,
    #[reflect(@OperatorOutput)]
    pub output: LinearRgba,
}

impl Operator for Grayscale {
    fn gen(&self) {
        // todo!()
    }
}
