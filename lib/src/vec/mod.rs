mod generic;
mod custom;

pub use generic::*;
pub use custom::*;

use std::marker::PhantomData;

pub struct DefaultParam;

pub struct Vec2t<T, P = DefaultParam> {
	pub x: T,
	pub y: T,
	_p: PhantomData<P>,
}
