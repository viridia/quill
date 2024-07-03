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
    #[reflect(@OperatorOutput, @DisplayName("Out"))]
    pub output: LinearRgba,
    #[reflect(@OperatorInput, @DisplayName("In"))]
    pub input: LinearRgba,
}

impl Operator for Grayscale {
    fn to_boxed_clone(&self) -> Box<dyn Operator> {
        Box::new(self.clone())
    }

    fn gen(&self) {
        // todo!()
    }
}
