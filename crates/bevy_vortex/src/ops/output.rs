use bevy::prelude::*;

use crate::operator::{
    DisplayName, Operator, OperatorCategory, OperatorClass, OperatorDescription, OperatorInput,
    ReflectOperator,
};

#[derive(Debug, Reflect, Clone, Default)]
#[reflect(Operator, Default, @OperatorClass(OperatorCategory::Output), @OperatorDescription("
Displays the output of the shader.
"))]
pub struct Output {
    #[reflect(@OperatorInput, @DisplayName("Color"))]
    pub input: LinearRgba,
}

impl Operator for Output {
    fn to_boxed_clone(&self) -> Box<dyn Operator> {
        Box::new(self.clone())
    }

    fn gen(&self) {
        // todo!()
    }
}
