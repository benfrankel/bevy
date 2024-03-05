#![allow(deprecated)]

use crate::{Sprite, TextureAtlas};
use bevy_ecs::bundle::Bundle;
use bevy_render::view::{InheritedVisibility, ViewVisibility, Visibility};
use bevy_transform::components::{GlobalTransform, Transform};

/// A [`Bundle`] of components for drawing a single sprite from an image.
///
/// # Extra behaviours
///
/// You may add the following components to enable additional behaviours:
/// - [`ImageScaleMode`](crate::ImageScaleMode) to enable either slicing or tiling of the texture
/// - [`TextureAtlas`] to draw specific sections of the texture
///
/// Note that `ImageScaleMode` is currently not compatible with `TextureAtlas`.
#[derive(Bundle, Clone, Debug, Default)]
pub struct SpriteBundle {
    /// The image to be rendered along with properties such as color tint and flip.
    pub sprite: Sprite,
    /// The local transform of the sprite, relative to its parent.
    pub transform: Transform,
    /// The absolute transform of the sprite. This should generally not be written to directly.
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
}

impl<I> From<I> for SpriteBundle
where
    I: Into<Sprite>,
{
    fn from(value: I) -> Self {
        Self {
            sprite: value.into(),
            ..Default::default()
        }
    }
}

/// A [`Bundle`] of components for drawing a single sprite from a sprite sheet (also referred
/// to as a `TextureAtlas`) or for animated sprites.
///
/// Note:
/// This bundle is identical to [`SpriteBundle`] with an additional [`TextureAtlas`] component.
///
/// Check the following examples for usage:
/// - [`animated sprite sheet example`](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs)
/// - [`texture atlas example`](https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs)
#[deprecated(
    since = "0.14.0",
    note = "Use `TextureAtlas` alongside a `SpriteBundle` instead"
)]
#[derive(Bundle, Clone, Debug, Default)]
pub struct SpriteSheetBundle {
    /// The image to be rendered along with properties such as color tint and flip.
    pub sprite: Sprite,
    /// The local transform of the sprite, relative to its parent.
    pub transform: Transform,
    /// The absolute transform of the sprite. This should generally not be written to directly.
    pub global_transform: GlobalTransform,
    /// The sprite sheet texture atlas, allowing to draw a custom section of `texture`.
    pub atlas: TextureAtlas,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
}
