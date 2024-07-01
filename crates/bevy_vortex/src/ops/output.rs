use bevy::prelude::*;

use crate::operator::{
    DisplayName, Operator, OperatorCategory, OperatorClass, OperatorDescription, OperatorInput,
    ReflectOperator,
};

#[derive(Debug, Reflect, Clone, Default)]
#[reflect(Operator, @OperatorClass(OperatorCategory::Output), @OperatorDescription("
Displays the output of the shader.
"))]
pub struct Output {
    #[reflect(@OperatorInput, @DisplayName("in"))]
    pub input: LinearRgba,
}

impl Operator for Output {
    fn gen(&self) {
        // todo!()
    }
}
