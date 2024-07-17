use std::fmt::{Error, Write};

use bevy::utils::HashMap;

/// Struct that manages de-duping of import depdenencies.
#[derive(Debug, Default)]
pub struct ShaderImports(ImportTreeNode);

#[derive(Debug, Default)]
struct ImportTreeNode {
    /// A terminal node is the end of a path. Note that a node can be both terminal and also
    /// have children if it was imported multiple times.
    terminal: bool,

    /// The sub-paths of this node.
    children: HashMap<&'static str, ImportTreeNode>,
}

impl ImportTreeNode {
    fn add(&mut self, path: &[&'static str]) {
        let mut node = self;
        for segment in path {
            node = node.children.entry(segment).or_default();
        }
        node.terminal = true;
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        if self.children.is_empty() {
            return Ok(());
        }
        writer.write_str("::")?;
        let mut keys = self.children.keys().collect::<Vec<_>>();
        keys.sort();
        if keys.len() == 1 && !self.terminal {
            let node = self.children.get(keys[0]).unwrap();
            writer.write_str(keys[0])?;
            node.write(writer)?;
        } else {
            let mut sep = false;
            writer.write_char('{')?;
            if self.terminal {
                writer.write_str("self")?;
                sep = true;
            }
            for key in keys.iter() {
                if sep {
                    writer.write_str(", ")?;
                }
                writer.write_str(key)?;
                self.children.get(*key).unwrap().write(writer)?;
                sep = true;
            }
            writer.write_char('}')?;
        }
        Ok(())
    }
}

impl ShaderImports {
    pub fn add(&mut self, path: &'static str) {
        let segments = path.split("::").collect::<Vec<_>>();
        self.0.add(segments.as_slice());
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        let mut keys = self.0.children.keys().collect::<Vec<_>>();
        keys.sort();
        for key in keys.iter() {
            let node = self.0.children.get(*key).unwrap();
            writer.write_str("#import ")?;
            writer.write_str(key)?;
            node.write(writer)?;
            writer.write_str(";\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_single() {
        let mut si = ShaderImports::default();
        si.add("path::to::file");
        assert_eq!(si.0.children.len(), 1);

        let mut out = String::new();
        si.write(&mut out).unwrap();
        assert_eq!(out, "#import path::to::file;\n");
    }

    #[test]
    fn test_insert_disjoint() {
        let mut si = ShaderImports::default();
        si.add("path::to::file");
        si.add("other::to::file");
        assert_eq!(si.0.children.len(), 2);

        let mut out = String::new();
        si.write(&mut out).unwrap();
        assert_eq!(out, "#import other::to::file;\n#import path::to::file;\n");
    }

    #[test]
    fn test_insert_overlap() {
        let mut si = ShaderImports::default();
        si.add("path::to::file");
        si.add("path::to::other");
        assert_eq!(si.0.children.len(), 1);

        let mut out = String::new();
        si.write(&mut out).unwrap();
        assert_eq!(out, "#import path::to::{file, other};\n");
    }

    #[test]
    fn test_insert_self() {
        let mut si = ShaderImports::default();
        si.add("path::to::file");
        si.add("path::to::file::other");
        assert_eq!(si.0.children.len(), 1);

        let mut out = String::new();
        si.write(&mut out).unwrap();
        assert_eq!(out, "#import path::to::file::{self, other};\n");
    }
}
