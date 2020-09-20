mod generic;
mod custom;

pub use generic::*;
pub use custom::*;

use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Display, Debug};

pub trait Primitive:             Default + Display + Debug + PartialEq + Copy + PartialOrd + Mul<Self, Output=Self> + Add<Self, Output=Self> + Div<Self, Output=Self> + Sub<Self, Output=Self> + num_traits::cast::NumCast {}
impl<T> Primitive for T where T: Default + Display + Debug + PartialEq + Copy + PartialOrd + Mul<T   , Output=T   > + Add<T   , Output=T   > + Div<T   , Output=T   > + Sub<T   , Output=T   > + num_traits::cast::NumCast {}

pub struct Vec2t<T: Primitive, P> {
	pub x: T,
	pub y: T,
	_p: PhantomData<P>,
}
