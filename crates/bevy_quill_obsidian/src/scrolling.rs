use bevy::{input::mouse::MouseWheel, prelude::*, render::view::visibility, ui};
use bevy_mod_picking::{focus::HoverMap, pointer::PointerId, prelude::EntityEvent};

/// Mouse wheel entity event
#[derive(Clone, Event, EntityEvent, Debug)]
#[can_bubble]
pub struct ScrollWheel {
    /// Event target
    #[target]
    pub target: Entity,

    /// Mouse wheel delta
    pub delta: Vec2,
}

/// Component that enables scrolling on an element
#[derive(Component, Default)]
pub struct ScrollArea {
    /// Scroll amount along X-axis
    pub scroll_left: f32,

    /// Scroll amount along Y-axis
    pub scroll_top: f32,

    /// Size of scrolling content
    pub content_size: Vec2,

    /// Size of visible scrolling area
    pub visible_size: Vec2,

    /// Entity id of the X scrollbar
    pub id_scrollbar_x: Option<Entity>,

    /// Entity id of the Y scrollbar
    pub id_scrollbar_y: Option<Entity>,
}

impl ScrollArea {
    /// Offset the current scroll position by the given values.
    pub fn scroll_by(&mut self, dx: f32, dy: f32) {
        // Apply max constraint first, then min - don't use clamp() here.
        self.scroll_left = (self.scroll_left + dx)
            .min(self.content_size.x - self.visible_size.x)
            .max(0.);
        self.scroll_top = (self.scroll_top + dy)
            .min(self.content_size.y - self.visible_size.y)
            .max(0.);
    }

    /// Scroll to the given scroll position (values clamped).
    pub fn scroll_to(&mut self, x: f32, y: f32) {
        // Apply max constraint first, then min - don't use clamp() here.
        self.scroll_left = x.min(self.content_size.x - self.visible_size.x).max(0.);
        self.scroll_top = y.min(self.content_size.y - self.visible_size.y).max(0.);
    }

    /// Current scroll position
    pub fn scroll_position(&self) -> Vec2 {
        Vec2::new(self.scroll_left, self.scroll_top)
    }

    /// Size of the content rect
    pub fn content_size(&self) -> Vec2 {
        self.content_size
    }

    /// Visible size
    pub fn visible_size(&self) -> Vec2 {
        self.visible_size
    }
}

/// Marker component indicating this entity is the scrolling content area.
#[derive(Component, Clone, Default)]
pub struct ScrollContent;

/// Marker component indicating this entity is the scrollbar on the X-axis.
#[derive(Component)]
pub struct ScrollBar {
    /// True means this scrollbar controls the Y-axis
    pub vertical: bool,

    /// Entity id of the scroll area.
    pub id_scroll_area: Entity,

    /// Minimum thumb size.
    pub min_thumb_size: f32,
}

/// Marker component indicating this entity is a scrollbar thumb.
#[derive(Component)]
pub struct ScrollBarThumb;

#[allow(clippy::type_complexity)]
pub(crate) fn update_scroll_positions(
    mut query: Query<(&Node, &mut ScrollArea, &GlobalTransform, &Children)>,
    mut query_content: Query<
        (&Node, &mut Style, &GlobalTransform),
        (With<ScrollContent>, Without<ScrollArea>),
    >,
    query_scrollbar: Query<(&ScrollBar, &Children)>,
    mut query_scrollbar_thumb: Query<
        (&mut Style, &mut Visibility),
        (With<ScrollBarThumb>, Without<ScrollContent>),
    >,
) {
    for (node, mut scrolling, gt, children) in query.iter_mut() {
        // Measure size and update scroll width and height
        let scroll_size = node.logical_rect(gt);
        scrolling.visible_size.x = scroll_size.width();
        scrolling.visible_size.y = scroll_size.height();

        // Measure size of content
        if let Some(child) = children
            .iter()
            .find(|chid| query_content.get(**chid).is_ok())
        {
            let (content, mut style, content_gt) = query_content.get_mut(*child).unwrap();
            let content_size = content.logical_rect(content_gt);
            scrolling.content_size.x = content_size.width();
            scrolling.content_size.y = content_size.height();

            scrolling.scroll_left = scrolling
                .scroll_left
                .min(scrolling.content_size.x - scrolling.visible_size.x)
                .max(0.);
            scrolling.scroll_top = scrolling
                .scroll_top
                .min(scrolling.content_size.y - scrolling.visible_size.y)
                .max(0.);

            style.left = ui::Val::Px(-scrolling.scroll_left);
            style.top = ui::Val::Px(-scrolling.scroll_top);
        } else {
            scrolling.content_size.x = 0.;
            scrolling.content_size.y = 0.;
        }

        // Adjust horizontal scrollbar
        if let Some(sid) = scrolling.id_scrollbar_x {
            if let Ok((scrollbar, children)) = query_scrollbar.get(sid) {
                if let Some(child_id) = children.first() {
                    if let Ok((mut style, mut visibility)) =
                        query_scrollbar_thumb.get_mut(*child_id)
                    {
                        // Thumb should be equal to proportion of scroll width / content width.
                        // Thumb should be no smaller than min size, and no bigger than full size.
                        let thumb_size = (scrolling.visible_size.x / scrolling.content_size.x)
                            .max(scrollbar.min_thumb_size / scrolling.visible_size.x)
                            .min(1.);
                        let range = scrolling.content_size.x - scrolling.visible_size.x;
                        let scroll_pos = if range > 0. {
                            scrolling.scroll_left * (1. - thumb_size) / range
                        } else {
                            0.
                        };
                        style.left = ui::Val::Percent(scroll_pos * 100.);
                        style.width = ui::Val::Percent(thumb_size * 100.);
                        *visibility = if thumb_size < 1. {
                            visibility::Visibility::Visible
                        } else {
                            visibility::Visibility::Hidden
                        }
                    }
                }
            }
        }

        // Adjust vertical scrollbar
        if let Some(sid) = scrolling.id_scrollbar_y {
            if let Ok((scrollbar, children)) = query_scrollbar.get(sid) {
                if let Some(child_id) = children.first() {
                    if let Ok((mut style, mut visibility)) =
                        query_scrollbar_thumb.get_mut(*child_id)
                    {
                        let thumb_size = (scrolling.visible_size.y / scrolling.content_size.y)
                            .max(scrollbar.min_thumb_size / scrolling.visible_size.y)
                            .min(1.);
                        let range = scrolling.content_size.y - scrolling.visible_size.y;
                        let scroll_pos = if range > 0. {
                            scrolling.scroll_top * (1. - thumb_size) / range
                        } else {
                            0.
                        };
                        style.top = ui::Val::Percent(scroll_pos * 100.);
                        style.height = ui::Val::Percent(thumb_size * 100.);
                        *visibility = if thumb_size < 1. {
                            visibility::Visibility::Visible
                        } else {
                            visibility::Visibility::Hidden
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn handle_scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut writer: EventWriter<ScrollWheel>,
    hover_map: Res<HoverMap>,
) {
    if let Some(hover) = hover_map.get(&PointerId::Mouse) {
        use bevy::input::mouse::MouseScrollUnit;
        for ev in scroll_evr.read() {
            match ev.unit {
                MouseScrollUnit::Line => {
                    // Ignore for now.
                }
                MouseScrollUnit::Pixel => {
                    for k in hover.keys() {
                        writer.send(ScrollWheel {
                            target: *k,
                            delta: Vec2 { x: ev.x, y: ev.y },
                        });
                    }
                }
            }
        }
    }
}
