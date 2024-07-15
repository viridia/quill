use std::fmt::{Error, Write};

use bevy::{prelude::*, utils::HashSet};

/// Structure which contains all of the parts of a shader source.
pub struct ShaderAssembly {
    /// Entity for the graph node that defines this shader.
    node: Entity,

    /// Set of imports for the shader.
    imports: HashSet<String>,
}

impl ShaderAssembly {
    /// Create a new shader assembly.
    pub fn new(node: Entity) -> Self {
        Self {
            node,
            imports: HashSet::default(),
        }
    }

    /// Add an import depdenency to the shader.
    pub fn add_import(&mut self, name: String) {
        self.imports.insert(name);
    }

    /// Generate the source code for the shader.
    pub fn gen_source(&self) -> Result<String, Error> {
        let mut out = String::new();
        self.prelude(&mut out)?;
        self.imports(&mut out)?;
        // let attribs = self.attribs();
        // let uniforms = self.uniforms();
        // let main = self.main();
        self.fragment(&mut out)?;

        // [prelude, imports, attribs, uniforms, main].join("\n")
        Ok(out)
    }

    /// Get the prelude for the shader.
    fn prelude(&self, out: &mut String) -> Result<(), Error> {
        out.write_str("#import bevy_ui::ui_vertex_output::UiVertexOutput\n")?;
        // [
        //     "#version 300 es",
        //     "precision mediump float;",
        //     "",
        //     "// Shader for node",
        //     "",
        // ]
        // .join("\n")
        Ok(())
    }

    /// Get the imports for the shader.
    fn imports(&self, out: &mut String) -> Result<(), Error> {
        let mut result = String::new();
        for name in &self.imports {
            out.write_str(name)?;
            out.write_char('\n')?;
            // let chunk = by_name(name);
            // if chunk.is_none() {
            //     panic!("Invalid shader fragment: {}", name);
            // }

            // result.push_str(&format!("// Imported from {}.glsl\n", name));
            // result.push_str(&chunk.unwrap());
        }

        Ok(())
    }

    /// Get the attributes for the shader.
    // fn attribs(&self) -> String {
    //     ["in highp vec2 vTextureCoord;", "out vec4 fragColor;", ""].join("\n")
    // }

    /// Get the uniforms for the shader.
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

    /// Get the main function for the shader.
    // fn fragment(&self) -> String {
    //     ["void fragment() {", self.body(), "}"].join("\n")
    // }

    // / Get the body of the shader.
    fn fragment(&self, out: &mut String) -> Result<(), Error> {
        out.write_str(
            "\
@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4(vertex.position, 1.0));
    out.position = position_world_to_clip(out.world_position.xyz);
    return vertex_output;
}\n\n",
        )?;
        out.write_str("@fragment\n")?;
        out.write_str("fn fragment(\n")?;
        out.write_str("    @builtin(front_facing) is_front: bool,\n")?;
        out.write_str("    mesh: VertexOutput,\n")?;
        out.write_str(") -> @location(0) vec4<f32> {\n")?;
        out.write_str("    return vec4<f32>(1.0, 0.0, 0.0, 1.0);\n")?;
        out.write_str("}\n")?;
        Ok(())
    }

    //     let mut stmts = Vec::new();
    //     let result = self.node.read_output_value(self.node.outputs[]
}

// import { DataType } from '../operators';
// import { Expr, assign, refLocal } from './Expr';
// import { GraphNode } from '../graph';
// import { byName } from '../operators/library/shaders';
// import { lowerExprs } from './pass/transform';
// import { generate } from './pass/generate';
// import { printToString } from './codefmt/print';

// /** Observer that regenerates a shader when the inputs change. */
// export class ShaderAssembly {
//   constructor(private node: GraphNode) {}

//   public dispose() {}

//   public get source(): string {
//     return [...this.prelude, ...this.imports, ...this.attribs, ...this.uniforms, ...this.main].join(
//       '\n'
//     );
//   }

//   private get prelude(): string[] {
//     return [
//       '#version 300 es',
//       'precision mediump float;',
//       '',
//       `// Shader for ${this.node.operator.name}`,
//       '',
//     ];
//   }

//   private get imports(): string[] {
//     const result: string[] = [];
//     this.node.transitiveImports.forEach(name => {
//       const chunk = byName[name];
//       if (!chunk) {
//         throw Error(`Invalid shader fragment: ${name}`);
//       }

//       result.push(`// Imported from ${name}.glsl`);
//       result.push(chunk);
//     });

//     return result;
//   }

//   private get attribs(): string[] {
//     return ['in highp vec2 vTextureCoord;', 'out vec4 fragColor;', ''];
//   }

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

//   private get main(): string[] {
//     return ['void main() {', this.body, '}'];
//   }

//   private get body(): string {
//     // Get expressions
//     const stmts: Expr[] = [];
//     const result = this.node.readOutputValue(this.node.outputs[0], stmts);

//     // Transform expessions
//     lowerExprs(assign(refLocal('fragColor', DataType.VEC4), result), stmts);

//     return printToString(stmts.map(generate), {
//       maxWidth: 100,
//       initialIndent: 1,
//     });
//   }
// }
