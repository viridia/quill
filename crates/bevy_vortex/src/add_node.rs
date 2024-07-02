use bevy_quill::*;
use quill_obsidian::controls::*;

use crate::catalog::SelectedCatalogEntry;

/// Displays the list of available operators, by category.
#[derive(Clone, PartialEq)]
pub struct AddNodeButton;

impl ViewTemplate for AddNodeButton {
    type View = impl View;

    fn create(&self, cx: &mut bevy_quill::Cx) -> Self::View {
        let catalog = cx.use_resource::<SelectedCatalogEntry>();
        Button::new()
            .disabled(catalog.0.is_none())
            .children("Add node")
    }
}
