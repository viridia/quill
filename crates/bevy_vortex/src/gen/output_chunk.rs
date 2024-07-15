use std::fmt::{Error, Write};

const INDENT_SIZE: usize = 4;

/// Struct which keeps track of the current line length and indentation level, and can break lines.
pub struct LineWrapping {
    /// The current length of the current line.
    pub line_length: usize,
    /// The maximum length of a line before it should be wrapped.
    pub max_line_length: usize,
    /// The current indentation level. Each indent level is `INDENT_SIZE` spaces.
    pub current_line_indent: usize,
    /// The indentation level to be used for the next line.
    pub next_line_indent: usize,
}

impl LineWrapping {
    pub fn new(max_line_length: usize) -> Self {
        Self {
            line_length: 0,
            max_line_length,
            current_line_indent: 0,
            next_line_indent: 0,
        }
    }

    pub fn indent(&mut self) {
        // Max 16 levels of indentation. After that we just stop indenting further.
        if self.current_line_indent < 16 {
            self.current_line_indent += 1;
        }
    }

    pub fn break_line<W: Write>(&mut self, out: &mut W) -> Result<(), Error> {
        out.write_char('\n')?;
        self.current_line_indent = self.next_line_indent;
        for _ in 0..(self.current_line_indent * INDENT_SIZE) {
            out.write_char(' ')?;
        }
        self.line_length = self.current_line_indent * INDENT_SIZE;
        Ok(())
    }

    pub fn can_fit(&self, length: usize) -> bool {
        self.current_line_indent * INDENT_SIZE + self.line_length + length < self.max_line_length
    }
}

/// Represents a chunk of output code with predifined locations for line-breaking / wrapping.
pub enum OutputChunk {
    /// A literal string.
    Literal(String),
    /// A literal string (static).
    Str(&'static str),
    /// A sequence of chunks which are concatenated together with no separators or spaces.
    Concat(Vec<OutputChunk>),
    /// A sequence of comma-separated expressions, surrounded by parentheses.
    Parens(Vec<OutputChunk>),
    /// A sequence of comma-separated expressions, surrounded by brackets.
    Brackets(Vec<OutputChunk>),
    /// A statement block.
    Stmt(Vec<OutputChunk>),
    /// A return statement.
    Ret(Vec<OutputChunk>),
    /// An infix operator.
    Infix {
        oper: String,
        precedence: usize,
        args: Vec<OutputChunk>,
    },
    /// A function call.
    FCall {
        func: String,
        args: Vec<OutputChunk>,
    },
}

impl From<String> for OutputChunk {
    fn from(s: String) -> Self {
        OutputChunk::Literal(s)
    }
}

impl From<&'static str> for OutputChunk {
    fn from(s: &'static str) -> Self {
        OutputChunk::Str(s)
    }
}

impl OutputChunk {
    /// The total length, in characters, of this chunk and all its descendants.
    pub fn length(&self) -> usize {
        match self {
            OutputChunk::Literal(s) => s.len(),
            OutputChunk::Str(s) => s.len(),
            OutputChunk::Concat(chunks) => chunks.iter().map(|c| c.length()).sum(),
            OutputChunk::Parens(chunks) | OutputChunk::Brackets(chunks) => {
                if chunks.is_empty() {
                    2
                } else {
                    2 + 2 * (chunks.len() - 1) + chunks.iter().map(|c| c.length()).sum::<usize>()
                }
            }
            OutputChunk::Stmt(chunks) => {
                if chunks.is_empty() {
                    1
                } else {
                    1 + (chunks.len() - 1) + chunks.iter().map(|c| c.length()).sum::<usize>()
                }
            }
            OutputChunk::Ret(chunks) => {
                "return ".len() + chunks.iter().map(|c| c.length()).sum::<usize>() + 1
            }
            OutputChunk::Infix {
                oper,
                args,
                precedence: _,
            } => {
                args.iter().map(|c| c.length()).sum::<usize>() + (args.len() - 1) * (oper.len() + 2)
            }
            OutputChunk::FCall { func, args } => {
                func.len() + args.iter().map(|c| c.length()).sum::<usize>() + 2 * args.len()
            }
        }
    }

    /// The number of characters before the first line-break point.
    pub fn head_length(&self) -> usize {
        match self {
            OutputChunk::Literal(s) => s.len(),
            OutputChunk::Str(s) => s.len(),
            OutputChunk::Concat(chunks) => chunks[0].head_length(),
            OutputChunk::Parens(_) => 1,
            OutputChunk::Brackets(_) => 1,
            OutputChunk::Stmt(chunks) => chunks[0].head_length(),
            OutputChunk::Ret(_) => "return".len(),
            OutputChunk::Infix {
                oper: _,
                args,
                precedence: _,
            } => args[0].head_length(),
            OutputChunk::FCall { func, args: _ } => func.len() + 1,
        }
    }

