// import { DataType } from '../operators';
// import { GraphNode } from '../graph';
// import { FunctionDefn, OverloadDefn } from '../operators/FunctionDefn';

use bevy::{
    color::LinearRgba,
    math::{Vec2, Vec3},
};

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    I32,
    F32,
    Vec2,
    Vec3,
    Vec4,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl BinOp {
    pub fn precedence(&self) -> Precedence {
        match self {
            BinOp::Add | BinOp::Sub => Precedence::AddSub,
            BinOp::Mul | BinOp::Div | BinOp::Mod => Precedence::MulDiv,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Group,
    AddSub,
    MulDiv,
    Relational,
    Eq,
    And,
    Xor,
    Or,
    Assign,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    LiteralStr(String),
    ConstI32(f32),
    ConstF32(f32),
    ConstVec2(Vec2),
    ConstVec3(Vec3),
    ConstColor(LinearRgba),
    LocalDefn(DataType, String, Option<Box<Expr>>),
    Assign(DataType, String, Box<Expr>),
    RefLocal(DataType, String),
    RefInput(DataType, String),
    RefUnform(DataType, String),
    TypeCast(DataType, Box<Expr>),
    GetAttr(DataType, Box<Expr>, String),
    BinOp(DataType, Box<Expr>, BinOp),
}

// export type ExprKind =
//   | 'call'
//   | 'ovcall'
//   | 'op'
//   | 'deflocal'
//   | 'reflocal'
//   | 'refuniform'
//   | 'refinput'
//   | 'reftexcoords'
//   | 'literal'
//   | 'typecast'
//   | 'getattr'
//   | 'binop'
//   | 'fork';

// export type BinaryOperator = 'add' | 'sub' | 'mul' | 'div';

// /** An function call expression. */
// export interface CallExpr extends BaseExpr {
//   kind: 'call';
//   callable: FunctionDefn;
//   args: ExprOrLiteral[];
// }

// /** Constructor for function call expression. */
// export const call = (callable: FunctionDefn, ...args: ExprOrLiteral[]): CallExpr => ({
//   kind: 'call',
//   args,
//   type: callable.type.length > 1 ? DataType.OTHER : callable.type[0].result,
//   callable,
// });

// /** An overloaded function call expression, used when the overload hasn't been decided yet. */
// export interface OverloadCallExpr extends BaseExpr {
//   kind: 'ovcall';
//   callable: OverloadDefn;
//   args: Expr[];
// }

// export const ovcall = (callable: OverloadDefn, ...args: Expr[]): OverloadCallExpr => ({
//   kind: 'ovcall',
//   args,
//   type: callable.type.result,
//   callable,
// });

// /** Reading a local variable. */
// export interface RefLocalExpr extends BaseExpr {
//   kind: 'reflocal';
//   name: string;
//   type: DataType;
// }

// /** Constructor for reading a local variable. */
// export const refLocal = (name: string, type: DataType): RefLocalExpr => ({
//   kind: 'reflocal',
//   name,
//   type,
// });

// /** Reading a node parameter. */
// export interface RefUniformExpr extends BaseExpr {
//   kind: 'refuniform';
//   name: string;
//   type: DataType;
//   node: GraphNode;
// }

// /** Constructor for reading a node parameter. */
// export const refUniform = (name: string, type: DataType, node: GraphNode): RefUniformExpr => ({
//   kind: 'refuniform',
//   name,
//   type,
//   node,
// });

// /** Reading an input terminal. */
// export interface RefInputExpr extends BaseExpr {
//   kind: 'refinput';
//   name: string;
//   type: DataType;
//   node: GraphNode;
//   uv: Expr;
// }

// /** Constructor for reading from an input terminal. */
// export const refInput = (
//   name: string,
//   type: DataType,
//   node: GraphNode,
//   uv: Expr
// ): RefInputExpr => ({
//   kind: 'refinput',
//   name,
//   type,
//   node,
//   uv,
// });

// export interface RefTexCoordsExpr extends BaseExpr {
//   kind: 'reftexcoords';
//   type: DataType;
// }

// export const refTexCoords = (): RefTexCoordsExpr => ({
//   kind: 'reftexcoords',
//   type: DataType.VEC2,
// });

// export interface LiteralNode extends BaseExpr {
//   kind: 'literal';
//   value: string;
// }

// export const literal = (value: string, type: DataType): LiteralNode => ({
//   kind: 'literal',
//   value,
//   type,
// });

// export interface TypeCastExpr extends BaseExpr {
//   kind: 'typecast';
//   value: Expr;
// }

// export const typeCast = (value: Expr, type: DataType): TypeCastExpr => ({
//   kind: 'typecast',
//   value,
//   type,
// });

// export interface GetAttrExpr extends BaseExpr {
//   kind: 'getattr';
//   base: Expr;
//   name: string;
//   type: DataType;
// }

// export const getAttr = (base: Expr, name: string, type: DataType): GetAttrExpr => ({
//   kind: 'getattr',
//   base,
//   name,
//   type,
// });

// export interface BinaryOpExpr extends BaseExpr {
//   kind: 'binop';
//   op: BinaryOperator;
//   left: Expr;
//   right: Expr;
//   type: DataType;
// }

// export const binop = (
//   op: BinaryOperator,
//   left: Expr,
//   right: Expr,
//   type: DataType
// ): BinaryOpExpr => ({
//   kind: 'binop',
//   op,
//   left,
//   right,
//   type,
// });

// export const add = (left: Expr, right: Expr, type: DataType): BinaryOpExpr => ({
//   kind: 'binop',
//   op: 'add',
//   left,
//   right,
//   type,
// });

// export const multiply = (left: Expr, right: Expr, type: DataType): BinaryOpExpr => ({
//   kind: 'binop',
//   op: 'mul',
//   left,
//   right,
//   type,
// });

// export const subtract = (left: Expr, right: Expr, type: DataType): BinaryOpExpr => ({
//   kind: 'binop',
//   op: 'sub',
//   left,
//   right,
//   type,
// });

// export const divide = (left: Expr, right: Expr, type: DataType): BinaryOpExpr => ({
//   kind: 'binop',
//   op: 'div',
//   left,
//   right,
//   type,
// });

// /** A 'fork' is a hint to the code generator that we are going to use the expression
//     multiple times, and we might not want to re-evaluate it, so store it in a local
//     variable if needed.
//  */
// export interface ForkExpr extends BaseExpr {
//   kind: 'fork';
//   value: Expr;
//   name: string;
//   key: symbol;
//   type: DataType;
// }

// export const fork = (value: Expr, name: string): ForkExpr => ({
//   kind: 'fork',
//   value,
//   name,
//   key: Symbol('expr'),
//   type: value.type,
// });

// export type Expr =
//   | AssignExpr
//   | CallExpr
//   | OverloadCallExpr
//   | LocalDefn
//   | RefLocalExpr
//   | RefUniformExpr
//   | RefInputExpr
//   | RefTexCoordsExpr
//   | LiteralNode
//   | TypeCastExpr
//   | GetAttrExpr
//   | BinaryOpExpr
//   | ForkExpr;

// export type ExprOrLiteral = Expr | string | number;

// export interface CallFactory {
//   (...args: ExprOrLiteral[]): CallExpr;
//   defn: FunctionDefn;
// }

// export function defineFn(defn: FunctionDefn): CallFactory {
//   function fn(...args: ExprOrLiteral[]): CallExpr {
//     return {
//       kind: 'call',
//       args,
//       type: defn.type.length === 1 ? defn.type[0].result : DataType.OTHER,
//       callable: defn,
//     };
//   }
//   fn.defn = defn;
//   return fn;
// }
