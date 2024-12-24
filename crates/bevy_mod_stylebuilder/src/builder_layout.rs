use super::builder::{LengthParam, OptFloatParam, StyleBuilder, UiRectParam};
use bevy::ui;

#[allow(missing_docs)]
pub trait StyleBuilderLayout {
    fn display(&mut self, disp: ui::Display) -> &mut Self;

    /// Set the display to flex and the flex direction to row.
    fn flex_row(&mut self) -> &mut Self;

    /// Set the display to flex and the flex direction to column.
    fn flex_column(&mut self) -> &mut Self;

    fn position(&mut self, pos: ui::PositionType) -> &mut Self;
    fn overflow(&mut self, ov: ui::OverflowAxis) -> &mut Self;
    fn overflow_x(&mut self, ov: ui::OverflowAxis) -> &mut Self;
    fn overflow_y(&mut self, ov: ui::OverflowAxis) -> &mut Self;
    fn left(&mut self, length: impl LengthParam) -> &mut Self;
    fn right(&mut self, length: impl LengthParam) -> &mut Self;
    fn top(&mut self, length: impl LengthParam) -> &mut Self;
    fn bottom(&mut self, length: impl LengthParam) -> &mut Self;
    fn width(&mut self, length: impl LengthParam) -> &mut Self;
    fn height(&mut self, length: impl LengthParam) -> &mut Self;
    fn min_width(&mut self, length: impl LengthParam) -> &mut Self;
    fn min_height(&mut self, length: impl LengthParam) -> &mut Self;
    fn max_width(&mut self, length: impl LengthParam) -> &mut Self;
    fn max_height(&mut self, length: impl LengthParam) -> &mut Self;
    fn aspect_ratio(&mut self, length: impl OptFloatParam) -> &mut Self;
    fn margin(&mut self, rect: impl UiRectParam) -> &mut Self;
    fn margin_left(&mut self, length: impl LengthParam) -> &mut Self;
    fn margin_right(&mut self, length: impl LengthParam) -> &mut Self;
    fn margin_top(&mut self, length: impl LengthParam) -> &mut Self;
    fn margin_bottom(&mut self, length: impl LengthParam) -> &mut Self;
    fn padding(&mut self, rect: impl UiRectParam) -> &mut Self;
    fn padding_left(&mut self, length: impl LengthParam) -> &mut Self;
    fn padding_right(&mut self, length: impl LengthParam) -> &mut Self;
    fn padding_top(&mut self, length: impl LengthParam) -> &mut Self;
    fn padding_bottom(&mut self, length: impl LengthParam) -> &mut Self;
    fn border(&mut self, rect: impl UiRectParam) -> &mut Self;
    fn border_left(&mut self, length: impl LengthParam) -> &mut Self;
    fn border_right(&mut self, length: impl LengthParam) -> &mut Self;
    fn border_top(&mut self, length: impl LengthParam) -> &mut Self;
    fn border_bottom(&mut self, length: impl LengthParam) -> &mut Self;
    fn flex_direction(&mut self, dir: ui::FlexDirection) -> &mut Self;
    fn flex_wrap(&mut self, w: ui::FlexWrap) -> &mut Self;
    fn flex(&mut self, grow: f32, shrink: f32, basis: impl LengthParam) -> &mut Self;
    fn flex_grow(&mut self, n: f32) -> &mut Self;
    fn flex_shrink(&mut self, n: f32) -> &mut Self;
    fn flex_basis(&mut self, length: impl LengthParam) -> &mut Self;
    fn row_gap(&mut self, length: impl LengthParam) -> &mut Self;
    fn column_gap(&mut self, length: impl LengthParam) -> &mut Self;
    fn gap(&mut self, length: impl LengthParam) -> &mut Self;
    fn align_items(&mut self, align: ui::AlignItems) -> &mut Self;
    fn align_self(&mut self, align: ui::AlignSelf) -> &mut Self;
    fn align_content(&mut self, align: ui::AlignContent) -> &mut Self;
    fn justify_items(&mut self, justify: ui::JustifyItems) -> &mut Self;
    fn justify_self(&mut self, justify: ui::JustifySelf) -> &mut Self;
    fn justify_content(&mut self, justify: ui::JustifyContent) -> &mut Self;
    fn grid_auto_flow(&mut self, flow: ui::GridAutoFlow) -> &mut Self;
    fn grid_template_rows(&mut self, rows: Vec<ui::RepeatedGridTrack>) -> &mut Self;
    fn grid_template_columns(&mut self, columns: Vec<ui::RepeatedGridTrack>) -> &mut Self;
    fn grid_auto_rows(&mut self, rows: Vec<ui::GridTrack>) -> &mut Self;
    fn grid_auto_columns(&mut self, columns: Vec<ui::GridTrack>) -> &mut Self;
    fn grid_row(&mut self, val: ui::GridPlacement) -> &mut Self;
    fn grid_row_start(&mut self, val: i16) -> &mut Self;
    fn grid_row_span(&mut self, val: u16) -> &mut Self;
    fn grid_row_end(&mut self, val: i16) -> &mut Self;
    fn grid_column(&mut self, val: ui::GridPlacement) -> &mut Self;
    fn grid_column_start(&mut self, val: i16) -> &mut Self;
    fn grid_column_span(&mut self, val: u16) -> &mut Self;
    fn grid_column_end(&mut self, val: i16) -> &mut Self;
}

