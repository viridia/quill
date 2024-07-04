use bevy_mod_stylebuilder::{StyleBuilder, StyleBuilderFont};

/// Default text style for UI.
pub fn text_default(ss: &mut StyleBuilder) {
    ss.font("embedded://bevy_quill_obsidian/assets/fonts/Open_Sans/static/OpenSans-Medium.ttf")
        .font_size(16);
}

/// When we need to emphasize a label
pub fn text_strong(ss: &mut StyleBuilder) {
    ss.font("embedded://bevy_quill_obsidian/assets/fonts/Open_Sans/static/OpenSans-Bold.ttf")
        .font_size(16);
}
