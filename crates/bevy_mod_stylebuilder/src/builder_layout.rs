use super::builder::{LengthParam, OptFloatParam, StyleBuilder, UiRectParam};
use bevy::ui;

#[allow(missing_docs)]
pub trait StyleBuilderLayout {
    fn display(&mut self, disp: ui::Display) -> &mut Self;
    fn position(&mut self, pos: ui::PositionType) -> &mut Self;
    fn overflow(&mut self, ov: ui::OverflowAxis) -> &mut Self;
    fn overflow_x(&mut self, ov: ui::OverflowAxis) -> &mut Self;
    fn overflow_y(&mut self, ov: ui::OverflowAxis) -> &mut Self;
    fn direction(&mut self, dir: ui::Direction) -> &mut Self;
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
        self.style.display = disp;
        self.style_changed = true;
        self
    }

    fn position(&mut self, pos: ui::PositionType) -> &mut Self {
        self.style.position_type = pos;
        self.style_changed = true;
        self
    }

    fn overflow(&mut self, ov: ui::OverflowAxis) -> &mut Self {
        self.style.overflow.x = ov;
        self.style.overflow.y = ov;
        self.style_changed = true;
        self
    }

    fn overflow_x(&mut self, ov: ui::OverflowAxis) -> &mut Self {
        self.style.overflow.x = ov;
        self.style_changed = true;
        self
    }

    fn overflow_y(&mut self, ov: ui::OverflowAxis) -> &mut Self {
        self.style.overflow.y = ov;
        self.style_changed = true;
        self
    }

    fn direction(&mut self, dir: ui::Direction) -> &mut Self {
        self.style.direction = dir;
        self.style_changed = true;
        self
    }

    fn left(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.left = length.to_val();
        self.style_changed = true;
        self
    }

    fn right(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.right = length.to_val();
        self.style_changed = true;
        self
    }

    fn top(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.top = length.to_val();
        self.style_changed = true;
        self
    }

    fn bottom(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.bottom = length.to_val();
        self.style_changed = true;
        self
    }

    fn width(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.width = length.to_val();
        self.style_changed = true;
        self
    }

    fn height(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.height = length.to_val();
        self.style_changed = true;
        self
    }

    fn min_width(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.min_width = length.to_val();
        self.style_changed = true;
        self
    }

    fn min_height(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.min_height = length.to_val();
        self.style_changed = true;
        self
    }

    fn max_width(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.max_width = length.to_val();
        self.style_changed = true;
        self
    }

    fn max_height(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.max_height = length.to_val();
        self.style_changed = true;
        self
    }

    fn aspect_ratio(&mut self, length: impl OptFloatParam) -> &mut Self {
        self.style.aspect_ratio = length.to_val();
        self.style_changed = true;
        self
    }

    fn margin(&mut self, rect: impl UiRectParam) -> &mut Self {
        self.style.margin = rect.to_uirect();
        self.style_changed = true;
        self
    }

    fn margin_left(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.margin.left = length.to_val();
        self.style_changed = true;
        self
    }

    fn margin_right(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.margin.right = length.to_val();
        self.style_changed = true;
        self
    }

    fn margin_top(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.margin.top = length.to_val();
        self.style_changed = true;
        self
    }

    fn margin_bottom(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.margin.bottom = length.to_val();
        self.style_changed = true;
        self
    }

    fn padding(&mut self, rect: impl UiRectParam) -> &mut Self {
        self.style.padding = rect.to_uirect();
        self.style_changed = true;
        self
    }

    fn padding_left(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.padding.left = length.to_val();
        self.style_changed = true;
        self
    }

    fn padding_right(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.padding.right = length.to_val();
        self.style_changed = true;
        self
    }

    fn padding_top(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.padding.top = length.to_val();
        self.style_changed = true;
        self
    }

    fn padding_bottom(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.padding.bottom = length.to_val();
        self.style_changed = true;
        self
    }

    fn border(&mut self, rect: impl UiRectParam) -> &mut Self {
        self.style.border = rect.to_uirect();
        self.style_changed = true;
        self
    }

    fn border_left(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.border.left = length.to_val();
        self.style_changed = true;
        self
    }

    fn border_right(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.border.right = length.to_val();
        self.style_changed = true;
        self
    }

    fn border_top(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.border.top = length.to_val();
        self.style_changed = true;
        self
    }

    fn border_bottom(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.border.bottom = length.to_val();
        self.style_changed = true;
        self
    }

    fn flex_direction(&mut self, dir: ui::FlexDirection) -> &mut Self {
        self.style.flex_direction = dir;
        self.style_changed = true;
        self
    }

    fn flex_wrap(&mut self, w: ui::FlexWrap) -> &mut Self {
        self.style.flex_wrap = w;
        self.style_changed = true;
        self
    }

    fn flex(&mut self, grow: f32, shrink: f32, basis: impl LengthParam) -> &mut Self {
        self.style.flex_grow = grow;
        self.style.flex_shrink = shrink;
        self.style.flex_basis = basis.to_val();
        self.style_changed = true;
        self
    }

    fn flex_grow(&mut self, n: f32) -> &mut Self {
        self.style.flex_grow = n;
        self.style_changed = true;
        self
    }

    fn flex_shrink(&mut self, n: f32) -> &mut Self {
        self.style.flex_shrink = n;
        self.style_changed = true;
        self
    }

    fn flex_basis(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.flex_basis = length.to_val();
        self.style_changed = true;
        self
    }

    fn row_gap(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.row_gap = length.to_val();
        self.style_changed = true;
        self
    }

    fn column_gap(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.column_gap = length.to_val();
        self.style_changed = true;
        self
    }

    fn gap(&mut self, length: impl LengthParam) -> &mut Self {
        self.style.row_gap = length.to_val();
        self.style.column_gap = self.style.row_gap;
        self.style_changed = true;
        self
    }

    fn align_items(&mut self, align: ui::AlignItems) -> &mut Self {
        self.style.align_items = align;
        self.style_changed = true;
        self
    }

    fn align_self(&mut self, align: ui::AlignSelf) -> &mut Self {
        self.style.align_self = align;
        self.style_changed = true;
        self
    }

    fn align_content(&mut self, align: ui::AlignContent) -> &mut Self {
        self.style.align_content = align;
        self.style_changed = true;
        self
    }

    fn justify_items(&mut self, justify: ui::JustifyItems) -> &mut Self {
        self.style.justify_items = justify;
        self.style_changed = true;
        self
    }

    fn justify_self(&mut self, justify: ui::JustifySelf) -> &mut Self {
        self.style.justify_self = justify;
        self.style_changed = true;
        self
    }

    fn justify_content(&mut self, justify: ui::JustifyContent) -> &mut Self {
        self.style.justify_content = justify;
        self.style_changed = true;
        self
    }

    fn grid_auto_flow(&mut self, flow: ui::GridAutoFlow) -> &mut Self {
        self.style.grid_auto_flow = flow;
        self.style_changed = true;
        self
    }

    fn grid_template_rows(&mut self, rows: Vec<ui::RepeatedGridTrack>) -> &mut Self {
        self.style.grid_template_rows = rows;
        self.style_changed = true;
        self
    }

    fn grid_template_columns(&mut self, columns: Vec<ui::RepeatedGridTrack>) -> &mut Self {
        self.style.grid_template_columns = columns;
        self.style_changed = true;
        self
    }

    fn grid_auto_rows(&mut self, rows: Vec<ui::GridTrack>) -> &mut Self {
        self.style.grid_auto_rows = rows;
        self.style_changed = true;
        self
    }

    fn grid_auto_columns(&mut self, columns: Vec<ui::GridTrack>) -> &mut Self {
        self.style.grid_auto_columns = columns;
        self.style_changed = true;
        self
    }

    fn grid_row(&mut self, val: ui::GridPlacement) -> &mut Self {
        self.style.grid_row = val;
        self.style_changed = true;
        self
    }

    fn grid_row_start(&mut self, val: i16) -> &mut Self {
        self.style.grid_row = self.style.grid_row.set_start(val);
        self.style_changed = true;
        self
    }

    fn grid_row_span(&mut self, val: u16) -> &mut Self {
        self.style.grid_row = self.style.grid_row.set_span(val);
        self.style_changed = true;
        self
    }

    fn grid_row_end(&mut self, val: i16) -> &mut Self {
        self.style.grid_row = self.style.grid_row.set_end(val);
        self.style_changed = true;
        self
    }

    fn grid_column(&mut self, val: ui::GridPlacement) -> &mut Self {
        self.style.grid_column = val;
        self.style_changed = true;
        self
    }

    fn grid_column_start(&mut self, val: i16) -> &mut Self {
        self.style.grid_column = self.style.grid_column.set_start(val);
        self.style_changed = true;
        self
    }

    fn grid_column_span(&mut self, val: u16) -> &mut Self {
        self.style.grid_column = self.style.grid_column.set_span(val);
        self.style_changed = true;
        self
    }

    fn grid_column_end(&mut self, val: i16) -> &mut Self {
        self.style.grid_column = self.style.grid_column.set_end(val);
        self.style_changed = true;
        self
    }
}
