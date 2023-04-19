use std::any::{Any};

pub trait Component {}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ComponentArray: AsAny {}

impl<T: Component + 'static> ComponentArray for Vec<T> {}

impl<T: ComponentArray + 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct ComponentStorage {
    pub component_vectors: Vec<Box<dyn ComponentArray>>
}

impl ComponentStorage {
    pub fn get_component_vec<T: Component + 'static>(&self) -> Option<&Vec<T>> {
        for component_vec in self.component_vectors.iter() {
            let component_vec_ref = component_vec.as_ref();
            if let Some(cv) = component_vec_ref.as_any().downcast_ref::<Vec<T>>() {
                return Some(cv);
            }
        }
        None
    }

    pub fn get_component_vec_mut<T: Component + 'static>(&mut self) -> Option<&mut Vec<T>> {
        for component_vec in self.component_vectors.iter_mut() {
            let component_vec_ref = component_vec.as_mut();
            if let Some(cv) = component_vec_ref.as_any_mut().downcast_mut::<Vec<T>>() {
                return Some(cv);
            }
        }
        None
    }

    pub fn add_component_vec<T: Component + 'static>(&mut self, component_vec: Vec<T>) {
        self.component_vectors.push(Box::new(component_vec));
    }

    pub fn add_component<T: Component + 'static>(&mut self, component: T) -> u32 {
        let mut id: u32 = 0;
        if let Some(comp_vec) = self.get_component_vec_mut::<T>() {
            id = comp_vec.len() as u32;
            comp_vec.push(component);
        } else {
            let component_vec: Vec<T> = vec![component];
            self.add_component_vec(component_vec);
        }
        id
    }
}

#[derive(Clone)]
pub struct Entity(pub u32);
