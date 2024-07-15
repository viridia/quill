use bevy::prelude::*;

use crate::{
    gen::{Expr, ShaderAssembly},
    operator::{
        DisplayName, OpValuePrecision, OpValueRange, Operator, OperatorCategory, OperatorClass,
        OperatorDescription, OperatorInput, OperatorInputOnly, OperatorOutput, ReflectOperator,
    },
};

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

    fn gen(&self) -> Expr {
        // let uv = node.read_input::<Vec2>("uv");
        // if uv.is_none() {
        //     uv = "in.uv";
        // }

        // let x_count = node.prop_value::<i32>("x_count");

        // f32 bricks(
        //     uv: vec2<f32>,
        //     x_count: i32,
        //     y_count: i32,
        //     x_spacing: f32,
        //     y_spacing: f32,
        //     x_blur: f32,
        //     y_blur: f32,
        //     stagger: f32,
        //     corner: i32) {
        //   let y = uv.y * float(y_count);
        //   let yr = floor(y);
        //   let yi = floor(y + 0.5);
        //   let yf = smootherstep(y_spacing, y_spacing + y_blur, abs(y - yi));
        //   let x = uv.x * float(x_count) + (floor(yr * 0.5) * 2.0 == yr ? stagger : 0.0);
        //   let xi = floor(x + 0.5);
        //   let xf = smootherstep(x_spacing, x_spacing + x_blur, abs(x - xi));
        //   var value: f32;
        //   if corner == 1 { // Mitered
        //     value = max(0., (xf + yf) - 1.0);
        //   } else if corner == 2 { // Rounded
        //     value = max(0., 1. - sqrt((1.-xf) * (1.-xf) + (1.-yf) * (1.-yf)));
        //   } else { // Square
        //     value = min(xf, yf);
        //   }
        //   return value;
        // }

        // todo!()
        Expr::ConstF32(0.5)
    }

    fn get_deps(&self, assembly: &mut ShaderAssembly) {
        assembly.add_import("embedded://bevy_vortex/ops/wgsl/smootherstep.wgsl".to_string());
        assembly.add_import("embedded://bevy_vortex/ops/wgsl/bricks.wgsl".to_string());
    }
}

impl Default for Bricks {
    fn default() -> Self {
        Bricks {
            output: 0.0,
            uv: Vec2::default(),
            x_count: 2,
            y_count: 4,
            x_spacing: 0.25,
            y_spacing: 0.25,
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
