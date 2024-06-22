use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode, Focus,
    },
    color::Luminance,
    prelude::*,
    ui,
};
use bevy_mod_picking::{events::PointerCancel, prelude::*};
use bevy_mod_stylebuilder::*;
use bevy_quill::*;

use crate::{
    colors,
    cursor::StyleBuilderCursor,
    focus::{KeyPressEvent, TabIndex},
    hooks::{UseIsFocus, UseIsHover},
    typography,
};

use super::{Disabled, IsDisabled};

fn style_checkbox(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .justify_content(ui::JustifyContent::Center)
        .align_items(ui::AlignItems::Center)
        .align_content(ui::AlignContent::Center)
        .gap(4)
        .color(colors::FOREGROUND);
}

fn style_checkbox_border(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .width(16)
        .height(16)
        .border_radius(3.0)
        .cursor(CursorIcon::Pointer);
}

fn style_checkbox_inner(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .background_image("obsidian_ui://textures/checkmark.png")
        // .background_color(colors::FOREGROUND)
        .position(ui::PositionType::Absolute)
        .left(2)
        .top(2)
        .width(12)
        .height(12);
}

fn style_checkbox_label(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .justify_content(ui::JustifyContent::FlexStart)
        .align_items(ui::AlignItems::Center)
        .color(colors::FOREGROUND);
}

/// A checkbox widget.
#[derive(Default, Clone, PartialEq)]
pub struct Checkbox {
    /// Whether the checkbox is checked.
    pub checked: bool,

    /// Whether the checkbox is disabled.
    pub disabled: bool,

    /// The content to display inside the button.
    pub label: ChildViews,

    /// Additional styles to be applied to the button.
    pub style: StyleHandle,

    /// Callback called when clicked
    pub on_change: Option<Callback<bool>>,

    /// The tab index of the checkbox (default 0).
    pub tab_index: i32,
}

impl Checkbox {
    /// Create a new checkbox.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the checked state of the checkbox.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Set the disabled state of the checkbox.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the label of the checkbox.
    pub fn label(mut self, label: impl IntoChildViews) -> Self {
        self.label = label.into_child_views();
        self
    }

    /// Set the style of the checkbox.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set the on_change callback of the checkbox.
    pub fn on_change(mut self, on_change: Callback<bool>) -> Self {
        self.on_change = Some(on_change);
        self
    }

    /// Set the tab index of the checkbox.
    pub fn tab_index(mut self, tab_index: i32) -> Self {
        self.tab_index = tab_index;
        self
    }
}

#[derive(Component)]
pub(crate) struct Checked;

impl ViewTemplate for Checkbox {
    type View = impl View;

    /// Construct a checkbox widget.
    fn create(&self, cx: &mut Cx) -> Self::View {
        let id = cx.create_entity();
        let pressed = cx.create_mutable::<bool>(false);
        let hovering = cx.is_hovered(id);
        let focused = cx.is_focus_visible(id);
        let checked = self.checked;
        let on_change = self.on_change;

        Element::<NodeBundle>::for_entity(id)
            .named("Checkbox")
            .style((typography::text_default, style_checkbox, self.style.clone()))
            .insert(TabIndex, self.tab_index)
            // The reason we do this is to avoid capturing `checked` and `disabled` in the
            // bevy_mod_picking event handlers, as this would require removing and inserting
            // them every time the checked or disabled state changes.
            .insert_if(self.disabled, || Disabled)
            .insert_if(self.checked, || Checked)
            .insert(
                move |_| {
                    (
                        AccessibilityNode::from(NodeBuilder::new(Role::CheckBox)),
                        On::<Pointer<Click>>::run(move |world: &mut World| {
                            let mut focus = world.get_resource_mut::<Focus>().unwrap();
                            focus.0 = Some(id);
                            if !world.is_disabled(id) {
                                let next_checked = world.get::<Checked>(id).is_some();
                                if let Some(on_click) = on_change {
                                    world.run_callback(on_click, !next_checked);
                                }
                            }
                        }),
                        On::<Pointer<DragStart>>::run(move |world: &mut World| {
                            if !world.is_disabled(id) {
                                pressed.set(world, true);
                            }
                        }),
                        On::<Pointer<DragEnd>>::run(move |world: &mut World| {
                            if !world.is_disabled(id) {
                                pressed.set(world, false);
                            }
                        }),
                        On::<Pointer<DragEnter>>::run(move |world: &mut World| {
                            if !world.is_disabled(id) {
                                pressed.set(world, true);
                            }
                        }),
                        On::<Pointer<DragLeave>>::run(move |world: &mut World| {
                            if !world.is_disabled(id) {
                                pressed.set(world, false);
                            }
                        }),
                        On::<Pointer<PointerCancel>>::run(move |world: &mut World| {
                            println!("PointerCancel");
                            if !world.is_disabled(id) {
                                pressed.set(world, false);
                            }
                        }),
                        On::<KeyPressEvent>::run(move |world: &mut World| {
                            if !world.is_disabled(id) {
                                let mut event = world
                                    .get_resource_mut::<ListenerInput<KeyPressEvent>>()
                                    .unwrap();
                                if !event.repeat
                                    && (event.key_code == KeyCode::Enter
                                        || event.key_code == KeyCode::Space)
                                {
                                    event.stop_propagation();
                                    let next_checked = world.get::<Checked>(id).is_some();
                                    if let Some(on_click) = on_change {
                                        world.run_callback(on_click, !next_checked);
                                    }
                                }
                            }
                        }),
                    )
                },
                (),
            )
            .children((
                Element::<NodeBundle>::new()
                    .named("Checkbox::Border")
                    .style(style_checkbox_border)
                    .style_effect(
                        |(checked, pressed, hovering), sb| {
                            let color = match (checked, pressed, hovering) {
                                (true, true, _) => colors::ACCENT.darker(0.1),
                                (true, false, true) => colors::ACCENT.darker(0.15),
                                (true, _, _) => colors::ACCENT.darker(0.2),
                                (false, true, _) => colors::U1.lighter(0.005),
                                (false, false, true) => colors::U1.lighter(0.002),
                                (false, false, false) => colors::U1,
                            };
                            sb.background_color(color);
                        },
                        (checked, pressed.get(cx), hovering),
                    )
                    .style_effect(
                        |focused, sb| {
                            if focused {
                                sb.outline_color(colors::FOCUS)
                                    .outline_offset(1.0)
                                    .width(2.0);
                            } else {
                                sb.outline_color(Option::<Color>::None);
                            }
                        },
                        focused,
                    )
                    .children(Cond::new(
                        checked,
                        // (),
                        Element::<NodeBundle>::new().style(style_checkbox_inner),
                        (),
                    )),
                Element::<NodeBundle>::new()
                    .style(style_checkbox_label)
                    .style_effect(
                        |disabled, sb| {
                            // info!("Checkbox disabled: {}", disabled);
                            // This doesn't work because inherited text styles don't change.
                            sb.color(if disabled {
                                colors::FOREGROUND.with_alpha(0.3)
                            } else {
                                colors::FOREGROUND
                            });
                        },
                        self.disabled,
                    )
                    .children(self.label.clone()),
            ))
    }
}
