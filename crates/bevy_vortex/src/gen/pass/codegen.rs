use crate::gen::{output_chunk::OutputChunk, Expr};

pub fn codegen(expr: &Expr) -> OutputChunk {
    match expr {
        Expr::LiteralStr(_) => todo!(),
        Expr::ConstI32(n) => OutputChunk::Literal(n.to_string()),
        Expr::ConstF32(f) => {
            let f = f.to_string();
            if f.contains('.') {
                OutputChunk::Literal(f)
            } else {
                OutputChunk::Literal(format!("{}.", f))
            }
        }
        Expr::ConstVec2(v) => OutputChunk::FCall {
            func: "vec2f",
            args: vec![codegen(&Expr::ConstF32(v.x)), codegen(&Expr::ConstF32(v.y))],
        },
        Expr::ConstVec3(v) => OutputChunk::FCall {
            func: "vec3f",
            args: vec![
                codegen(&Expr::ConstF32(v.x)),
                codegen(&Expr::ConstF32(v.y)),
                codegen(&Expr::ConstF32(v.z)),
            ],
        },
        Expr::ConstVec4(v) => OutputChunk::FCall {
            func: "vec4f",
            args: vec![
                codegen(&Expr::ConstF32(v.x)),
                codegen(&Expr::ConstF32(v.y)),
                codegen(&Expr::ConstF32(v.z)),
                codegen(&Expr::ConstF32(v.w)),
            ],
        },
        Expr::ConstColor(color) => OutputChunk::FCall {
            func: "vec4f",
            args: vec![
                codegen(&Expr::ConstF32(color.red)),
                codegen(&Expr::ConstF32(color.green)),
                codegen(&Expr::ConstF32(color.blue)),
                codegen(&Expr::ConstF32(color.alpha)),
            ],
        },
        Expr::LocalDefn(_, _, _) => todo!(),
        Expr::Assign(_, _, _) => todo!(),
        Expr::RefLocal(_, name) => OutputChunk::Literal(name.clone()),
        Expr::RefInput(_, _) => todo!(),
        Expr::RefUniform(_, _) => todo!(),
        Expr::TypeCast(_, _) => unreachable!("TypeCast should have been lowered"),
        Expr::GetAttr(_, expr, fieldname) => OutputChunk::Infix {
            oper: ".".to_string(),
            precedence: 1,
            args: vec![codegen(expr), OutputChunk::Str(fieldname)],
        },
        Expr::BinOp(_, _, _, _) => todo!(),
        Expr::FnCall(_, f, args) => OutputChunk::FCall {
            func: f,
            args: args.iter().map(|arg| codegen(arg)).collect(),
        },
        Expr::OvCall(_, _, _) => todo!(),
    }
}