    /// Convert this chunk and it's descendants into a flat string, with no line breaks.
    pub fn flatten<W: Write>(&self, out: &mut W) -> Result<(), Error> {
        match self {
            OutputChunk::Literal(s) => out.write_str(s)?,
            OutputChunk::Str(s) => out.write_str(s)?,
            OutputChunk::Concat(chunks) => {
                for chunk in chunks {
                    chunk.flatten(out)?;
                }
            }
            OutputChunk::Parens(chunks) => {
                out.write_char('(')?;
                for (i, chunk) in chunks.iter().enumerate() {
                    if i > 0 {
                        out.write_str(", ")?;
                    }
                    chunk.flatten(out)?;
                }
                out.write_char(')')?;
            }
            OutputChunk::Brackets(chunks) => {
                out.write_char('[')?;
                for (i, chunk) in chunks.iter().enumerate() {
                    if i > 0 {
                        out.write_str(", ")?;
                    }
                    chunk.flatten(out)?;
                }
                out.write_char(']')?;
            }
            OutputChunk::Stmt(chunks) => {
                for (i, chunk) in chunks.iter().enumerate() {
                    if i > 0 {
                        out.write_char(' ')?;
                    }
                    chunk.flatten(out)?;
                }
                out.write_char(';')?;
            }
            OutputChunk::Ret(chunks) => {
                out.write_str("return ")?;
                for chunk in chunks {
                    chunk.flatten(out)?;
                }
                out.write_char(';')?;
            }
            OutputChunk::Infix {
                oper,
                args,
                precedence: _,
            } => {
                for (i, chunk) in args.iter().enumerate() {
                    if i > 0 {
                        out.write_char(' ')?;
                        out.write_str(oper)?;
                        out.write_char(' ')?;
                    }
                    chunk.flatten(out)?;
                }
            }
            OutputChunk::FCall { func, args } => {
                out.write_str(func)?;
                out.write_char('(')?;
                for (i, chunk) in args.iter().enumerate() {
                    if i > 0 {
                        out.write_str(", ")?;
                    }
                    chunk.flatten(out)?;
                }
                out.write_char(')')?;
            }
        }
        Result::Ok(())
    }

