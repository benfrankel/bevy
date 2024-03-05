use bevy_asset::Handle;
use bevy_color::Color;
use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_math::{Rect, Vec2};
use bevy_reflect::{std_traits::ReflectDefault, Reflect};
use bevy_render::texture::Image;

use crate::TextureSlicer;

/// Specifies the rendering properties of a sprite.
///
/// This is commonly used as a component within [`SpriteBundle`](crate::bundle::SpriteBundle).
#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component, Default)]
#[repr(C)]
pub struct Sprite {
    /// Handle to the texture
    pub texture: Handle<Image>,
    /// The sprite's color tint
    pub color: Color,
    /// Flip the sprite along the `X` axis
    pub flip_x: bool,
    /// Flip the sprite along the `Y` axis
    pub flip_y: bool,
    /// An optional custom size for the sprite that will be used when rendering, instead of the size
    /// of the sprite's image
    pub custom_size: Option<Vec2>,
    /// An optional rectangle representing the region of the sprite's image to render, instead of rendering
    /// the full image. This is an easy one-off alternative to using a [`TextureAtlas`](crate::TextureAtlas).
    ///
    /// When used with a [`TextureAtlas`](crate::TextureAtlas), the rect
    /// is offset by the atlas's minimal (top-left) corner position.
    pub rect: Option<Rect>,
    /// [`Anchor`] point of the sprite in the world
    pub anchor: Anchor,
}

impl Sprite {
    pub fn new(texture: Handle<Image>) -> Self {
        Self {
            texture,
            ..Default::default()
        }
    }

    /// Set the color tint
    #[must_use]
    pub const fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Flip the image along its x-axis
    #[must_use]
    pub const fn with_flip_x(mut self) -> Self {
        self.flip_x = true;
        self
    }

    /// Flip the image along its y-axis
    #[must_use]
    pub const fn with_flip_y(mut self) -> Self {
        self.flip_y = true;
        self
    }

    /// Set a custom size
    #[must_use]
    pub const fn with_custom_size(mut self, size: Vec2) -> Self {
        self.custom_size = Some(size);
        self
    }

    /// Set a sub-region of the image to render
    #[must_use]
    pub const fn with_rect(mut self, rect: Rect) -> Self {
        self.rect = Some(rect);
        self
    }

    /// Set an [`Anchor`] point
    #[must_use]
    pub const fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }
}

impl From<Handle<Image>> for Sprite {
    fn from(texture: Handle<Image>) -> Self {
        Self::new(texture)
    }
}

/// Controls how the image is altered when scaled.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub enum ImageScaleMode {
    /// The texture will be cut in 9 slices, keeping the texture in proportions on resize
    Sliced(TextureSlicer),
    /// The texture will be repeated if stretched beyond `stretched_value`
    Tiled {
        /// Should the image repeat horizontally
        tile_x: bool,
        /// Should the image repeat vertically
        tile_y: bool,
        /// The texture will repeat when the ratio between the *drawing dimensions* of texture and the
        /// *original texture size* are above this value.
        stretch_value: f32,
    },
}

/// How a sprite is positioned relative to its [`Transform`](bevy_transform::components::Transform).
/// It defaults to `Anchor::Center`.
#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[doc(alias = "pivot")]
pub enum Anchor {
    #[default]
    Center,
    BottomLeft,
    BottomCenter,
    BottomRight,
    CenterLeft,
    CenterRight,
    TopLeft,
    TopCenter,
    TopRight,
    /// Custom anchor point. Top left is `(-0.5, 0.5)`, center is `(0.0, 0.0)`. The value will
    /// be scaled with the sprite size.
    Custom(Vec2),
}

impl Anchor {
    pub fn as_vec(&self) -> Vec2 {
        match self {
            Anchor::Center => Vec2::ZERO,
            Anchor::BottomLeft => Vec2::new(-0.5, -0.5),
            Anchor::BottomCenter => Vec2::new(0.0, -0.5),
            Anchor::BottomRight => Vec2::new(0.5, -0.5),
            Anchor::CenterLeft => Vec2::new(-0.5, 0.0),
            Anchor::CenterRight => Vec2::new(0.5, 0.0),
            Anchor::TopLeft => Vec2::new(-0.5, 0.5),
            Anchor::TopCenter => Vec2::new(0.0, 0.5),
            Anchor::TopRight => Vec2::new(0.5, 0.5),
            Anchor::Custom(point) => *point,
        }
    }
}
