extern crate alloc;
use holdable::Holdable;
use std::any::Any;

struct OwnHold {
	pool: Vec<Box<dyn Holdable>>,
	keys: Vec<Key>,
}

#[derive(Debug)]
struct Key(usize);

impl OwnHold {
	const fn new() -> Self {
		OwnHold {
			pool: Vec::new(),
			keys: Vec::new(),
		}
	}

	fn hold<T>(&mut self, obj: T) -> Key
	where
		T: Holdable + 'static,
	{
		let k = if self.keys.len() == 0 {
			Key(self.pool.len())
		} else {
			self.keys.remove(0)
		};
		self.pool.insert(k.0, Box::new(obj));
		k
	}

	fn get<T>(&self, key: &Key) -> &T
	where
		T: 'static,
	{
		match self.pool[key.0].as_any().downcast_ref::<T>() {
			Some(v) => v,
			None => panic!("object can not cast type: {}", std::any::type_name::<T>()),
		}
	}

	fn get_mut<T>(&mut self, key: &Key) -> &mut T
	where
		T: 'static,
	{
		match self.pool[key.0].as_any_mut().downcast_mut::<T>() {
			Some(v) => v,
			None => panic!("object can not cast type: {}", std::any::type_name::<T>()),
		}
	}

	fn remove<T>(&mut self, key: &Key) -> T
	where
		T: Default + 'static,
	{
		match self.pool[key.0].as_any_mut().downcast_mut::<T>() {
			Some(v) => std::mem::take(v),
			None => panic!("object can not cast type: {}", std::any::type_name::<T>()),
		}
	}
}

static mut OWN_HOLD: OwnHold = OwnHold::new();

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Holdable, Debug, Default)]
	struct TestStruct {
		a: i32,
	}

	#[test]
	fn test() {
		let t = TestStruct { a: 64 };
		let k = unsafe { OWN_HOLD.hold(t) };

		println!("{:?}", k);
		println!("{:?}\n", unsafe { OWN_HOLD.get::<TestStruct>(&k) });
	}

	fn hold_obj() -> Key {
		let t = TestStruct { a: 64 };
		unsafe { OWN_HOLD.hold(t) }
	}

	fn print_obj(k: Key) {
		println!("{:?}\n", unsafe { OWN_HOLD.get::<TestStruct>(&k) });
	}

	#[test]
	fn test2() {
		let k = hold_obj();
		print_obj(k);
	}

	#[test]
	fn test_get_mut() {
		let t = TestStruct { a: 64 };
		let k = unsafe { OWN_HOLD.hold(t) };
		let t = unsafe { OWN_HOLD.get_mut::<TestStruct>(&k) };
		t.a = 42;
		print_obj(k);
	}
}