impl<'a, 'w> StyleBuilderLayout for StyleBuilder<'a, 'w> {
    fn display(&mut self, disp: ui::Display) -> &mut Self {
        self.node.display = disp;
        self.node_changed = true;
        self
    }

    fn flex_row(&mut self) -> &mut Self {
        self.node.display = ui::Display::Flex;
        self.node.flex_direction = ui::FlexDirection::Row;
        self.node_changed = true;
        self
    }

    fn flex_column(&mut self) -> &mut Self {
        self.node.display = ui::Display::Flex;
        self.node.flex_direction = ui::FlexDirection::Column;
        self.node_changed = true;
        self
    }

    fn position(&mut self, pos: ui::PositionType) -> &mut Self {
        self.node.position_type = pos;
        self.node_changed = true;
        self
    }

    fn overflow(&mut self, ov: ui::OverflowAxis) -> &mut Self {
        self.node.overflow.x = ov;
        self.node.overflow.y = ov;
        self.node_changed = true;
        self
    }

    fn overflow_x(&mut self, ov: ui::OverflowAxis) -> &mut Self {
        self.node.overflow.x = ov;
        self.node_changed = true;
        self
    }

    fn overflow_y(&mut self, ov: ui::OverflowAxis) -> &mut Self {
        self.node.overflow.y = ov;
        self.node_changed = true;
        self
    }

    fn left(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.left = length.to_val();
        self.node_changed = true;
        self
    }

    fn right(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.right = length.to_val();
        self.node_changed = true;
        self
    }

    fn top(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.top = length.to_val();
        self.node_changed = true;
        self
    }

    fn bottom(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.bottom = length.to_val();
        self.node_changed = true;
        self
    }

    fn width(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.width = length.to_val();
        self.node_changed = true;
        self
    }

    fn height(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.height = length.to_val();
        self.node_changed = true;
        self
    }

    fn min_width(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.min_width = length.to_val();
        self.node_changed = true;
        self
    }

    fn min_height(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.min_height = length.to_val();
        self.node_changed = true;
        self
    }

    fn max_width(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.max_width = length.to_val();
        self.node_changed = true;
        self
    }

    fn max_height(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.max_height = length.to_val();
        self.node_changed = true;
        self
    }

    fn aspect_ratio(&mut self, length: impl OptFloatParam) -> &mut Self {
        self.node.aspect_ratio = length.to_val();
        self.node_changed = true;
        self
    }

    fn margin(&mut self, rect: impl UiRectParam) -> &mut Self {
        self.node.margin = rect.to_uirect();
        self.node_changed = true;
        self
    }

    fn margin_left(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.margin.left = length.to_val();
        self.node_changed = true;
        self
    }

    fn margin_right(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.margin.right = length.to_val();
        self.node_changed = true;
        self
    }

    fn margin_top(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.margin.top = length.to_val();
        self.node_changed = true;
        self
    }

    fn margin_bottom(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.margin.bottom = length.to_val();
        self.node_changed = true;
        self
    }

    fn padding(&mut self, rect: impl UiRectParam) -> &mut Self {
        self.node.padding = rect.to_uirect();
        self.node_changed = true;
        self
    }

    fn padding_left(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.padding.left = length.to_val();
        self.node_changed = true;
        self
    }

    fn padding_right(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.padding.right = length.to_val();
        self.node_changed = true;
        self
    }

    fn padding_top(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.padding.top = length.to_val();
        self.node_changed = true;
        self
    }

    fn padding_bottom(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.padding.bottom = length.to_val();
        self.node_changed = true;
        self
    }

    fn border(&mut self, rect: impl UiRectParam) -> &mut Self {
        self.node.border = rect.to_uirect();
        self.node_changed = true;
        self
    }

    fn border_left(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.border.left = length.to_val();
        self.node_changed = true;
        self
    }

    fn border_right(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.border.right = length.to_val();
        self.node_changed = true;
        self
    }

    fn border_top(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.border.top = length.to_val();
        self.node_changed = true;
        self
    }

    fn border_bottom(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.border.bottom = length.to_val();
        self.node_changed = true;
        self
    }

