use std::{
    fmt::{Error, Write},
    sync::Arc,
};

use bevy::prelude::*;

use super::{
    output_chunk::{LineWrapping, OutputChunk},
    pass::{codegen, lower_typecasts},
    shader_imports::ShaderImports,
    Expr,
};

struct ShaderLocalVar {
    name: String,
    data_type: super::DataType,
    mutable: bool,
    init: Arc<Expr>,
}

/// Structure which contains all of the parts of a shader source.
pub struct ShaderAssembly {
    /// Name of this shader.
    name: String,

    /// The expression that represents the return value of the fragment shader
    fragment_value: Arc<Expr>,

    /// Generated code for fragment shader.
    source: String,

    /// Set of imports for the shader.
    imports: ShaderImports,

    /// Code snippets that are included in the shader module.
    includes: Vec<&'static str>,

    /// Local variable definitions. Note we use a vector here so we can preserve declaration order.
    locals: Vec<ShaderLocalVar>,

    /// Whether the fragment shader needs position information.
    pub(crate) needs_position: bool,

    /// Whether the fragment shader needs normal information.
    pub(crate) needs_normal: bool,

    /// Whether the fragment shader needs texture coordinates.
    pub(crate) needs_uv: bool,
}

impl ShaderAssembly {
    /// Create a new shader assembly.
    pub fn new(name: String) -> Self {
        Self {
            name,
            fragment_value: Arc::new(Expr::ConstColor(LinearRgba::WHITE)),
            source: String::new(),
            imports: ShaderImports::default(),
            includes: Vec::new(),
            locals: Vec::new(),
            needs_position: false,
            needs_normal: false,
            needs_uv: false,
        }
    }

    /// Include a utility function in the shader.
    pub fn add_include(&mut self, include: &'static str) {
        if !self.includes.contains(&include) {
            self.includes.push(include);
        }
    }

    pub fn add_common_imports(&mut self) {
        self.add_import("bevy_pbr::mesh_functions");
        self.add_import("bevy_pbr::view_transformations::position_world_to_clip");
    }

    /// Add an import depdenency to the shader.
    pub fn add_import(&mut self, name: &'static str) {
        self.imports.add(name);
    }

    /// Set the expression that represents the return value of the fragment shader.
    pub fn set_fragment_value(&mut self, value: Arc<Expr>) {
        self.fragment_value = value;
    }

    /// Declare a local variable in the shader.
    /// Need to return:
    /// Whether the var already exists
    /// a way to initialize it.
    pub fn declare_local(
        &mut self,
        name: String,
        data_type: super::DataType,
        mutable: bool,
        init: Arc<Expr>,
    ) {
        self.locals.push(ShaderLocalVar {
            name,
            data_type,
            mutable,
            init,
        });
    }

    /// Return true if the given local variable has already been declared.
    pub fn local_exists(&self, name: &String) -> bool {
        self.locals.iter().any(|var| var.name == *name)
    }

    /// Return the source code for the shader.
    pub fn source(&self) -> &str {
        self.source.as_str()
    }

