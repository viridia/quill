use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;

/// An element within a stroked path.
#[derive(Debug, Copy, Clone)]
pub enum DrawablePathSegment {
    /// Move to a new position.
    Move(Vec2),
    /// Draw a straight line to a new position.
    Line(Vec2),
    /// Draw a quadratic curve to a new position.
    Quadratic((Vec2, Vec2)),
}

/// Defines a stroked path
#[derive(Debug, Clone)]
pub struct DrawablePath {
    color: Srgba,
    width: f32,
    commands: Vec<DrawablePathSegment>,
}

impl DrawablePath {
    pub fn new(color: Srgba, width: f32) -> Self {
        Self {
            color,
            width,
            commands: Vec::new(),
        }
    }

    pub fn move_to(&mut self, point: Vec2) {
        self.commands.push(DrawablePathSegment::Move(point));
    }

    pub fn line_to(&mut self, point: Vec2) {
        self.commands.push(DrawablePathSegment::Line(point));
    }

    pub fn quadratic_to(&mut self, control: Vec2, point: Vec2) {
        self.commands
            .push(DrawablePathSegment::Quadratic((control, point)));
    }

    pub fn bounds(&self) -> Rect {
        if self.commands.is_empty() {
            return Rect::default();
        }
        let mut bounds = Rect {
            min: Vec2::splat(f32::INFINITY),
            max: Vec2::splat(f32::NEG_INFINITY),
        };
        for segment in &self.commands {
            match segment {
                DrawablePathSegment::Move(point) | DrawablePathSegment::Line(point) => {
                    bounds = bounds.union_point(*point);
                }
                DrawablePathSegment::Quadratic((control, point)) => {
                    bounds = bounds.union_point(*control);
                    bounds = bounds.union_point(*point);
                }
            }
        }
        bounds.inflate(self.width * 0.5)
    }
}

/// Type of drawing operation for each path segment.
enum PathCommandType {
    Move = 0,
    Line = 1,
    Quad1 = 2,
    Quad2 = 3,
}

#[derive(ShaderType, Debug, Clone)]
pub struct PathCommand {
    op: u32,
    point: Vec2,
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone, Default)]
pub struct DrawPathMaterial {
    /// Stroke color
    #[uniform(0)]
    pub(crate) color: Vec4,

    /// Stroke width
    #[uniform(1)]
    pub(crate) width: f32,

    // #[uniform(2)]
    #[storage(2, read_only)]
    pub(crate) commands: Vec<PathCommand>,
}

impl DrawPathMaterial {
    pub fn update(&mut self, path: &DrawablePath) {
        let bounds = path.bounds();
        self.color = path.color.to_vec4();
        self.width = path.width;
        self.commands.clear();
        // println!("Updating material: {}", path.commands.len());
        for segment in &path.commands {
            match segment {
                DrawablePathSegment::Move(point) => {
                    self.commands.push(PathCommand {
                        op: PathCommandType::Move as u32,
                        point: *point - bounds.min,
                    });
                }
                DrawablePathSegment::Line(point) => {
                    self.commands.push(PathCommand {
                        op: PathCommandType::Line as u32,
                        point: *point - bounds.min,
                    });
                }
                DrawablePathSegment::Quadratic((control, point)) => {
                    self.commands.push(PathCommand {
                        op: PathCommandType::Quad1 as u32,
                        point: *control - bounds.min,
                    });
                    self.commands.push(PathCommand {
                        op: PathCommandType::Quad2 as u32,
                        point: *point - bounds.min,
                    });
                }
            }
        }
    }
}

impl UiMaterial for DrawPathMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_quill_obsidian_graph/assets/draw_path.wgsl".into()
    }
}
