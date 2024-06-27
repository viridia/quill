use std::ops::Range;

use bevy::reflect::Reflect;

/// An attribute that specifies the minimum and maximum allowed values for a field.
/// This range is inclusive.
///
/// This attribute can be applied to numeric fields. It can also be applied to aggregate types
/// that have a numeric type parameter, such as an `Option<f32>` or `Vec<i8>`.
#[derive(Debug, Clone, Reflect)]
pub struct ValueRange<T>(pub Range<T>);

/// An attribute that specifies how many decimal digits of precision should be allowed.
/// If the field is an integer, this will be ignored. If present, field values will be
/// rounded to the nearest value with the specified number of decimal digits.
///
/// This attribute can be applied to numeric fields. It can also be applied to aggregate types
/// that have a numeric type parameter, such as an `Option<f32>` or `Vec<i8>`.
#[derive(Debug, Clone, Reflect)]
pub struct Precision(pub usize);

/// An attribute that specifies the increment and decrement step size for a numeric field.
/// If not present, the step size will be determined from the precision. If the precision is
/// not present, a heuristic will be used based on the range.
///
/// This attribute can be applied to numeric fields. It can also be applied to aggregate types
/// that have a numeric type parameter, such as an `Option<f32>` or `Vec<i8>`.
#[derive(Debug, Clone, Reflect)]
pub struct Step<T>(pub T);

/// An attribute that specifies that a text field should be displayed as a multiline text field.
/// This also means that newlines can be inserted into the text.
#[derive(Debug, Clone, Reflect)]
pub struct Multiline;