    pub fn format<W: Write>(&self, out: &mut W, wrap: &mut LineWrapping) -> Result<(), Error> {
        let saved_indent = wrap.current_line_indent;
        match self {
            OutputChunk::Literal(str) => {
                wrap.line_length += str.len();
                self.flatten(out)
            }
            OutputChunk::Str(str) => {
                wrap.line_length += str.len();
                self.flatten(out)
            }
            OutputChunk::Concat(chunks) => {
                // Break line before each chunk that is not the first chunk.
                for (i, chunk) in chunks.iter().enumerate() {
                    if i > 0 {
                        wrap.line_length += 1;
                        if !wrap.can_fit(chunk.head_length() + 1) {
                            wrap.next_line_indent = saved_indent + 1;
                            wrap.break_line(out)?;
                        }
                    }
                    chunk.format(out, wrap)?;
                }
                wrap.current_line_indent = saved_indent;
                Ok(())
            }
            OutputChunk::Parens(chunks) => {
                wrap.line_length += 1;
                out.write_char('(')?;
                // Break line after initial delimiter, or after comma.
                for (i, chunk) in chunks.iter().enumerate() {
                    if wrap.can_fit(chunk.head_length() + 1) {
                        if i > 0 {
                            wrap.line_length += 2;
                            out.write_str(", ")?;
                        }
                    } else {
                        if i > 0 {
                            wrap.line_length += 1;
                            out.write_char(',')?;
                        }
                        wrap.next_line_indent = saved_indent + 1;
                        wrap.break_line(out)?;
                    }
                    chunk.format(out, wrap)?;
                }
                wrap.line_length += 1;
                wrap.current_line_indent = saved_indent;
                out.write_char(')')
            }
            OutputChunk::Brackets(chunks) => {
                wrap.line_length += 1;
                out.write_char('[')?;
                // Break line after initial delimiter, or after comma.
                for (i, chunk) in chunks.iter().enumerate() {
                    if wrap.can_fit(chunk.head_length() + 1) {
                        if i > 0 {
                            wrap.line_length += 2;
                            out.write_str(", ")?;
                        }
                    } else {
                        if i > 0 {
                            wrap.line_length += 1;
                            out.write_char(',')?;
                        }
                        wrap.next_line_indent = saved_indent + 1;
                        wrap.break_line(out)?;
                    }
                    chunk.format(out, wrap)?;
                }
                wrap.line_length += 1;
                wrap.current_line_indent = saved_indent;
                out.write_char(']')
            }
            OutputChunk::Stmt(chunks) => {
                // Break line before separators.
                for (i, chunk) in chunks.iter().enumerate() {
                    if i > 0 {
                        wrap.line_length += 1;
                        if wrap.can_fit(chunk.head_length() + 1) {
                            wrap.line_length += 1;
                            out.write_char(' ')?;
                        } else {
                            wrap.next_line_indent = saved_indent + 1;
                            wrap.break_line(out)?;
                        }
                    }
                    chunk.format(out, wrap)?;
                }
                wrap.current_line_indent = saved_indent;
                Ok(())
            }
            OutputChunk::Ret(chunks) => todo!(),
            OutputChunk::Infix {
                oper,
                precedence,
                args,
            } => todo!(),
            OutputChunk::FCall { func, args } => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head_length_literal() {
        let chunk = OutputChunk::Literal(String::from("Hello, world!"));
        assert_eq!(chunk.head_length(), 13);
    }

    #[test]
    fn test_head_length_str() {
        let chunk = OutputChunk::Str("Hello, world!");
        assert_eq!(chunk.head_length(), 13);
    }

    #[test]
    fn test_head_length_concat() {
        let chunk = OutputChunk::Concat(vec![
            OutputChunk::Literal(String::from("Hello")),
            OutputChunk::Literal(String::from(", ")),
            OutputChunk::Literal(String::from("world!")),
        ]);
        assert_eq!(chunk.head_length(), 5);
    }

    #[test]
    fn test_head_length_parens() {
        let chunk = OutputChunk::Parens(vec![
            OutputChunk::Literal(String::from("Hello")),
            OutputChunk::Literal(String::from(", ")),
            OutputChunk::Literal(String::from("world!")),
        ]);
        assert_eq!(chunk.head_length(), 1);
    }

    #[test]
    fn test_head_length_brackets() {
        let chunk = OutputChunk::Brackets(vec![
            OutputChunk::Literal(String::from("Hello")),
            OutputChunk::Literal(String::from(", ")),
            OutputChunk::Literal(String::from("world!")),
        ]);
        assert_eq!(chunk.head_length(), 1);
    }

    #[test]
    fn test_head_length_stmt() {
        let chunk = OutputChunk::Stmt(vec![
            OutputChunk::Literal(String::from("let")),
            OutputChunk::Literal(String::from("x")),
            OutputChunk::Literal(String::from("=")),
            OutputChunk::Literal(String::from("5")),
        ]);
        assert_eq!(chunk.head_length(), 3);
    }

    #[test]
    fn test_head_length_ret() {
        let chunk = OutputChunk::Ret(vec![
            OutputChunk::Literal(String::from("Hello")),
            OutputChunk::Literal(String::from("world!")),
        ]);
        assert_eq!(chunk.head_length(), "return".len());
    }

    #[test]
    fn test_head_length_infix() {
        let chunk = OutputChunk::Infix {
            oper: String::from("+"),
            precedence: 1,
            args: vec![
                OutputChunk::Literal(String::from("Hello")),
                OutputChunk::Literal(String::from("world!")),
            ],
        };
        assert_eq!(chunk.head_length(), 5);
    }

    #[test]
    fn test_head_length_fcall() {
        let chunk = OutputChunk::FCall {
            func: String::from("println"),
            args: vec![
                OutputChunk::Literal(String::from("Hello")),
                OutputChunk::Literal(String::from("world!")),
            ],
        };
        assert_eq!(chunk.head_length(), 8);
    }

    #[test]
    fn test_flatten_literal() {
        let chunk = OutputChunk::Literal(String::from("Hello, world!"));
        let mut output = String::new();
        chunk.flatten(&mut output).unwrap();
        assert_eq!(output, "Hello, world!");
        assert_eq!(output.len(), chunk.length());
    }

    #[test]
    fn test_flatten_str() {
        let chunk = OutputChunk::Str("Hello, world!");
        let mut output = String::new();
        chunk.flatten(&mut output).unwrap();
        assert_eq!(output, "Hello, world!");
        assert_eq!(output.len(), chunk.length());
    }

    #[test]
    fn test_flatten_concat() {
        let chunk = OutputChunk::Concat(vec![
            OutputChunk::Literal(String::from("Hello")),
            OutputChunk::Literal(String::from(", ")),
            OutputChunk::Literal(String::from("world!")),
        ]);
        let mut output = String::new();
        chunk.flatten(&mut output).unwrap();
        assert_eq!(output, "Hello, world!");
        assert_eq!(output.len(), chunk.length());
    }

    #[test]
    fn test_flatten_parens() {
        let chunk = OutputChunk::Parens(vec![
            OutputChunk::Literal(String::from("Hello")),
            OutputChunk::Literal(String::from("world!")),
        ]);
        let mut output = String::new();
        chunk.flatten(&mut output).unwrap();
        assert_eq!(output, "(Hello, world!)");
        assert_eq!(output.len(), chunk.length());
    }

    #[test]
    fn test_flatten_brackets() {
        let chunk = OutputChunk::Brackets(vec![
            OutputChunk::Literal(String::from("Hello")),
            OutputChunk::Literal(String::from("world!")),
        ]);
        let mut output = String::new();
        chunk.flatten(&mut output).unwrap();
        assert_eq!(output, "[Hello, world!]");
        assert_eq!(output.len(), chunk.length());
    }

    #[test]
    fn test_flatten_stmt() {
        let chunk = OutputChunk::Stmt(vec![
            OutputChunk::Literal(String::from("let")),
            OutputChunk::Literal(String::from("x")),
            OutputChunk::Literal(String::from("=")),
            OutputChunk::Literal(String::from("5")),
        ]);
        let mut output = String::new();
        chunk.flatten(&mut output).unwrap();
        assert_eq!(output, "let x = 5;");
        assert_eq!(output.len(), chunk.length());
    }

    #[test]
    fn test_flatten_ret() {
        let chunk = OutputChunk::Ret(vec![
            OutputChunk::Literal(String::from("Hello")),
            OutputChunk::Literal(String::from("world!")),
        ]);
        let mut output = String::new();
        chunk.flatten(&mut output).unwrap();
        assert_eq!(output, "return Helloworld!;");
        assert_eq!(output.len(), chunk.length());
    }

    #[test]
    fn test_flatten_infix() {
        let chunk = OutputChunk::Infix {
            oper: String::from("+"),
            precedence: 1,
            args: vec![
                OutputChunk::Literal(String::from("Hello")),
                OutputChunk::Literal(String::from("world!")),
            ],
        };
        let mut output = String::new();
        chunk.flatten(&mut output).unwrap();
        assert_eq!(output, "Hello + world!");
        assert_eq!(output.len(), chunk.length());
    }

    #[test]
    fn test_flatten_fcall() {
        let chunk = OutputChunk::FCall {
            func: String::from("println"),
            args: vec![
                OutputChunk::Literal(String::from("Hello")),
                OutputChunk::Literal(String::from("world!")),
            ],
        };
        let mut output = String::new();
        chunk.flatten(&mut output).unwrap();
        assert_eq!(output, "println(Hello, world!)");
        assert_eq!(output.len(), chunk.length());
    }

    #[test]
    fn test_format_parens() {
        let chunk = OutputChunk::Parens(vec![
            OutputChunk::Literal(String::from("Alpha")),
            OutputChunk::Literal(String::from("Beta")),
            OutputChunk::Literal(String::from("Gamma")),
        ]);
        let mut wrap = LineWrapping::new(80);
        let mut output = String::new();
        chunk.format(&mut output, &mut wrap).unwrap();
        assert_eq!(output, "(Alpha, Beta, Gamma)");

        let mut wrap = LineWrapping::new(16);
        let mut output = String::new();
        chunk.format(&mut output, &mut wrap).unwrap();
        assert_eq!(output, "(Alpha, Beta,\n    Gamma)");

        let mut wrap = LineWrapping::new(8);
        let mut output = String::new();
        chunk.format(&mut output, &mut wrap).unwrap();
        assert_eq!(output, "(Alpha,\n    Beta,\n    Gamma)");
    }

    #[test]
    fn test_format_brackets() {
        let chunk = OutputChunk::Brackets(vec![
            OutputChunk::Literal(String::from("Alpha")),
            OutputChunk::Literal(String::from("Beta")),
            OutputChunk::Literal(String::from("Gamma")),
        ]);
        let mut wrap = LineWrapping::new(80);
        let mut output = String::new();
        chunk.format(&mut output, &mut wrap).unwrap();
        assert_eq!(output, "[Alpha, Beta, Gamma]");

        let mut wrap = LineWrapping::new(16);
        let mut output = String::new();
        chunk.format(&mut output, &mut wrap).unwrap();
        assert_eq!(output, "[Alpha, Beta,\n    Gamma]");

        let mut wrap = LineWrapping::new(8);
        let mut output = String::new();
        chunk.format(&mut output, &mut wrap).unwrap();
        assert_eq!(output, "[Alpha,\n    Beta,\n    Gamma]");
    }
}
