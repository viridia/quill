use bevy_mod_stylebuilder::{StyleBuilder, StyleBuilderFont};

/// Default text style for UI.
pub fn text_default(ss: &mut StyleBuilder) {
    ss.font("embedded://quill_obsidian/assets/fonts/Open_Sans/static/OpenSans-Medium.ttf")
        .font_size(16);
}
