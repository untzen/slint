/* LICENSE BEGIN
    This file is part of the SixtyFPS Project -- https://sixtyfps.io
    Copyright (c) 2020 Olivier Goffart <olivier.goffart@sixtyfps.io>
    Copyright (c) 2020 Simon Hausmann <simon.hausmann@sixtyfps.io>

    SPDX-License-Identifier: GPL-3.0-only
    This file is also available under commercial licensing terms.
    Please contact info@sixtyfps.io for more information.
LICENSE END */
/*!
This module contains brush related types for the run-time library.
*/

use super::Color;
use crate::SharedVector;

/// A brush is a data structure that is used to describe how
/// a shape, such as a rectangle, path or even text, shall be filled.
/// A brush can also be applied to the outline of a shape, that means
/// the fill of the outline itself.
#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub enum Brush {
    /// The brush will not produce any fill.
    NoBrush,
    /// The color variant of brush is a plain color that is to be used for the fill.
    SolidColor(Color),
    /// The linear gradient variant of a brush describes the gradient stops for a fill
    /// where all color stops are along a line that's rotated by the specified angle.
    LinearGradient(LinearGradientBrush),
}

impl Default for Brush {
    fn default() -> Self {
        Self::NoBrush
    }
}

/// The LinearGradientBrush describes a way of filling a shape with different colors, which
/// are interpolated between different stops. The colors are aligned with a line that's rotated
/// by the LinearGradient's angle.
#[derive(Clone, PartialEq, Debug)]
#[repr(transparent)]
pub struct LinearGradientBrush(SharedVector<GradientStop>);

impl LinearGradientBrush {
    /// Creates a new linear gradient, described by the specified angle and the provided color stops.
    pub fn new(angle: f32, stops: impl IntoIterator<Item = GradientStop>) -> Self {
        let stop_iter = stops.into_iter();
        let mut encoded_angle_and_stops = SharedVector::with_capacity(stop_iter.size_hint().0 + 1);
        // The gradient's first stop is a fake stop to store the angle
        encoded_angle_and_stops.push(GradientStop { color: Default::default(), position: angle });
        encoded_angle_and_stops.extend(stop_iter);
        Self(encoded_angle_and_stops)
    }
    /// Returns the angle of the linear gradient in degrees.
    pub fn angle(&self) -> f32 {
        self.0[0].position
    }
    /// Returns the color stops of the linear gradient.
    pub fn stops<'a>(&'a self) -> impl Iterator<Item = &'a GradientStop> + 'a {
        // skip the first fake stop that just contains the angle
        self.0.iter().skip(1)
    }
}

/// GradientStop describes a single color stop in a gradient. The colors between multiple
/// stops are interpolated.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct GradientStop {
    /// The color to draw at this stop.
    pub color: Color,
    /// The position of this stop on the entire shape, as a normalized value between 0 and 1.
    pub position: f32,
}

#[cfg(feature = "femtovg_backend")]
impl From<&Brush> for femtovg::Paint {
    fn from(brush: &Brush) -> Self {
        match brush {
            Brush::NoBrush => Default::default(),
            Brush::SolidColor(color) => femtovg::Paint::color(color.into()),
            Brush::LinearGradient(_) => unimplemented!(),
        }
    }
}

#[cfg(feature = "femtovg_backend")]
impl From<Brush> for femtovg::Paint {
    fn from(brush: Brush) -> Self {
        match brush {
            Brush::NoBrush => Default::default(),
            Brush::SolidColor(color) => femtovg::Paint::color(color.into()),
            Brush::LinearGradient(_) => unimplemented!(),
        }
    }
}

#[test]
fn test_linear_gradient_encoding() {
    let stops: SharedVector<GradientStop> = [
        GradientStop { position: 0.0, color: Color::from_argb_u8(255, 255, 0, 0) },
        GradientStop { position: 0.5, color: Color::from_argb_u8(255, 0, 255, 0) },
        GradientStop { position: 1.0, color: Color::from_argb_u8(255, 0, 0, 255) },
    ]
    .into();
    let grad = LinearGradientBrush::new(256., stops.clone());
    assert_eq!(grad.angle(), 256.);
    assert!(grad.stops().eq(stops.iter()));
}
