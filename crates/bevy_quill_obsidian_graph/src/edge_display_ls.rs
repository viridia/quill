use std::ops::Mul;

use bevy::{
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_quill::prelude::*;
use bevy_quill_obsidian::colors;

use crate::materials::LineMaterial;

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
        let line_material = cx.create_memo(
            |world, _| {
                let mut line_materials = world.get_resource_mut::<Assets<LineMaterial>>().unwrap();
                line_materials.add(LineMaterial {
                    color: colors::U4.into(),
                })
            },
            (),
        );
        // let line_material_id = line_material.id();

        // let mut ui_materials = cx
        //     .world_mut()
        //     .get_resource_mut::<Assets<DrawPathMaterial>>()
        //     .unwrap();
        // let material = ui_materials.add(DrawPathMaterial::default());
        // let material_id = material.id();
        let src_pos = self.src_pos.as_vec2();
        let dst_pos = self.dst_pos.as_vec2();

        Element::<MaterialMesh2dBundle<LineMaterial>>::new()
            .named("NodeGraph::Edge")
            .insert(Style::default())
            .insert(line_material)
            .effect(
                move |cx, ent, (src, dst)| {
                    let mut position = Vec::with_capacity(20);
                    // let mut path = DrawablePath::new(colors::U4, 1.5);
                    let dx = (dst.x - src.x).abs().mul(0.3).min(20.);
                    let src1 = src + Vec2::new(dx, 0.);
                    let dst1 = dst - Vec2::new(dx, 0.);
                    // path.move_to(src);
                    position.push([src.x, src.y, 0.]);
                    let mlen = src1.distance(dst1);
                    if mlen > 40. {
                        let src2 = src1.lerp(dst1, 20. / mlen);
                        let dst2 = src1.lerp(dst1, (mlen - 20.) / mlen);
                        position.push([src1.x, src1.y, 0.]);
                        position.push([src2.x, src2.y, 0.]);
                        // path.quadratic_to(src1, src2);
                        // path.line_to(dst2);
                        // path.quadratic_to(dst1, dst);
                        position.push([dst1.x, dst1.y, 0.]);
                        position.push([dst2.x, dst2.y, 0.]);
                        position.push([dst.x, dst.y, 0.]);
                    } else if mlen > 10. {
                        let mid = src1.lerp(dst1, 0.5);
                        // path.quadratic_to(src1, mid);
                        position.push([src1.x, src1.y, 0.]);
                        position.push([mid.x, mid.y, 0.]);
                        position.push([dst1.x, dst1.y, 0.]);
                        // path.quadratic_to(dst1, dst);
                        position.push([dst.x, dst.y, 0.]);
                    } else {
                        // path.line_to(dst);
                        position.push([dst.x, dst.y, 0.]);
                    }
                    // let bounds = path.bounds();
                    // println!("bounds: {:?}", bounds);

                    // println!("path: {:?}", path);

                    // let mut style = cx.world_mut().get_mut::<Style>(ent).unwrap();
                    // style.left = ui::Val::Px(bounds.min.x);
                    // style.top = ui::Val::Px(bounds.min.y);
                    // style.width = ui::Val::Px(bounds.width());
                    // style.height = ui::Val::Px(bounds.height());
                    // style.position_type = ui::PositionType::Absolute;

                    let mut meshes = cx.world_mut().get_resource_mut::<Assets<Mesh>>().unwrap();
                    let mesh = meshes.add(
                        Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::default())
                            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, position),
                    );
                    cx.world_mut().entity_mut(ent).insert(Mesh2dHandle(mesh));

                    // let mut materials = cx
                    //     .world_mut()
                    //     .get_resource_mut::<Assets<DrawPathMaterial>>()
                    //     .unwrap();
                    // let material = materials.get_mut(material_id).unwrap();
                    // material.update(&path);

                    // Insert the path as a component
                    // cx.world_mut().entity_mut(ent).insert(LineStrip(vertices));

                    // let mesh = Mesh::from(value)
                },
                (src_pos, dst_pos),
            )
    }
}
