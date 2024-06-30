use super::Icon;
use crate::{
    animation::{AnimatedRotation, AnimatedTransition},
    colors,
    cursor::StyleBuilderCursor,
    focus::{KeyPressEvent, TabIndex},
    hooks::{UseIsFocus, UseIsHover},
    size::Size,
};
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode, Focus,
    },
    prelude::*,
    ui,
};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill::*;

fn style_toggle(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .justify_content(ui::JustifyContent::Center)
        .align_items(ui::AlignItems::Center)
        .align_content(ui::AlignContent::Center)
        .color(colors::FOREGROUND)
        .cursor(CursorIcon::Pointer);
}

/// A widget which displays small toggleable chevron that can be used to control whether
/// a panel is visible or hidden.
#[derive(Default, Clone, PartialEq)]
pub struct DisclosureToggle {
    /// Whether the toggle is in an expanded state.
    pub expanded: bool,

    /// Button size.
    pub size: Size,

    /// Whether the button is disabled.
    pub disabled: bool,

    /// Additional styles to be applied to the button.
    pub style: StyleHandle,

    /// Callback called when the state is toggled
    pub on_change: Option<Callback<bool>>,

    /// The tab index of the button (default 0).
    pub tab_index: i32,

    /// If true, set focus to this button when it's added to the UI.
    pub autofocus: bool,
}

impl DisclosureToggle {
    /// Construct a new `DisclosureToggle`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the expanded state of the button.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set the button size.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Set the button disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the additional styles for the button.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set callback when clicked
    pub fn on_change(mut self, callback: Callback<bool>) -> Self {
        self.on_change = Some(callback);
        self
    }

    /// Set the tab index of the button.
    pub fn tab_index(mut self, tab_index: i32) -> Self {
        self.tab_index = tab_index;
        self
    }

    /// Set whether to autofocus the button when it's added to the UI.
    pub fn autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }
}

impl ViewTemplate for DisclosureToggle {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let id = cx.create_entity();
        let hovering = cx.is_hovered(id);
        let focused = cx.is_focus_visible(id);
        let on_change = self.on_change;

        Element::<NodeBundle>::for_entity(id)
            .named("DisclosureToggle")
            .style((style_toggle, self.style.clone()))
            .insert((
                TabIndex(self.tab_index),
                AccessibilityNode::from(NodeBuilder::new(Role::CheckBox)),
            ))
            .insert_dyn(
                move |(disabled, expanded)| {
                    (
                        On::<Pointer<Click>>::run(move |world: &mut World| {
                            let mut focus = world.get_resource_mut::<Focus>().unwrap();
                            focus.0 = Some(id);
                            if !disabled {
                                let next_checked = expanded;
                                if let Some(on_click) = on_change {
                                    world.run_callback(on_click, !next_checked);
                                }
                            }
                        }),
                        On::<KeyPressEvent>::run({
                            move |world: &mut World| {
                                if !disabled {
                                    let mut event = world
                                        .get_resource_mut::<ListenerInput<KeyPressEvent>>()
                                        .unwrap();
                                    if !event.repeat
                                        && (event.key_code == KeyCode::Enter
                                            || event.key_code == KeyCode::Space)
                                    {
                                        event.stop_propagation();
                                        let next_checked = expanded;
                                        if let Some(on_click) = on_change {
                                            world.run_callback(on_click, !next_checked);
                                        }
                                    }
                                }
                            }
                        }),
                    )
                },
                (self.disabled, self.expanded),
            )
            .effect(
                move |cx, en, checked| {
                    let mut entt = cx.world_mut().entity_mut(en);
                    let angle = if checked {
                        std::f32::consts::PI * 0.5
                    } else {
                        0.
                    };
                    let target = Quat::from_rotation_z(angle);
                    AnimatedTransition::<AnimatedRotation>::start(&mut entt, target, 0.3);
                },
                self.expanded,
            )
            .style_dyn(
                move |focused, sb| {
                    match focused {
                        true => {
                            sb.outline_color(colors::FOCUS)
                                .outline_width(2)
                                .outline_offset(2);
                        }
                        false => {
                            sb.outline_color(Option::<Color>::None);
                        }
                    };
                },
                focused,
            )
            .children(
                Icon::new("embedded://quill_obsidian/assets/icons/chevron_right.png")
                    .color({
                        match (self.disabled, hovering) {
                            (true, _) => Color::from(colors::DIM).with_alpha(0.2),
                            (false, true) => Color::from(colors::FOREGROUND),
                            (false, false) => Color::from(colors::DIM),
                        }
                    })
                    .size(match self.size {
                        Size::Xl => Vec2::splat(24.),
                        Size::Lg => Vec2::splat(20.),
                        Size::Md => Vec2::splat(18.),
                        Size::Sm => Vec2::splat(16.),
                        Size::Xs => Vec2::splat(13.),
                        Size::Xxs => Vec2::splat(12.),
                        Size::Xxxs => Vec2::splat(11.),
                    })
                    .style(|ss: &mut StyleBuilder| {
                        ss.margin_right(2);
                    }),
            )
    }
}
