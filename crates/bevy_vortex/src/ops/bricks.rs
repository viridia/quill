use std::sync::Arc;

use bevy::prelude::*;

use crate::{
    gen::{DataType, Expr, ShaderAssembly, TerminalReader},
    operator::{
        DisplayName, OpValuePrecision, OpValueRange, Operator, OperatorCategory, OperatorClass,
        OperatorDescription, OperatorInput, OperatorInputOnly, OperatorOutput, ReflectOperator,
    },
};

use super::wgsl::{BRICKS, SMOOTHERSTEP};

#[derive(Debug, Reflect, Clone)]
#[reflect(Operator, Default, @OperatorClass(OperatorCategory::Pattern), @OperatorDescription("
Generates a pattern consisting of alternating rows of bricks.
* **Count X** is the number of bricks along the x-axis.
* **Count Y** is the number of bricks along the y-axis.
* **Spacing X** is the horizontal space between bricks.
* **Spacing Y** is the vertical space between bricks.
* **Blur X** controls the softness of the brick edges in the x direction.
* **Blur Y** controls the softness of the brick edges in the y direction.
* **Offset X** shifts the entire pattern along the X-axis.
* **Offset Y** shifts the entire pattern along the y-axis.
* **Stagger** controls how much the even rows of bricks should be offset relative to the odd rows.
* **Corner** controls the style of the corners (square, round or mitered).
"))]
pub struct Bricks {
    /// Output color
    #[reflect(@OperatorOutput, @DisplayName("Out"))]
    pub output: f32,

    /// Input texture coordinates.
    #[reflect(@OperatorInput, @OperatorInputOnly, @DisplayName("UV"))]
    pub uv: Vec2,

    #[reflect(
        @DisplayName("X Count"),
        @OpValueRange::<i32>(1..=16))]
    pub x_count: i32,

    #[reflect(
        @DisplayName("Y Count"),
        @OpValueRange::<i32>(1..=16))]
    pub y_count: i32,

    #[reflect(
        @DisplayName("X Spacing"),
        @OpValueRange::<f32>(0.0..=0.5),
        @OpValuePrecision(2))]
    pub x_spacing: f32,

    #[reflect(
        @DisplayName("Y Spacing"),
        @OpValueRange::<f32>(0.0..=0.5),
        @OpValuePrecision(2))]
    pub y_spacing: f32,

    #[reflect(
        @DisplayName("X Blur"),
        @OpValueRange::<f32>(0.0..=0.5),
        @OpValuePrecision(2))]
    pub x_blur: f32,

    #[reflect(
        @DisplayName("Y Blur"),
        @OpValueRange::<f32>(0.0..=0.5),
        @OpValuePrecision(2))]
    pub y_blur: f32,

    #[reflect(
        @DisplayName("Stagger "),
        @OpValueRange::<f32>(0.0..=1.0),
        @OpValuePrecision(2))]
    pub stagger: f32,

    #[reflect(@DisplayName("Corner Type"))]
    pub corner: i32,
}

impl Operator for Bricks {
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
        assembly.add_include(BRICKS);
        assembly.add_include(SMOOTHERSTEP);

        let uv = match reader.read_input_terminal(assembly, node_id, "uv") {
            Some(expr) => expr.cast(DataType::Vec2),
            None => {
                assembly.needs_uv = true;
                Expr::RefLocal(DataType::Vec3, "mesh.uv".to_string())
            }
        };

        Expr::FnCall(
            DataType::F32,
            "bricks",
            vec![
                Arc::new(uv),
                Arc::new(Expr::ConstI32(self.x_count)),
                Arc::new(Expr::ConstI32(self.y_count)),
                Arc::new(Expr::ConstF32(self.x_spacing)),
                Arc::new(Expr::ConstF32(self.y_spacing)),
                Arc::new(Expr::ConstF32(self.x_blur)),
                Arc::new(Expr::ConstF32(self.y_blur)),
                Arc::new(Expr::ConstF32(self.stagger)),
                Arc::new(Expr::ConstI32(0)),
            ],
        )
    }
}

impl Default for Bricks {
    fn default() -> Self {
        Bricks {
            output: 0.0,
            uv: Vec2::default(),
            x_count: 2,
            y_count: 4,
            x_spacing: 0.05,
            y_spacing: 0.1,
            x_blur: 0.0,
            y_blur: 0.0,
            stagger: 0.5,
            corner: 0,
        }
    }
}

//       enumVals: [
//         { name: 'Square', value: 0 },
//         { name: 'Mitered', value: 1 },
//         { name: 'Rounded', value: 2 },
//       ],
