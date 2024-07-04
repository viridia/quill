use bevy::{color::Srgba, prelude::*, ui};
use bevy_mod_stylebuilder::*;
use bevy_quill::*;

use crate::colors;

use super::Swatch;

fn style_swatch_grid(ss: &mut StyleBuilder) {
    ss.border(1)
        .min_width(16)
        .min_height(16)
        .gap(3)
        .display(ui::Display::Grid)
        .grid_auto_rows(vec![ui::GridTrack::default()])
        .border(0)
        .color(colors::FOREGROUND);
}

fn style_swatch(ss: &mut StyleBuilder) {
    ss.min_width(12).min_height(12);
}

fn style_empty_slot(ss: &mut StyleBuilder) {
    ss.border(1)
        .min_width(16)
        .min_height(16)
        .border_color(colors::U2.lighter(0.01));
}

/// Color swatch widget. This displays a solid color, and can also display a checkerboard
/// pattern behind the color if it has an alpha of less than 1.
#[derive(Clone, PartialEq)]
pub struct SwatchGrid {
    /// Color to display.
    /// TODO: Should this be `Color` instead? How will we serialize?
    pub colors: Vec<Srgba>,

    /// Number of rows and columns
    pub grid_size: UVec2,

    /// The currently selected color.
    pub selected: Srgba,

    /// Additional styles to be applied to the grid.
    pub style: StyleHandle,

    /// Callback called when a swatch is clicked
    pub on_change: Option<Callback<Srgba>>,
}

impl SwatchGrid {
    /// Create a new swatch.
    pub fn new(colors: impl Into<Vec<Srgba>>) -> Self {
        Self::default().colors(colors.into())
    }

    /// Set the color to display.
    pub fn colors(mut self, colors: impl Into<Vec<Srgba>>) -> Self {
        self.colors = colors.into();
        self
    }

    /// Set which color is selected.
    pub fn selected(mut self, selected: impl Into<Srgba>) -> Self {
        self.selected = selected.into();
        self
    }

    /// Set the number of rows and columns in the grid.
    pub fn grid_size(mut self, grid_size: UVec2) -> Self {
        self.grid_size = grid_size;
        self
    }

    /// Set additional styles to be applied to the button.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set the callback called when clicked.
    pub fn on_change(mut self, on_click: Callback<Srgba>) -> Self {
        self.on_change = Some(on_click);
        self
    }
}

impl Default for SwatchGrid {
    fn default() -> Self {
        Self {
            colors: Vec::new(),
            grid_size: UVec2::new(8, 8),
            selected: Srgba::default(),
            style: Default::default(),
            on_change: None,
        }
    }
}

impl ViewTemplate for SwatchGrid {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let num_cells = (self.grid_size.x * self.grid_size.y) as usize;
        let grid_size = self.grid_size;
        let selected = self.selected;
        let on_change = self.on_change;

        let on_click = cx.create_callback(move |color: In<Srgba>, world: &mut World| {
            if let Some(on_change) = on_change.as_ref() {
                world.run_callback(*on_change, *color)
            }
        });

        let num_colors = self.colors.len().min(num_cells);
        let padding_len = num_cells - num_colors;

        Element::<NodeBundle>::new()
            .named("SwatchGrid")
            .style((
                style_swatch_grid,
                move |ss: &mut StyleBuilder| {
                    ss.grid_template_columns(vec![ui::RepeatedGridTrack::flex(
                        grid_size.x as u16,
                        1.,
                    )])
                    .grid_template_rows(vec![ui::RepeatedGridTrack::flex(grid_size.y as u16, 1.)]);
                },
                self.style.clone(),
            ))
            .children((
                // Generate cells for each color
                For::each(self.colors[0..num_colors].to_owned(), move |color| {
                    Swatch::new(*color)
                        .selected(selected == *color)
                        .style(style_swatch)
                        .on_click(on_click)
                }),
                // Pad with empty cells
                For::each(0..padding_len, move |_| {
                    Element::<NodeBundle>::new().style(style_empty_slot)
                }),
            ))
    }
}
