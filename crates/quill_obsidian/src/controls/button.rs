use super::{Disabled, IsDisabled};
use crate::{
    colors,
    cursor::StyleBuilderCursor,
    focus::{AutoFocus, KeyPressEvent, TabIndex},
    hooks::{UseIsFocus, UseIsHover},
    size::Size,
    typography, RoundedCorners,
};
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
use bevy_quill::{
    Callback, ChildViews, Cx, Element, IntoChildViews, RunCallback, View, ViewTemplate,
};

/// The variant determines the button's color scheme
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ButtonVariant {
    /// The default button apperance.
    #[default]
    Default,

    /// A more prominent, "call to action", appearance.
    Primary,

    /// An appearance indicating a potentially dangerous action.
    Danger,

    /// A button that is in a "toggled" state.
    Selected,
}

pub(crate) fn style_button(ss: &mut StyleBuilder) {
    ss.border(1)
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .justify_content(ui::JustifyContent::Center)
        .align_items(ui::AlignItems::Center)
        .align_content(ui::AlignContent::Center)
        .padding((12, 0))
        .border(0)
        .color(colors::FOREGROUND)
        .cursor(CursorIcon::Pointer);
}

pub(crate) fn style_button_bg(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Grid)
        .position(ui::PositionType::Absolute)
        .left(0)
        .right(0)
        .top(0)
        .bottom(0);
}

/// Button widget
#[derive(Default, PartialEq, Clone)]
pub struct Button {
    /// Color variant - default, primary or danger.
    pub variant: ButtonVariant,

    /// Button size.
    pub size: Size,

    /// Whether the button is disabled.
    pub disabled: bool,

    /// The content to display inside the button.
    pub children: ChildViews,

    /// Additional styles to be applied to the button.
    pub style: StyleHandle,

    /// Callback called when clicked
    pub on_click: Option<Callback>,

    /// The tab index of the button (default 0).
    pub tab_index: i32,

    /// Which corners to render rounded.
    pub corners: RoundedCorners,

    /// If true, set focus to this button when it's added to the UI.
    pub autofocus: bool,

    /// If true, render the button in a 'minimal' style with no background and reduced padding.
    pub minimal: bool,
}

impl Button {
    /// Construct a new `Button`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the button color variant.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set whether to render the button in a 'minimal' style with no background and reduced padding.
    pub fn minimal(mut self, minimal: bool) -> Self {
        self.minimal = minimal;
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

    /// Set the child views for this element.
    pub fn children(mut self, children: impl IntoChildViews) -> Self {
        self.children = children.into_child_views();
        self
    }

    /// Set the additional styles for the button.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set callback when clicked
    pub fn on_click(mut self, callback: Callback) -> Self {
        self.on_click = Some(callback);
        self
    }

    /// Set the tab index of the button.
    pub fn tab_index(mut self, tab_index: i32) -> Self {
        self.tab_index = tab_index;
        self
    }

    /// Set which corners to render rounded.
    pub fn corners(mut self, corners: RoundedCorners) -> Self {
        self.corners = corners;
        self
    }

    /// Set whether to autofocus the button when it's added to the UI.
    pub fn autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }
}

impl ViewTemplate for Button {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        let id = cx.create_entity();
        let variant = self.variant;
        let pressed = cx.create_mutable::<bool>(false);
        let hovering = cx.use_is_hover(id);
        let focused = cx.use_is_focus_visible(id);

        let disabled = self.disabled;
        let corners = self.corners;
        let minimal = self.minimal;

        let size = self.size;
        let on_click = self.on_click;

        Element::<NodeBundle>::for_entity(id)
            .named("Button")
            .style((
                typography::text_default,
                style_button,
                move |ss: &mut StyleBuilder| {
                    ss.min_height(size.height()).font_size(size.font_size());
                    if minimal {
                        ss.padding(0);
                    } else {
                        ss.padding((size.font_size() * 0.75, 0));
                    }
                },
                self.style.clone(),
            ))
            .insert(TabIndex, self.tab_index)
            .insert(
                move |_| {
                    (
                        AccessibilityNode::from(NodeBuilder::new(Role::Button)),
                        On::<Pointer<Click>>::run(move |world: &mut World| {
                            let mut focus = world.get_resource_mut::<Focus>().unwrap();
                            focus.0 = Some(id);
                            if !world.is_disabled(id) {
                                let mut event = world
                                    .get_resource_mut::<ListenerInput<Pointer<Click>>>()
                                    .unwrap();
                                event.stop_propagation();
                                if let Some(on_click) = on_click {
                                    world.run_callback(on_click, ());
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
                                    if let Some(on_click) = on_click {
                                        world.run_callback(on_click, ());
                                    }
                                }
                            }
                        }),
                    )
                },
                (),
            )
            .insert_if(self.disabled, || Disabled)
            .insert_if(self.autofocus, || AutoFocus)
            .children((
                Element::<NodeBundle>::new()
                    .named("Button::Background")
                    .style(style_button_bg)
                    .insert(
                        move |size| corners.to_border_radius(size.border_radius()),
                        self.size,
                    )
                    .style_effect(
                        |(minimal, variant, disabled, pressed, hovering), sb| {
                            let color = if minimal {
                                colors::TRANSPARENT
                            } else {
                                button_bg_color(variant, disabled, pressed, hovering)
                            };
                            sb.background_color(color);
                        },
                        (minimal, variant, disabled, pressed.get(cx), hovering),
                    )
                    .style_effect(
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
                    ),
                self.children.clone(),
            ))
    }
}

pub(crate) fn button_bg_color(
    variant: ButtonVariant,
    is_disabled: bool,
    is_pressed: bool,
    is_hovering: bool,
) -> Srgba {
    let base_color = match variant {
        ButtonVariant::Default => colors::U3,
        ButtonVariant::Primary => colors::PRIMARY,
        ButtonVariant::Danger => colors::DESTRUCTIVE,
        ButtonVariant::Selected => colors::U4,
    };
    match (is_disabled, is_pressed, is_hovering) {
        (true, _, _) => base_color.with_alpha(0.2),
        (_, true, _) => base_color.lighter(0.05),
        (_, false, true) => base_color.lighter(0.02),
        (_, false, false) => base_color,
    }
}
