use bevy::{
    math::{Rect, Vec2, Vec3},
    render::mesh::{Indices, Mesh, PrimitiveTopology},
};

use super::mesh_builder::MeshBuilder;

/// Indicates which way the flat primitives are facing in 3D space.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ShapeOrientation {
    XPositive,
    XNegative,
    YPositive,
    YNegative,
    #[default]
    ZPositive,
    ZNegative,
}

impl ShapeOrientation {
    /// Returns the normal vector for the orientation.
    pub fn normal(&self) -> Vec3 {
        match self {
            ShapeOrientation::XPositive => Vec3::new(1.0, 0.0, 0.0),
            ShapeOrientation::XNegative => Vec3::new(-1.0, 0.0, 0.0),
            ShapeOrientation::YPositive => Vec3::new(0.0, 1.0, 0.0),
            ShapeOrientation::YNegative => Vec3::new(0.0, -1.0, 0.0),
            ShapeOrientation::ZPositive => Vec3::new(0.0, 0.0, 1.0),
            ShapeOrientation::ZNegative => Vec3::new(0.0, 0.0, -1.0),
        }
    }

    /// Convert a 3D point to a 2D point in the orientation.
    pub fn vec2(&self, pt: Vec3) -> Vec2 {
        match self {
            ShapeOrientation::XPositive => Vec2::new(pt.y, pt.z),
            ShapeOrientation::XNegative => Vec2::new(-pt.y, pt.z),
            ShapeOrientation::YPositive => Vec2::new(pt.x, pt.z),
            ShapeOrientation::YNegative => Vec2::new(-pt.x, pt.z),
            ShapeOrientation::ZPositive => Vec2::new(pt.x, pt.y),
            ShapeOrientation::ZNegative => Vec2::new(-pt.x, pt.y),
        }
    }

    /// Convert a 2D point to a 3D point in the orientation.
    pub fn vec3(&self, pt: Vec2) -> Vec3 {
        match self {
            ShapeOrientation::XPositive => Vec3::new(0.0, pt.x, pt.y),
            ShapeOrientation::XNegative => Vec3::new(0.0, -pt.x, pt.y),
            ShapeOrientation::YPositive => Vec3::new(pt.x, 0.0, pt.y),
            ShapeOrientation::YNegative => Vec3::new(-pt.x, 0.0, pt.y),
            ShapeOrientation::ZPositive => Vec3::new(pt.x, pt.y, 0.0),
            ShapeOrientation::ZNegative => Vec3::new(-pt.x, pt.y, 0.0),
        }
    }
}

/// A marker for the start or end of a shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum StrokeMarker {
    /// No marker.
    #[default]
    None,

    /// Arrowhead marker.
    Arrowhead,
    // Future: Diamond, Circle, Square, etc.
}

/// A builder for creating two-dimensional shapes.
#[derive(Clone, Debug, Default)]
pub struct ShapeBuilder {
    vertices: Vec<Vec3>,
    indices: Vec<u32>,
    stroke_width: f32,
    orientation: ShapeOrientation,
}

/// Options for drawing a polygon or polyline stroke.
#[derive(Clone, Debug)]
pub struct PolygonOptions {
    /// Whether the polygon should be closed.
    pub closed: bool,

    /// Line style: length of the dash.
    pub dash_length: f32,

    /// Line style: length of the gap between dashes.
    pub gap_length: f32,

    /// Marker at the start of the polyline.
    pub start_marker: StrokeMarker,

    /// Marker at the end of the polyline.
    pub end_marker: StrokeMarker,
}

impl Default for PolygonOptions {
    fn default() -> Self {
        Self {
            closed: false,
            dash_length: f32::MAX,
            gap_length: 0.,
            start_marker: StrokeMarker::None,
            end_marker: StrokeMarker::None,
        }
    }
}