    /// Run transpilation passes
    pub fn run_passes(&mut self) -> Result<(), Error> {
        let mut source = String::new();
        source.write_fmt(format_args!("// Shader for {}\n\n", self.name))?;

        // Write imports
        self.imports.write(&mut source)?;
        source.write_str("\n")?;

        // Write vertex input format
        source.write_str("struct Vertex {\n")?;
        source.write_str("    @builtin(instance_index) instance_index: u32,\n")?;
        source.write_str("    @location(0) position: vec3<f32>,\n")?;
        if self.needs_normal {
            source.write_str("    @location(1) normal: vec3<f32>,\n")?;
        }
        if self.needs_uv {
            source.write_str("    @location(2) uv: vec2<f32>,\n")?;
        }
        source.write_str("};\n\n")?;

        // Write vertex output format
        source.write_str("struct VertexOutput {\n")?;
        source.write_str("    @builtin(position) position: vec4<f32>,\n")?;
        source.write_str("    @location(0) world_position: vec4<f32>,\n")?;
        if self.needs_normal {
            source.write_str("    @location(1) world_normal: vec3<f32>,\n")?;
        }
        if self.needs_uv {
            source.write_str("    @location(2) uv: vec2<f32>,\n")?;
        }
        if self.needs_position {
            source.write_str("    @location(3) position_local: vec3<f32>,\n")?;
        }
        source.write_str("};\n\n")?;

        // Write vertex shader
        source.write_str("@vertex\n")?;
        source.write_str("fn vertex(vertex: Vertex) -> VertexOutput {\n")?;
        source.write_str("    var out: VertexOutput;\n")?;
        source.write_str("    var world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);\n")?;
        source.write_str("    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4(vertex.position, 1.0));\n")?;
        source.write_str("    out.position = position_world_to_clip(out.world_position.xyz);\n")?;
        if self.needs_normal {
            source.write_str(
                "    out.world_normal = mesh_functions::mesh_normal_local_to_world(\n",
            )?;
            source.write_str("        vertex.normal,\n")?;
            source.write_str("        vertex.instance_index\n")?;
            source.write_str("    );\n")?;
        }
        if self.needs_uv {
            source.write_str("    out.uv = vertex.uv;\n")?;
        }
        if self.needs_position {
            source.write_str("    out.position_local = vertex.position;\n")?;
        }
        source.write_str("    return out;\n")?;
        source.write_str("}\n\n")?;

        // Write fragment shader
        source.write_str("@fragment\n")?;
        source.write_str("fn fragment(\n")?;
        source.write_str("    @builtin(front_facing) is_front: bool,\n")?;
        source.write_str("    mesh: VertexOutput,\n")?;
        source.write_str(") -> @location(0) vec4<f32> {\n")?;

        let mut wrap = LineWrapping::new(100);
        wrap.indent();

        // Write local variables
        for local in &self.locals {
            wrap.write_indent(&mut source)?;
            let init = lower_typecasts(Arc::new(Expr::TypeCast(
                local.data_type,
                local.init.clone(),
            )));
            let chunk = OutputChunk::Stmt(vec![
                OutputChunk::Str(if local.mutable { "var" } else { "let" }),
                OutputChunk::Literal(local.name.clone()),
                OutputChunk::Str("="),
                codegen(init.as_ref()),
            ]);
            chunk.format(&mut source, &mut wrap)?;
        }

        // Write return result
        let out = OutputChunk::Ret(Box::new(codegen(
            lower_typecasts(Arc::new(Expr::TypeCast(
                super::DataType::LinearRgba,
                self.fragment_value.clone(),
            )))
            .as_ref(),
        )));

        wrap.write_indent(&mut source)?;
        out.format(&mut source, &mut wrap)?;
        source.write_str("\n")?;
        source.write_str("}\n")?;

        // Add includes
        for include in &self.includes {
            source.write_char('\n')?;
            source.write_str(include)?;
        }

        // println!("Shader source:\n{}", source);
        self.source = source;
        Ok(())
    }

    // / Get the uniforms for the shader.
    // fn uniforms(&self) -> String {
    //     let mut result = String::new();
    //     // let mut visited_nodes = HashSet::default();
    //     // self.node.visit_upstream_nodes(|node| {
    //     //     if !visited_nodes.contains(node) {
    //     //         visited_nodes.insert(node);
    //     //         result.push_str(&node.uniforms());
    //     //     }
    //     // });
    //     // result.push_str(&self.node.uniforms());
    //     result
    // }
}

//   /** List of uniform declarations. */
//   private get uniforms(): string[] {
//     const result: string[][] = [];
//     const visitedNodes = new Set<GraphNode>();
//     this.node.visitUpstreamNodes(node => {
//       if (!visitedNodes.has(node)) {
//         visitedNodes.add(node);
//         result.push(node.uniforms);
//       }
//     });
//     return ([] as string[]).concat(...result, this.node.uniforms);
//   }
