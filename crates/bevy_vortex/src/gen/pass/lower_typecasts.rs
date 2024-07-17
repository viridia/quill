use core::panic;
use std::sync::Arc;

use crate::gen::{DataType, Expr};

/// Lower typecasts to WGSL operators or function calls.
pub fn lower_typecasts(expr: Arc<Expr>) -> Arc<Expr> {
    match expr.as_ref() {
        Expr::LiteralStr(_)
        | Expr::ConstI32(_)
        | Expr::ConstF32(_)
        | Expr::ConstVec2(_)
        | Expr::ConstVec3(_)
        | Expr::ConstVec4(_)
        | Expr::ConstColor(_) => expr,
        Expr::LocalDefn(_, _, _) => todo!(),
        Expr::Assign(dt, lhs, rhs) => {
            Arc::new(Expr::Assign(*dt, lhs.clone(), lower_typecasts(rhs.clone())))
        }
        Expr::RefLocal(_, _) => expr,
        Expr::RefInput(_, _) => expr,
        Expr::RefUniform(_, _) => expr,
        Expr::TypeCast(to_type, e) => {
            let from_type = e.typ();
            let expr = lower_typecasts(e.clone());
            match (to_type, from_type) {
                // No conversion needed.
                (_, _) if *to_type == from_type => expr,

                // Void type is not allowed in expressions.
                (DataType::Void, _) => panic!("Cannot cast expression to void type"),
                (_, DataType::Void) => panic!("Cannot cast expression from void type"),

                (DataType::I32, DataType::F32) => {
                    Arc::new(Expr::FnCall(DataType::I32, "i32".to_string(), vec![expr]))
                }

                // Use only x, then cast to i32
                (DataType::I32, DataType::Vec2 | DataType::Vec3 | DataType::Vec4) => {
                    Arc::new(Expr::FnCall(
                        DataType::I32,
                        "i32".to_string(),
                        vec![Arc::new(Expr::GetAttr(
                            DataType::Vec3,
                            expr,
                            "x".to_string(),
                        ))],
                    ))
                }

                // Extract luminance, convert to i32
                (DataType::I32, DataType::LinearRgba) => Arc::new(Expr::FnCall(
                    DataType::I32,
                    "i32".to_string(),
                    vec![Arc::new(Expr::GetAttr(
                        DataType::Vec3,
                        Arc::new(Expr::FnCall(
                            DataType::F32,
                            "dot".to_string(),
                            vec![
                                expr,
                                Arc::new(Expr::FnCall(
                                    DataType::Vec3,
                                    "vec3<f32>".to_string(),
                                    vec![
                                        Arc::new(Expr::ConstF32(0.2126)),
                                        Arc::new(Expr::ConstF32(0.7152)),
                                        Arc::new(Expr::ConstF32(0.0722)),
                                        Arc::new(Expr::ConstF32(0.0)),
                                    ],
                                )),
                            ],
                        )),
                        "x".to_string(),
                    ))],
                )),

                // Simple numeric cast
                (DataType::F32, DataType::I32) => {
                    Arc::new(Expr::FnCall(DataType::F32, "f32".to_string(), vec![expr]))
                }

                // Use only x
                (DataType::F32, DataType::Vec2 | DataType::Vec3 | DataType::Vec4) => {
                    Arc::new(Expr::GetAttr(DataType::Vec3, expr, "xyz".to_string()))
                }

                // For color to f32, extract luminance
                (DataType::F32, DataType::LinearRgba) => Arc::new(Expr::FnCall(
                    DataType::F32,
                    "dot".to_string(),
                    vec![
                        expr,
                        Arc::new(Expr::FnCall(
                            DataType::Vec3,
                            "vec3<f32>".to_string(),
                            vec![
                                Arc::new(Expr::ConstF32(0.2126)),
                                Arc::new(Expr::ConstF32(0.7152)),
                                Arc::new(Expr::ConstF32(0.0722)),
                                Arc::new(Expr::ConstF32(0.0)),
                            ],
                        )),
                    ],
                )),

                // Splat constructor with f32 conversion
                (DataType::Vec2, DataType::I32) => Arc::new(Expr::FnCall(
                    DataType::Vec2,
                    "vec2<f32>".to_string(),
                    vec![Arc::new(Expr::FnCall(
                        DataType::F32,
                        "f32".to_string(),
                        vec![expr],
                    ))],
                )),

                // Splat constructor
                (DataType::Vec2, DataType::F32) => Arc::new(Expr::FnCall(
                    DataType::Vec3,
                    "vec2<f32>".to_string(),
                    vec![expr],
                )),

                // Use only xy
                (DataType::Vec2, DataType::Vec3) => {
                    Arc::new(Expr::GetAttr(DataType::Vec3, expr, "xy".to_string()))
                }

                // Use only xy
                (DataType::Vec2, DataType::Vec4) => {
                    Arc::new(Expr::GetAttr(DataType::Vec3, expr, "xy".to_string()))
                }

                // Use only rg
                (DataType::Vec2, DataType::LinearRgba) => {
                    Arc::new(Expr::GetAttr(DataType::Vec3, expr, "rg".to_string()))
                }

                // Splat constructor with f32 conversion
                (DataType::Vec3, DataType::I32) => Arc::new(Expr::FnCall(
                    DataType::Vec3,
                    "vec3<f32>".to_string(),
                    vec![Arc::new(Expr::FnCall(
                        DataType::F32,
                        "f32".to_string(),
                        vec![expr],
                    ))],
                )),

                // Splat constructor
                (DataType::Vec3, DataType::F32) => Arc::new(Expr::FnCall(
                    DataType::Vec3,
                    "vec3<f32>".to_string(),
                    vec![expr],
                )),

                // Spread constructor with z=0
                (DataType::Vec3, DataType::Vec2) => Arc::new(Expr::FnCall(
                    DataType::Vec3,
                    "vec3<f32>".to_string(),
                    vec![expr, Arc::new(Expr::ConstF32(0.0))],
                )),

                // Use only xyz
                (DataType::Vec3, DataType::Vec4) => {
                    Arc::new(Expr::GetAttr(DataType::Vec3, expr, "xyz".to_string()))
                }

                // Drop alpha
                (DataType::Vec3, DataType::LinearRgba) => {
                    Arc::new(Expr::GetAttr(DataType::Vec3, expr, "rgb".to_string()))
                }

                // Splat constructor with f32 conversion
                (DataType::Vec4, DataType::I32) => Arc::new(Expr::FnCall(
                    DataType::Vec4,
                    "vec4<f32>".to_string(),
                    vec![Arc::new(Expr::FnCall(
                        DataType::F32,
                        "f32".to_string(),
                        vec![expr],
                    ))],
                )),

                // Splat constructor
                (DataType::Vec4, DataType::F32) => Arc::new(Expr::FnCall(
                    DataType::Vec3,
                    "vec4<f32>".to_string(),
                    vec![expr],
                )),

                // Spread constructor with zw=0
                (DataType::Vec4, DataType::Vec2) => Arc::new(Expr::FnCall(
                    DataType::Vec4,
                    "vec3<f32>".to_string(),
                    vec![
                        expr,
                        Arc::new(Expr::ConstF32(0.0)),
                        Arc::new(Expr::ConstF32(0.0)),
                    ],
                )),

                // Spread constructor with w=0
                (DataType::Vec4, DataType::Vec3) => Arc::new(Expr::FnCall(
                    DataType::Vec4,
                    "vec3<f32>".to_string(),
                    vec![expr, Arc::new(Expr::ConstF32(0.0))],
                )),
                (DataType::Vec4, DataType::LinearRgba) => expr,

                (DataType::LinearRgba, DataType::I32) => todo!("cast i32 to rgba"),

                // Splat constructor with alpha: `vec4<f32>(vec3<f32>(expr), 1.0)`
                (DataType::LinearRgba, DataType::F32) => Arc::new(Expr::FnCall(
                    DataType::LinearRgba,
                    "vec4<f32>".to_string(),
                    vec![
                        Arc::new(Expr::FnCall(
                            DataType::Vec3,
                            "vec3<f32>".to_string(),
                            vec![expr],
                        )),
                        Arc::new(Expr::ConstF32(1.0)),
                    ],
                )),

                // Assign to RG, default BA: `vec4<f32>(expr, 0.0, 1.0)`
                (DataType::LinearRgba, DataType::Vec2) => Arc::new(Expr::FnCall(
                    DataType::LinearRgba,
                    "vec4<f32>".to_string(),
                    vec![
                        expr,
                        Arc::new(Expr::ConstF32(0.0)),
                        Arc::new(Expr::ConstF32(1.0)),
                    ],
                )),

                // Assign to RGB, default A: `vec4<f32>(expr, 1.0)`
                (DataType::LinearRgba, DataType::Vec3) => Arc::new(Expr::FnCall(
                    DataType::LinearRgba,
                    "vec4<f32>".to_string(),
                    vec![expr, Arc::new(Expr::ConstF32(1.0))],
                )),

                // Identity conversion
                (DataType::LinearRgba, DataType::Vec4) => expr,

                _ => panic!("Unknown cast from {:?} to {:?}", from_type, to_type),
            }
        }

        Expr::GetAttr(dt, base, fieldname) => Arc::new(Expr::GetAttr(
            *dt,
            lower_typecasts(base.clone()),
            fieldname.clone(),
        )),

        Expr::BinOp(_, _, _, _) => todo!(),

        Expr::FnCall(_, f, args) => {
            let args = args
                .iter()
                .map(|arg| lower_typecasts(arg.clone()))
                .collect();
            Arc::new(Expr::FnCall(expr.typ(), f.clone(), args))
        }
        Expr::OvCall(_, _, _args) => todo!(),
    }
}
