use bevy::prelude::*;
use bevy_mod_picking::{focus::HoverMap, picking_core::Pickable, pointer::PointerId};
use bevy_mod_stylebuilder::{MaybeHandleOrPath, StyleBuilder};

/// A component which can be added to an entity to specify the cursor that should be used when
/// the mouse is over the entity. Relies on bevy_mod_picking to determine which entity is being
/// hovered.
#[derive(Component, Clone)]
pub enum Cursor {
    /// Don't show a cursor. Often used when we want to display a 3d cursor instead.
    Hidden,

    /// Show one of the standard winit cursors
    Icon(CursorIcon),

    /// Show a custom cursor image.
    Image(Handle<Image>, Vec2),
}

/// Resource which tracks the UI element used to make a custom cursor.
#[derive(Component)]
pub(crate) struct CustomCursor;

#[allow(missing_docs)]
pub trait StyleBuilderCursor {
    fn cursor(&mut self, icon: CursorIcon) -> &mut Self;
    fn cursor_image<'p>(
        &mut self,
        path: impl Into<MaybeHandleOrPath<'p, Image>>,
        origin: Vec2,
    ) -> &mut Self;
    fn cursor_hidden(&mut self) -> &mut Self;
}

impl<'a, 'w> StyleBuilderCursor for StyleBuilder<'a, 'w> {
    fn cursor(&mut self, icon: CursorIcon) -> &mut Self {
        match self.target.get_mut::<Cursor>() {
            Some(mut cursor) => {
                *cursor = Cursor::Icon(icon);
            }
            None => {
                self.target.insert(Cursor::Icon(icon));
            }
        };
        self
    }

    fn cursor_image<'p>(
        &mut self,
        path: impl Into<MaybeHandleOrPath<'p, Image>>,
        origin: Vec2,
    ) -> &mut Self {
        let image = match path.into() {
            MaybeHandleOrPath::Handle(h) => Some(h),
            MaybeHandleOrPath::Path(p) => Some(self.load_asset::<Image>(p)),
            MaybeHandleOrPath::None => None,
        };
        match (image, self.target.get_mut::<Cursor>()) {
            (Some(image), Some(mut cursor)) => {
                *cursor = Cursor::Image(image, origin);
            }
            (Some(image), None) => {
                self.target.insert(Cursor::Image(image, origin));
            }
            (None, Some(_)) => {
                self.target.remove::<Cursor>();
            }
            _ => (),
        };
        self
    }

    fn cursor_hidden(&mut self) -> &mut Self {
        match self.target.get_mut::<Cursor>() {
            Some(mut cursor) => {
                *cursor = Cursor::Hidden;
            }
            None => {
                self.target.insert(Cursor::Hidden);
            }
        };
        self
    }
}

pub(crate) fn update_cursor(
    mut commands: Commands,
    hover_map: Option<Res<HoverMap>>,
    parent_query: Query<&Parent>,
    cursor_query: Query<&Cursor>,
    mut custom_cursor_query: Query<(Entity, &mut CustomCursor, &mut UiImage, &mut Style)>,
    mut windows: Query<&mut Window>,
) {
    let cursor = hover_map.and_then(|hover_map| match hover_map.get(&PointerId::Mouse) {
        Some(hover_set) => hover_set.keys().find_map(|entity| {
            cursor_query.get(*entity).ok().or_else(|| {
                parent_query
                    .iter_ancestors(*entity)
                    .find_map(|e| cursor_query.get(e).ok())
            })
        }),
        None => None,
    });

    let mut show_custom = false;
    match cursor {
        Some(Cursor::Hidden) => {
            windows.iter_mut().for_each(|mut window| {
                window.cursor.visible = false;
                window.cursor.icon = CursorIcon::Default;
            });
        }
        Some(Cursor::Icon(icon)) => {
            windows.iter_mut().for_each(|mut window| {
                window.cursor.visible = true;
                window.cursor.icon = *icon;
            });
        }
        Some(Cursor::Image(image, origin)) => {
            show_custom = true;
            // Hide the winit cursor.
            windows.iter_mut().for_each(|mut window| {
                window.cursor.visible = false;
            });
            // TODO: Need to figure out which window the cursor is within and only show it on that window.
            let cursor_pos = windows
                .iter()
                .find_map(|w| w.cursor_position())
                .unwrap_or(Vec2::default())
                - *origin;
            if custom_cursor_query.is_empty() {
                commands.spawn((
                    ImageBundle {
                        image: UiImage::new(image.clone()),
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(cursor_pos.x),
                            top: Val::Px(cursor_pos.y),
                            ..default()
                        },
                        z_index: ZIndex::Global(1000),
                        ..default()
                    },
                    Pickable {
                        should_block_lower: false,
                        is_hoverable: false,
                    },
                    CustomCursor,
                ));
            } else {
                for (_, _, mut img, mut style) in custom_cursor_query.iter_mut() {
                    if img.texture != *image {
                        img.texture = image.clone();
                    }
                    if style.left != Val::Px(cursor_pos.x) {
                        style.left = Val::Px(cursor_pos.x);
                    }
                    if style.top != Val::Px(cursor_pos.y) {
                        style.top = Val::Px(cursor_pos.y);
                    }
                }
            }
        }
        None => {
            windows.iter_mut().for_each(|mut window| {
                window.cursor.visible = true;
                window.cursor.icon = CursorIcon::Default;
            });
        }
    }

    if !show_custom {
        custom_cursor_query.iter().for_each(|(entity, _, _, _)| {
            commands.entity(entity).despawn_recursive();
        });
    }
}