impl ShapeBuilder {
    /// Create a new `ShapeBuilder`.
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            stroke_width: 1.0,
            orientation: ShapeOrientation::ZPositive,
        }
    }

    /// Set the orientation of the shape.
    #[inline]
    pub fn with_orientation(&mut self, orientation: ShapeOrientation) -> &mut Self {
        self.orientation = orientation;
        self
    }

    /// Set the stroke width for the shape.
    #[inline]
    pub fn with_stroke_width(&mut self, stroke_width: f32) -> &mut Self {
        self.stroke_width = stroke_width;
        self
    }

    /// Reserve space for vertices and indices.
    pub fn reserve(&mut self, vertices: usize, indices: usize) -> &mut Self {
        self.vertices.reserve(vertices);
        self.indices.reserve(indices);
        self
    }

    /// Add a vertex to the shape.
    #[inline]
    pub fn push_vertex(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.vertices.push(Vec3::new(x, y, z));
        self
    }

    /// Add a vertex to the shape.
    #[inline]
    pub fn push_xy(&mut self, x: f32, y: f32) -> &mut Self {
        self.vertices.push(self.orientation.vec3(Vec2::new(x, y)));
        self
    }

    /// Add an index to the shape.
    #[inline]
    pub fn push_index(&mut self, index: u32) -> &mut Self {
        self.indices.push(index);
        self
    }

    /// Add an index to the shape.
    #[inline]
    pub fn push_indices(&mut self, indices: &[u32]) -> &mut Self {
        self.indices.extend(indices);
        self
    }

    /// Draw a stroke in the shape of a rectangle.
    ///
    /// Arguments:
    /// `rect` - The outer bounds of the rectangle.
    pub fn stroke_rect(&mut self, rect: Rect) -> &mut Self {
        self.reserve(8, 24);

        let start = self.vertices.len() as u32;
        let lw = self.stroke_width;

        self.push_xy(rect.min.x + lw, rect.min.y + lw);
        self.push_xy(rect.min.x, rect.min.y);

        self.push_xy(rect.max.x - lw, rect.min.y + lw);
        self.push_xy(rect.max.x, rect.min.y);

        self.push_xy(rect.max.x - lw, rect.max.y - lw);
        self.push_xy(rect.max.x, rect.max.y);

        self.push_xy(rect.min.x + lw, rect.max.y - lw);
        self.push_xy(rect.min.x, rect.max.y);

        self.push_index(start);
        self.push_index(start + 2);
        self.push_index(start + 1);

        self.push_index(start + 1);
        self.push_index(start + 2);
        self.push_index(start + 3);

        self.push_index(start + 2);
        self.push_index(start + 4);
        self.push_index(start + 3);

        self.push_index(start + 4);
        self.push_index(start + 5);
        self.push_index(start + 3);

        self.push_index(start + 4);
        self.push_index(start + 6);
        self.push_index(start + 5);

        self.push_index(start + 5);
        self.push_index(start + 6);
        self.push_index(start + 7);

        self.push_index(start + 6);
        self.push_index(start);
        self.push_index(start + 1);

        self.push_index(start + 6);
        self.push_index(start + 1);
        self.push_index(start + 7);

        self
    }

    /// Draw a filled rectangle.
    pub fn fill_rect(&mut self, rect: Rect) -> &mut Self {
        self.reserve(4, 6);
        let start = self.vertices.len() as u32;
        self.push_xy(rect.min.x, rect.min.y);
        self.push_xy(rect.max.x, rect.min.y);
        self.push_xy(rect.max.x, rect.max.y);
        self.push_xy(rect.min.x, rect.max.y);
        self.push_index(start);
        self.push_index(start + 2);
        self.push_index(start + 1);
        self.push_index(start);
        self.push_index(start + 3);
        self.push_index(start + 2);
        self
    }

    /// Draw a circular stroke.
    pub fn stroke_circle(&mut self, center: Vec2, radius: f32, segments: u32) -> &mut Self {
        self.reserve((segments * 2) as usize, (segments * 6) as usize);
        let start = self.vertices.len() as u32;
        let step = 2.0 * std::f32::consts::PI / segments as f32;
        let radius_inner = (radius - self.stroke_width).max(0.0);
        let radius_outer = radius_inner + self.stroke_width;
        for i in 0..segments {
            let angle = i as f32 * step;
            let c = angle.cos();
            let s = angle.sin();
            let x_inner = center.x + radius_inner * c;
            let y_inner = center.y + radius_inner * s;
            let x_outer = center.x + radius_outer * c;
            let y_outer = center.y + radius_outer * s;
            let next_index = (i + 1).rem_euclid(segments);
            self.push_xy(x_inner, y_inner);
            self.push_xy(x_outer, y_outer);

            self.push_index(start + i * 2);
            self.push_index(start + next_index * 2);
            self.push_index(start + i * 2 + 1);

            self.push_index(start + i * 2 + 1);
            self.push_index(start + next_index * 2);
            self.push_index(start + next_index * 2 + 1);
        }
        self
    }

    /// Draw a filled circle.
    pub fn fill_circle(&mut self, center: Vec2, radius: f32, segments: u32) -> &mut Self {
        self.reserve((segments + 1) as usize, (segments * 3) as usize);
        let start = self.vertices.len() as u32;
        let step = 2.0 * std::f32::consts::PI / (segments as f32);
        self.push_xy(center.x, center.y);
        for i in 0..segments {
            let angle = i as f32 * step;
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();
            self.push_xy(x, y);
            self.push_index(start);
            self.push_index(start + (i + 1).rem_euclid(segments) + 1);
            self.push_index(start + i + 1);
        }
        self
    }

    /// Draw a filled triangle.
    pub fn fill_triangle(&mut self, a: Vec2, b: Vec2, c: Vec2) -> &mut Self {
        self.reserve(3, 3);
        let start = self.vertices.len() as u32;
        self.push_xy(a.x, a.y);
        self.push_xy(b.x, b.y);
        self.push_xy(c.x, c.y);
        self.push_index(start);
        self.push_index(start + 2);
        self.push_index(start + 1);
        self
    }

    /// Draw a filled triangle.
    pub fn fill_triangle_3d(&mut self, a: Vec3, b: Vec3, c: Vec3) -> &mut Self {
        self.reserve(3, 3);
        let start = self.vertices.len() as u32;
        self.vertices.push(a);
        self.vertices.push(b);
        self.vertices.push(c);
        self.push_index(start);
        self.push_index(start + 2);
        self.push_index(start + 1);
        self
    }

    /// Draw a filled quad.
    pub fn fill_quad(&mut self, a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> &mut Self {
        self.reserve(4, 6);
        let start = self.vertices.len() as u32;
        self.push_xy(a.x, a.y);
        self.push_xy(b.x, b.y);
        self.push_xy(c.x, c.y);
        self.push_xy(d.x, d.y);
        self.push_index(start);
        self.push_index(start + 2);
        self.push_index(start + 1);
        self.push_index(start);
        self.push_index(start + 3);
        self.push_index(start + 2);
        self
    }

    /// Draw a line segment.
    pub fn stroke_line_segment(&mut self, p0: Vec2, p1: Vec2) -> &mut Self {
        let lw = self.stroke_width * 0.5;
        let v_dir = p1 - p0;
        let v_perp = Vec2::new(v_dir.y, -v_dir.x).normalize() * lw;

        let v0_index = self.push_vec2_index(p0 + v_perp);
        let v1_index = self.push_vec2_index(p0 - v_perp);

        let v2_index = self.push_vec2_index(p1 + v_perp);
        let v3_index = self.push_vec2_index(p1 - v_perp);

        self.push_indices(&[v0_index, v1_index, v2_index, v1_index, v3_index, v2_index]);
        self
    }

    /// Draw a line segment in 3d space, with the face of the stroke oriented along the orientation
    /// axis. This generally works best if the line is mostly parallel to the orientation plane.
    pub fn stroke_line_segment_3d(&mut self, p0: Vec3, p1: Vec3) -> &mut Self {
        let norm = self.orientation.normal();
        let lw = self.stroke_width * 0.5;
        let v_dir = (p1 - p0).normalize();
        let v_perp = norm.cross(v_dir) * lw;

        let v0_index = self.push_vec3_index(p0 + v_perp);
        let v1_index = self.push_vec3_index(p0 - v_perp);

        let v2_index = self.push_vec3_index(p1 + v_perp);
        let v3_index = self.push_vec3_index(p1 - v_perp);

        self.push_indices(&[v0_index, v1_index, v2_index, v1_index, v3_index, v2_index]);
        self
    }

    /// Draw a polygon from a list of 2d points.
    pub fn stroke_polygon(&mut self, vertices: &[Vec2], options: PolygonOptions) -> &mut Self {
        if vertices.len() < 2 {
            return self;
        }
        let closed = options.closed && vertices.len() > 2;
        let lw = self.stroke_width * 0.5;
        let count = vertices.len();

        let mut dash_end = options.dash_length;

        // Indices of the vertices at the start of the current dash.
        let mut v0_index: u32 = 0;
        let mut v1_index: u32 = 0;

        for i in 0..count {
            let vtx = vertices[i];
            let vtx_next = vertices[(i + 1).rem_euclid(count)];

            // Length and direction of line segment
            let mut length = vtx.distance(vtx_next);
            let v_dir = (vtx_next - vtx) / length;
            let v_perp = Vec2::new(v_dir.y, -v_dir.x).normalize() * lw;

            if i == 0 {
                // Generate vertices for the start of first segment.
                if closed {
                    // Mitered starting point.
                    let vtx_prev = *vertices.last().unwrap();
                    let v_dir_prev = (vtx - vtx_prev).normalize();
                    let dot = (v_dir + v_dir_prev).normalize().dot(v_dir_prev);
                    let v_miter =
                        Vec2::new(v_dir_prev.y + v_dir.y, -v_dir_prev.x - v_dir.x).normalize() * lw
                            / dot;
                    let v2_index = self.push_vec2_index(vtx + v_miter);
                    let v3_index = self.push_vec2_index(vtx - v_miter);
                    self.push_indices(&[
                        v0_index, v1_index, v2_index, v1_index, v3_index, v2_index,
                    ]);
                    v0_index = v2_index;
                    v1_index = v3_index;
                    // todo!();
                } else {
                    // Draw start marker and update position.
                    let marker_length = self.marker_length(options.start_marker).min(length * 0.4);
                    self.fill_marker(
                        options.start_marker,
                        vtx + v_dir * marker_length,
                        -v_dir,
                        marker_length,
                    );
                    v0_index = self.push_vec2_index(vtx + v_perp + v_dir * marker_length);
                    v1_index = self.push_vec2_index(vtx - v_perp + v_dir * marker_length);
                    dash_end += marker_length;
                }
            }

            // If the segment ends in a marker, reduce the line segment length.
            let marker_length = self.marker_length(options.end_marker).min(length * 0.4);
            if i == count - 2 && !closed {
                length -= marker_length;
            }

            while dash_end < length {
                // Finish the previous dash.
                let v_dash_end = vtx + v_dir * dash_end.min(length);
                let v2_index = self.push_vec2_index(v_dash_end + v_perp);
                let v3_index = self.push_vec2_index(v_dash_end - v_perp);
                self.push_indices(&[v0_index, v1_index, v2_index, v1_index, v3_index, v2_index]);

                // Start a new dash if there's room
                if dash_end + options.gap_length < length {
                    let v_dash_start = vtx + v_dir * (dash_end + options.gap_length);
                    v0_index = self.push_vec2_index(v_dash_start + v_perp);
                    v1_index = self.push_vec2_index(v_dash_start - v_perp);
                }

                // Prep for next dash
                dash_end += options.dash_length + options.gap_length;
            }

            // Miter at end, if it's in the middle of a dash.
            if dash_end - options.dash_length < length {
                if i < count - 2 || options.closed {
                    // Mitered angle.
                    let vtx_next2 = vertices[(i + 2).rem_euclid(count)];
                    let v_dir_next = (vtx_next2 - vtx_next).normalize();
                    let dot = (v_dir_next + v_dir).normalize().dot(v_dir);
                    let v_miter =
                        Vec2::new(v_dir.y + v_dir_next.y, -v_dir.x - v_dir_next.x).normalize() * lw
                            / dot;
                    let v2_index = self.push_vec2_index(vtx_next + v_miter);
                    let v3_index = self.push_vec2_index(vtx_next - v_miter);
                    self.push_indices(&[
                        v0_index, v1_index, v2_index, v1_index, v3_index, v2_index,
                    ]);
                    v0_index = v2_index;
                    v1_index = v3_index;
                } else {
                    // Butt end
                    let v_seg_end = vtx + v_dir * length;
                    let v2 = v_seg_end + v_perp;
                    let v3 = v_seg_end - v_perp;
                    let v2_index = self.push_vec2_index(v2);
                    let v3_index = self.push_vec2_index(v3);
                    self.push_indices(&[
                        v0_index, v1_index, v2_index, v1_index, v3_index, v2_index,
                    ]);
                    self.fill_marker(options.end_marker, v_seg_end, v_dir, marker_length);
                    break;
                }
            }

            dash_end -= length;
        }
        self
    }

    /// Draw a polygon from a list of 3d points.
    pub fn stroke_polygon_3d(&mut self, vertices: &[Vec3], options: PolygonOptions) -> &mut Self {
        if vertices.len() < 2 {
            return self;
        }
        let norm = self.orientation.normal();
        let closed = options.closed && vertices.len() > 2;
        let lw = self.stroke_width * 0.5;
        let count = vertices.len();

        let mut dash_end = options.dash_length;

        // Indices of the vertices at the start of the current dash.
        let mut v0_index: u32 = 0;
        let mut v1_index: u32 = 0;

        for i in 0..count {
            let vtx = vertices[i];
            let vtx_next = vertices[(i + 1).rem_euclid(count)];

            // Length and direction of line segment
            let mut length = vtx.distance(vtx_next);
            let v_dir = (vtx_next - vtx) / length;
            let v_perp = norm.cross(v_dir) * lw;

            if i == 0 {
                // Generate vertices for the start of first segment.
                if closed {
                    // Mitered starting point.
                    let vtx_prev = *vertices.last().unwrap();
                    let v_dir_prev = (vtx - vtx_prev).normalize();
                    let dot = (v_dir + v_dir_prev).normalize().dot(v_dir_prev);
                    let v_miter = norm.cross(v_dir_prev + v_dir).normalize() * lw / dot;
                    // Vec3::new(v_dir_prev.y + v_dir.y, -v_dir_prev.x - v_dir.x).normalize() * lw
                    //     / dot;
                    let v2_index = self.push_vec3_index(vtx + v_miter);
                    let v3_index = self.push_vec3_index(vtx - v_miter);
                    self.push_indices(&[
                        v0_index, v1_index, v2_index, v1_index, v3_index, v2_index,
                    ]);
                    v0_index = v2_index;
                    v1_index = v3_index;
                    // todo!();
                } else {
                    // Draw start marker and update position.
                    let marker_length = self.marker_length(options.start_marker).min(length * 0.4);
                    self.fill_marker_3d(
                        options.start_marker,
                        vtx + v_dir * marker_length,
                        -v_dir,
                        marker_length,
                    );
                    v0_index = self.push_vec3_index(vtx + v_perp + v_dir * marker_length);
                    v1_index = self.push_vec3_index(vtx - v_perp + v_dir * marker_length);
                    dash_end += marker_length;
                }
            }

            // If the segment ends in a marker, reduce the line segment length.
            let marker_length = self.marker_length(options.end_marker).min(length * 0.4);
            if i == count - 2 && !closed {
                length -= marker_length;
            }

            while dash_end < length {
                // Finish the previous dash.
                let v_dash_end = vtx + v_dir * dash_end.min(length);
                let v2_index = self.push_vec3_index(v_dash_end + v_perp);
                let v3_index = self.push_vec3_index(v_dash_end - v_perp);
                self.push_indices(&[v0_index, v1_index, v2_index, v1_index, v3_index, v2_index]);

                // Start a new dash if there's room
                if dash_end + options.gap_length < length {
                    let v_dash_start = vtx + v_dir * (dash_end + options.gap_length);
                    v0_index = self.push_vec3_index(v_dash_start + v_perp);
                    v1_index = self.push_vec3_index(v_dash_start - v_perp);
                }

                // Prep for next dash
                dash_end += options.dash_length + options.gap_length;
            }

            // Miter at end, if it's in the middle of a dash.
            if dash_end - options.dash_length < length {
                if i < count - 2 || options.closed {
                    // Mitered angle.
                    let vtx_next2 = vertices[(i + 2).rem_euclid(count)];
                    let v_dir_next = (vtx_next2 - vtx_next).normalize();
                    let dot = (v_dir_next + v_dir).normalize().dot(v_dir);
                    let v_miter = norm.cross(v_dir + v_dir_next).normalize() * lw / dot;
                    // Vec3::new(v_dir.y + v_dir_next.y, -v_dir.x - v_dir_next.x).normalize() * lw
                    //     / dot;
                    let v2_index = self.push_vec3_index(vtx_next + v_miter);
                    let v3_index = self.push_vec3_index(vtx_next - v_miter);
                    self.push_indices(&[
                        v0_index, v1_index, v2_index, v1_index, v3_index, v2_index,
                    ]);
                    v0_index = v2_index;
                    v1_index = v3_index;
                } else {
                    // Butt end
                    let v_seg_end = vtx + v_dir * length;
                    let v2 = v_seg_end + v_perp;
                    let v3 = v_seg_end - v_perp;
                    let v2_index = self.push_vec3_index(v2);
                    let v3_index = self.push_vec3_index(v3);
                    self.push_indices(&[
                        v0_index, v1_index, v2_index, v1_index, v3_index, v2_index,
                    ]);
                    self.fill_marker_3d(options.end_marker, v_seg_end, v_dir, marker_length);
                    break;
                }
            }

            dash_end -= length;
        }
        self
    }

    /// Add a vertex to the shape, and return the index of that vertex.
    #[inline]
    fn push_vec2_index(&mut self, v: Vec2) -> u32 {
        let index = self.vertices.len() as u32;
        self.push_xy(v.x, v.y);
        index
    }

    /// Add a vertex to the shape, and return the index of that vertex.
    #[inline]
    fn push_vec3_index(&mut self, v: Vec3) -> u32 {
        let index = self.vertices.len() as u32;
        self.vertices.push(v);
        index
    }

    fn fill_marker(&mut self, marker: StrokeMarker, position: Vec2, direction: Vec2, length: f32) {
        #[allow(clippy::single_match)]
        match marker {
            StrokeMarker::Arrowhead => {
                let v_perp = Vec2::new(direction.y, -direction.x).normalize() * length * 0.75;
                let v0 = position + direction * length;
                let v1 = position + v_perp;
                let v2 = position - v_perp;
                self.fill_triangle(v0, v2, v1);
            }
            _ => {}
        }
    }

    fn fill_marker_3d(
        &mut self,
        marker: StrokeMarker,
        position: Vec3,
        direction: Vec3,
        length: f32,
    ) {
        #[allow(clippy::single_match)]
        match marker {
            StrokeMarker::Arrowhead => {
                let norm = self.orientation.normal();
                let v_perp = norm.cross(direction.normalize()) * length * 0.75;
                let v0 = position + direction * length;
                let v1 = position + v_perp;
                let v2 = position - v_perp;
                self.fill_triangle_3d(v0, v2, v1);
            }
            _ => {}
        }
    }

    /// Compute the length of the stroke marker, relative to the stroke width.
    fn marker_length(&self, marker: StrokeMarker) -> f32 {
        match marker {
            StrokeMarker::Arrowhead => self.stroke_width * 2.0,
            _ => 0.0,
        }
    }
}

impl MeshBuilder for ShapeBuilder {
    fn topology() -> PrimitiveTopology {
        PrimitiveTopology::TriangleList
    }

    /// Copy the shape into a [`Mesh`]. This will consume the builder and return a mesh.
    fn build(self, mesh: &mut Mesh) {
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
        mesh.insert_indices(Indices::U32(self.indices));
    }
}
