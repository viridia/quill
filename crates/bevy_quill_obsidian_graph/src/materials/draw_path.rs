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
    width: f32,
    commands: Vec<DrawablePathSegment>,
}

impl DrawablePath {
    pub fn new(width: f32) -> Self {
        Self {
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
    /// Direction of color gradient, normalized
    #[uniform(0)]
    pub(crate) gradient_normal: Vec2,

    /// Color at start of gradient
    #[uniform(1)]
    pub(crate) from_color: Vec4,

    /// Offset of first color stop along gradient normal
    #[uniform(2)]
    pub(crate) from_offset: f32,

    /// Color at end of gradient
    #[uniform(3)]
    pub(crate) to_color: Vec4,

    /// Offset of second color stop along gradient normal
    #[uniform(4)]
    pub(crate) to_offset: f32,

    /// Stroke width
    #[uniform(5)]
    pub(crate) width: f32,

    // #[uniform(2)]
    #[storage(6, read_only)]
    pub(crate) commands: Vec<PathCommand>,
}

impl DrawPathMaterial {
    pub fn update_color(
        &mut self,
        from_color: Srgba,
        from_pos: Vec2,
        to_color: Srgba,
        to_pos: Vec2,
    ) {
        let norm = (to_pos - from_pos).normalize();
        self.gradient_normal = norm;
        self.from_color = from_color.to_vec4();
        self.from_offset = from_pos.dot(norm);
        self.to_color = to_color.to_vec4();
        self.to_offset = to_pos.dot(norm);
    }

    pub fn update_path(&mut self, path: &DrawablePath) {
        let bounds = path.bounds();
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
