use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq)]
pub enum BadgeStyle {
    Flat,
    FlatSquare,
    ForTheBadge,
    Plastic,
    Social,
    Unspecified,
}
