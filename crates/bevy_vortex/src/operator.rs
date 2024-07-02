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
pub trait Operator: Send + Sync {
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
