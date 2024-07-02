use crate::graph::GraphResource;
use bevy::color::Color;
use bevy_mod_stylebuilder::*;
use bevy_quill::prelude::*;
use bevy_quill_obsidian_graph::GraphDisplay;

fn style_node_graph(ss: &mut StyleBuilder) {
    ss.flex_grow(1.).border_left(1).border_color(Color::BLACK);
}

#[derive(Clone, PartialEq)]
pub struct GraphView;

impl ViewTemplate for GraphView {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let graph = cx.use_resource::<GraphResource>();
        let node_ids: Vec<_> = graph.0.iter_nodes().map(|(key, _)| *key).collect();
        println!("GraphView render");
        GraphDisplay::new()
            .style(style_node_graph)
            .children((For::each(node_ids, |node| "A"),))
    }
}
