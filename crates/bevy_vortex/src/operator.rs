use std::ops::RangeInclusive;

use bevy::{
    reflect::{reflect_trait, Reflect},
    utils::hashbrown::HashSet,
};

/// Groupings for operators
#[derive(Debug, Clone, Reflect, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperatorCategory {
    Input,
    Output,
    Pattern,
    Generator,
    Filter,
    Converter,
}

impl OperatorCategory {
    pub fn to_local_string(&self) -> &'static str {
        match self {
            OperatorCategory::Input => "Input",
            OperatorCategory::Output => "Output",
            OperatorCategory::Pattern => "Pattern",
            OperatorCategory::Generator => "Generator",
            OperatorCategory::Filter => "Filter",
            OperatorCategory::Converter => "Converter",
        }
    }
}

/// Defines an operational component in a node graph.
#[reflect_trait]
pub trait Operator: Send + Sync + Reflect {
    /// Clone the implementation of this operator and return it as a boxed trait object.
    fn to_boxed_clone(&self) -> Box<dyn Operator>;

    /// Returns the names of all .wgsl imports needed for this operator to compile.
    fn get_imports(&self) -> HashSet<String> {
        HashSet::default()
    }

    //   /** Return the expression for this node. */
    //   public getCode(node: GraphNode, terminal: OutputTerminal, prologue: Expr[]): Expr {
    //     return vec4(0, 0, 0, 1);
    //   }

    /// Generate code for this operator.
    fn gen(&self);
}

/// Width of the operator node, in pixels.
#[derive(Debug, Clone, Reflect)]
pub struct DisplayWidth(pub i32);

/// Name of this operator or operator property.
#[derive(Debug, Clone, Reflect)]
pub struct DisplayName(pub &'static str);

/// Name of this operator or operator property.
#[derive(Debug, Clone, Reflect)]
pub struct OperatorClass(pub OperatorCategory);

/// Description / documentation for the operator.
#[derive(Debug, Clone, Reflect)]
pub struct OperatorDescription(pub &'static str);

/// Indicates that an operator property is an input terminal.
#[derive(Debug, Clone, Reflect)]
pub struct OperatorInput;

/// Indicates that an operator property is an output terminal.
#[derive(Debug, Clone, Reflect)]
pub struct OperatorOutput;

/// Indicates that an operator input terminal should not have an editable widget when not
/// connected.
#[derive(Debug, Clone, Reflect)]
pub struct OperatorInputOnly;

/// An attribute that specifies the minimum and maximum allowed values for a field.
/// This range is inclusive.
///
/// This attribute can be applied to numeric fields. It can also be applied to aggregate types
/// that have a numeric type parameter, such as an `Option<f32>` or `Vec<i8>`.
#[derive(Debug, Clone, Reflect)]
pub struct OpValueRange<T>(pub RangeInclusive<T>);

/// An attribute that specifies how many decimal digits of precision should be allowed.
/// If the field is an integer, this will be ignored. If present, field values will be
/// rounded to the nearest value with the specified number of decimal digits.
///
/// This attribute can be applied to numeric fields. It can also be applied to aggregate types
/// that have a numeric type parameter, such as an `Option<f32>` or `Vec<i8>`.
#[derive(Debug, Clone, Reflect)]
pub struct OpValuePrecision(pub usize);

/// An attribute that specifies the increment and decrement step size for a numeric field.
/// If not present, the step size will be determined from the precision. If the precision is
/// not present, a heuristic will be used based on the range.
///
/// This attribute can be applied to numeric fields. It can also be applied to aggregate types
/// that have a numeric type parameter, such as an `Option<f32>` or `Vec<i8>`.
#[derive(Debug, Clone, Reflect)]
pub struct OpValueStep<T>(pub T);
