use crate::graph::{GraphNode, GraphResource};
use bevy::{color::Color, prelude::*};
use bevy_mod_stylebuilder::*;
use bevy_quill::prelude::*;
use bevy_quill_obsidian_graph::{GraphDisplay, NodeDisplay};

fn style_node_graph(ss: &mut StyleBuilder) {
    ss.flex_grow(1.).border_left(1).border_color(Color::BLACK);
}

#[derive(Clone, PartialEq)]
pub struct GraphView;

impl ViewTemplate for GraphView {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let graph = cx.use_resource::<GraphResource>();
        let node_ids: Vec<_> = graph.0.iter_nodes().map(|(_, v)| *v).collect();
        GraphDisplay::new()
            .style(style_node_graph)
            .children((For::each(node_ids, |node| GraphNodeView(*node)),))
    }
}

#[derive(Clone, PartialEq)]
pub struct GraphNodeView(Entity);

impl ViewTemplate for GraphNodeView {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let entity = self.0;
        let node = cx.use_component::<GraphNode>(entity).unwrap();
        NodeDisplay::new()
            .position(node.position)
            .title(node.title())
            .on_drag(
                cx.create_callback(move |new_pos: In<Vec2>, world: &mut World| {
                    let mut entt = world.entity_mut(entity);
                    let mut pos = entt.get_mut::<GraphNode>().unwrap();
                    pos.position = new_pos.as_ivec2();
                }),
            )
    }
}
