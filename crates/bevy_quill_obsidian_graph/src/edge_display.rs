use std::ops::Mul;

use bevy::{prelude::*, ui};
use bevy_quill::prelude::*;
use bevy_quill_obsidian::colors;

use crate::materials::{DrawPathMaterial, DrawablePath};

/// Displays a stroked path between two nodes.
#[derive(Clone, PartialEq)]
pub struct EdgeDisplay {
    /// Pixel position of the source terminal.
    pub src_pos: IVec2,

    /// Pixel position of the destination terminal.
    pub dst_pos: IVec2,
}

impl ViewTemplate for EdgeDisplay {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        let mut ui_materials = cx
            .world_mut()
            .get_resource_mut::<Assets<DrawPathMaterial>>()
            .unwrap();
        let material = ui_materials.add(DrawPathMaterial::default());
        let material_id = material.id();
        let src_pos = self.src_pos.as_vec2();
        let dst_pos = self.dst_pos.as_vec2();

        Element::<MaterialNodeBundle<DrawPathMaterial>>::new()
            .named("NodeGraph::Edge")
            .insert(material)
            .effect(
                move |cx, ent, (src, dst)| {
                    let mut path = DrawablePath::new(colors::U4, 1.5);
                    let dx = (dst.x - src.x).abs().mul(0.3).min(20.);
                    let src1 = src + Vec2::new(dx, 0.);
                    let dst1 = dst - Vec2::new(dx, 0.);
                    path.move_to(src);
                    let mlen = src1.distance(dst1);
                    if mlen > 40. {
                        let src2 = src1.lerp(dst1, 20. / mlen);
                        let dst2 = src1.lerp(dst1, (mlen - 20.) / mlen);
                        path.quadratic_to(src1, src2);
                        path.line_to(dst2);
                        path.quadratic_to(dst1, dst);
                    } else {
                        let mid = src1.lerp(dst1, 0.5);
                        path.quadratic_to(src1, mid);
                        path.quadratic_to(dst1, dst);
                    }
                    let bounds = path.bounds();

                    let mut style = cx.world_mut().get_mut::<Style>(ent).unwrap();
                    style.left = ui::Val::Px(bounds.min.x);
                    style.top = ui::Val::Px(bounds.min.y);
                    style.width = ui::Val::Px(bounds.width());
                    style.height = ui::Val::Px(bounds.height());
                    style.position_type = ui::PositionType::Absolute;

                    let mut materials = cx
                        .world_mut()
                        .get_resource_mut::<Assets<DrawPathMaterial>>()
                        .unwrap();
                    let material = materials.get_mut(material_id).unwrap();
                    material.update(&path);
                },
                (src_pos, dst_pos),
            )
    }
}
