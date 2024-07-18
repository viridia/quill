use std::sync::Arc;

use bevy::prelude::*;

use crate::{
    gen::{DataType, Expr, ShaderAssembly, TerminalReader},
    operator::{
        DisplayName, OpValuePrecision, OpValueRange, Operator, OperatorCategory, OperatorClass,
        OperatorDescription, OperatorInput, OperatorInputOnly, OperatorOutput, ReflectOperator,
    },
};

use super::wgsl::NOISED;

#[derive(Debug, Reflect, Clone)]
#[reflect(
    Operator,
    Default,
    @OperatorClass(OperatorCategory::Generator),
    @OperatorDescription("
Simplex Noise.
"))]
pub struct Noise {
    /// Output color
    #[reflect(@OperatorOutput, @DisplayName("Out"))]
    pub output: f32,

    /// Output gradient
    #[reflect(@OperatorOutput, @DisplayName("dOut"))]
    pub d_output: Vec3,

    /// Input Position
    #[reflect(@OperatorInput, @OperatorInputOnly, @DisplayName("Vector"))]
    pub vector: Vec3,

    /// Scale
    #[reflect(
        @OperatorInput,
        @DisplayName("Scale"),
        @OpValueRange::<f32>(0.0..=32.0),
        @OpValuePrecision(3))]
    pub scale: f32,

    /// Octaves
    #[reflect(
        @DisplayName("Octaves"),
        @OpValueRange::<i32>(1..=16))]
    pub octaves: i32,

    /// Persistence, a.k.a roughness, from 0 to 1.
    #[reflect(
        @OperatorInput,
        @DisplayName("Roughness"),
        @OpValueRange::<f32>(0.0..=1.0),
        @OpValuePrecision(3))]
    pub roughness: f32,

    /// Distortion, feedback
    #[reflect(
        @OperatorInput,
        @DisplayName("Distortion"),
        @OpValueRange::<f32>(0.0..=100.0),
        @OpValuePrecision(3))]
    pub distortion: f32,
}

impl Operator for Noise {
    fn to_boxed_clone(&self) -> Box<dyn Operator> {
        Box::new(self.clone())
    }

    fn gen(
        &self,
        assembly: &mut ShaderAssembly,
        reader: &TerminalReader,
        node_id: Entity,
        out_id: &str,
    ) -> Expr {
        assembly.add_include(NOISED);

        let vector = match reader.read_input_terminal(assembly, node_id, "vector") {
            Some(expr) => expr.cast(DataType::Vec3),
            None => {
                assembly.needs_position = true;
                Expr::RefLocal(DataType::Vec3, "mesh.position_local.xyz".to_string())
            }
        };

        let scale = match reader.read_input_terminal(assembly, node_id, "scale") {
            Some(expr) => expr.cast(DataType::F32),
            None => Expr::ConstF32(self.scale),
        };

        let octaves = Expr::ConstI32(self.octaves);

        let roughness = match reader.read_input_terminal(assembly, node_id, "roughness") {
            Some(expr) => expr.cast(DataType::F32),
            None => Expr::ConstF32(self.roughness),
        };

        let distortion = match reader.read_input_terminal(assembly, node_id, "distortion") {
            Some(expr) => expr.cast(DataType::F32),
            None => Expr::ConstF32(self.distortion),
        };

        let id = reader.get_node_index(node_id);
        let var_name = format!("noise_out_{}", id.0);
        if !assembly.local_exists(&var_name) {
            assembly.declare_local(
                var_name.clone(),
                DataType::Vec4,
                false,
                Arc::new(Expr::FnCall(
                    DataType::F32,
                    "noised_octaves",
                    vec![
                        Arc::new(vector),
                        Arc::new(scale),
                        Arc::new(octaves),
                        Arc::new(roughness),
                        Arc::new(distortion),
                    ],
                )),
            );
        }

        if out_id == "output" {
            Expr::RefLocal(DataType::F32, format!("{}.x", var_name))
        } else {
            Expr::RefLocal(DataType::Vec3, format!("{}.yzw", var_name))
        }
    }
}

impl Default for Noise {
    fn default() -> Self {
        Noise {
            output: 0.0,
            d_output: Vec3::ZERO,
            vector: Vec3::ZERO,
            scale: 5.0,
            octaves: 2,
            roughness: 0.5,
            distortion: 0.0,
        }
    }
}