    fn flex_direction(&mut self, dir: ui::FlexDirection) -> &mut Self {
        self.node.flex_direction = dir;
        self.node_changed = true;
        self
    }

    fn flex_wrap(&mut self, w: ui::FlexWrap) -> &mut Self {
        self.node.flex_wrap = w;
        self.node_changed = true;
        self
    }

    fn flex(&mut self, grow: f32, shrink: f32, basis: impl LengthParam) -> &mut Self {
        self.node.flex_grow = grow;
        self.node.flex_shrink = shrink;
        self.node.flex_basis = basis.to_val();
        self.node_changed = true;
        self
    }

    fn flex_grow(&mut self, n: f32) -> &mut Self {
        self.node.flex_grow = n;
        self.node_changed = true;
        self
    }

    fn flex_shrink(&mut self, n: f32) -> &mut Self {
        self.node.flex_shrink = n;
        self.node_changed = true;
        self
    }

    fn flex_basis(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.flex_basis = length.to_val();
        self.node_changed = true;
        self
    }

    fn row_gap(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.row_gap = length.to_val();
        self.node_changed = true;
        self
    }

    fn column_gap(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.column_gap = length.to_val();
        self.node_changed = true;
        self
    }

    fn gap(&mut self, length: impl LengthParam) -> &mut Self {
        self.node.row_gap = length.to_val();
        self.node.column_gap = self.node.row_gap;
        self.node_changed = true;
        self
    }

    fn align_items(&mut self, align: ui::AlignItems) -> &mut Self {
        self.node.align_items = align;
        self.node_changed = true;
        self
    }

    fn align_self(&mut self, align: ui::AlignSelf) -> &mut Self {
        self.node.align_self = align;
        self.node_changed = true;
        self
    }

    fn align_content(&mut self, align: ui::AlignContent) -> &mut Self {
        self.node.align_content = align;
        self.node_changed = true;
        self
    }

    fn justify_items(&mut self, justify: ui::JustifyItems) -> &mut Self {
        self.node.justify_items = justify;
        self.node_changed = true;
        self
    }

    fn justify_self(&mut self, justify: ui::JustifySelf) -> &mut Self {
        self.node.justify_self = justify;
        self.node_changed = true;
        self
    }

    fn justify_content(&mut self, justify: ui::JustifyContent) -> &mut Self {
        self.node.justify_content = justify;
        self.node_changed = true;
        self
    }

    fn grid_auto_flow(&mut self, flow: ui::GridAutoFlow) -> &mut Self {
        self.node.grid_auto_flow = flow;
        self.node_changed = true;
        self
    }

    fn grid_template_rows(&mut self, rows: Vec<ui::RepeatedGridTrack>) -> &mut Self {
        self.node.grid_template_rows = rows;
        self.node_changed = true;
        self
    }

    fn grid_template_columns(&mut self, columns: Vec<ui::RepeatedGridTrack>) -> &mut Self {
        self.node.grid_template_columns = columns;
        self.node_changed = true;
        self
    }

    fn grid_auto_rows(&mut self, rows: Vec<ui::GridTrack>) -> &mut Self {
        self.node.grid_auto_rows = rows;
        self.node_changed = true;
        self
    }

    fn grid_auto_columns(&mut self, columns: Vec<ui::GridTrack>) -> &mut Self {
        self.node.grid_auto_columns = columns;
        self.node_changed = true;
        self
    }

    fn grid_row(&mut self, val: ui::GridPlacement) -> &mut Self {
        self.node.grid_row = val;
        self.node_changed = true;
        self
    }

    fn grid_row_start(&mut self, val: i16) -> &mut Self {
        self.node.grid_row = self.node.grid_row.set_start(val);
        self.node_changed = true;
        self
    }

    fn grid_row_span(&mut self, val: u16) -> &mut Self {
        self.node.grid_row = self.node.grid_row.set_span(val);
        self.node_changed = true;
        self
    }

    fn grid_row_end(&mut self, val: i16) -> &mut Self {
        self.node.grid_row = self.node.grid_row.set_end(val);
        self.node_changed = true;
        self
    }

    fn grid_column(&mut self, val: ui::GridPlacement) -> &mut Self {
        self.node.grid_column = val;
        self.node_changed = true;
        self
    }

    fn grid_column_start(&mut self, val: i16) -> &mut Self {
        self.node.grid_column = self.node.grid_column.set_start(val);
        self.node_changed = true;
        self
    }

    fn grid_column_span(&mut self, val: u16) -> &mut Self {
        self.node.grid_column = self.node.grid_column.set_span(val);
        self.node_changed = true;
        self
    }

    fn grid_column_end(&mut self, val: i16) -> &mut Self {
        self.node.grid_column = self.node.grid_column.set_end(val);
        self.node_changed = true;
        self
    }
}
