use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{ Mutex, Arc };
use service::Service;
use std::mem;

type Void = ();
pub struct Locator {
	map: Mutex<HashMap<Service, (TypeId, Arc<Void>)>>
}

impl Locator {
	
	#[inline(always)]
	pub fn new() -> Self {
		Locator { 
			map: Mutex::new(HashMap::new())
		}
	}
	
	#[inline(always)]
	pub fn get<T: Send + 'static>(&self, t: Service) -> Arc<T> {
		let map = self.map.lock().unwrap();
		
		let &(ref id, ref arc) = map.get(&t).unwrap();
		assert!(*id == TypeId::of::<T>());
		
		unsafe { mem::transmute(arc.clone()) }
	}
	
	#[inline(always)]
	pub fn set<T: Send + 'static>(&mut self, t: Service, v: T) -> Arc<T> {
		let mut map = self.map.lock().unwrap();
		let arc = Arc::new(v);
		
		map.insert(t, (TypeId::of::<T>(), unsafe { mem::transmute(arc.clone()) }));
		arc
	}
}