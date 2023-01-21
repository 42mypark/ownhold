pub use holdable_derive::Holdable;
use std::any::Any;

pub trait Holdable {
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
}
