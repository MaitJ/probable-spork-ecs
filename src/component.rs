use std::{any::{Any, TypeId}, cell::{RefCell, Ref, RefMut}, borrow::BorrowMut, collections::HashMap};
pub trait Component {
    fn setup(&mut self);
    fn update(&mut self, world: &ComponentStorage);
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ComponentArray: AsAny {
    fn setup_components(&self);
    fn update_components(&self, world: &ComponentStorage);
}

impl<T: Component + 'static> ComponentArray for Vec<RefCell<T>> {
    fn setup_components(&self) {
        self.iter().for_each(|c| {
            let mut component = c.borrow_mut();
            component.setup();
        })
    }
    fn update_components(&self, world: &ComponentStorage) {
        self.iter().for_each(|c| {
            let mut component = c.borrow_mut();
            component.update(world);
        })
    }
}

impl<T: ComponentArray + 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct ComponentStorage {
    pub component_vectors: Vec<Box<dyn ComponentArray>>,
    component_table: Vec<Option<HashMap<TypeId, u32>>>,
    entities: u32
}

impl ComponentStorage {
    pub fn new() -> Self {
        Self {
            component_vectors: vec![],
            component_table: vec![],
            entities: 0
        }
    }
    pub fn setup_components(&self) {
        self.component_vectors
            .iter()
            .for_each(|component_vec| component_vec.setup_components());
    }

    pub fn update_components(&self) {
        for component in self.component_vectors.iter() {
            component.update_components(self);
        }
    }

    fn get_component_vec<T: Component + 'static>(&self) -> Option<&Vec<RefCell<T>>> {
        for component_vec in self.component_vectors.iter() {
            let component_vec_ref = component_vec.as_ref();
            if let Some(cv) = component_vec_ref.as_any().downcast_ref::<Vec<RefCell<T>>>() {
                return Some(cv);
            }
        }
        None
    }

    fn get_component_vec_mut<T: Component + 'static>(&mut self) -> Option<&mut Vec<RefCell<T>>> {
        for component_vec in self.component_vectors.iter_mut() {
            let component_vec_ref = component_vec.as_mut();
            if let Some(cv) = component_vec_ref.as_any_mut().downcast_mut::<Vec<RefCell<T>>>() {
                return Some(cv);
            }
        }
        None
    }

    pub fn add_component_vec<T: Component + 'static>(&mut self, component_vec: Vec<RefCell<T>>) {
        self.component_vectors.push(Box::new(component_vec));
    }

    fn add_component<T: Component + 'static>(&mut self, component: T) -> u32 {
        let mut id: u32 = 0;
        if let Some(comp_vec) = self.get_component_vec_mut::<T>() {
            id = comp_vec.len() as u32;
            comp_vec.push(RefCell::new(component));
        } else {
            let component_vec: Vec<RefCell<T>> = vec![RefCell::new(component)];
            self.add_component_vec(component_vec);
        }
        id
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity = Entity(self.entities);
        self.component_table.push(Some(HashMap::new()));
        self.entities += 1;
        entity
    }

    fn get_entity_component_table_mut(&mut self, entity: &Entity) -> Option<&mut HashMap<TypeId, u32>> {
        if let Some(row) = self.component_table.get_mut(entity.0 as usize) {
            if let Some(map) = row {
                return Some(map);
            }
        }
        None
    }

    pub fn register_component<T: Component + 'static>(&mut self, entity: &Entity, component: T) {
        let component_id = self.add_component(component);
        if let Some(table) = self.get_entity_component_table_mut(entity) {
            table.insert(TypeId::of::<T>(), component_id);
        }
    }

    fn get_entity_component_id<T: Component + 'static>(&self, entity: &Entity) -> Option<u32> {
        if let Some(row) = self.component_table.get(entity.0 as usize) {
            if let Some(component_table) = row {
                let type_id = TypeId::of::<T>();
                if let Some(component_id) = component_table.get(&type_id) {
                    return Some(*component_id);
                }
            }
        }
        None
    }

    pub fn get_entity_component<T: Component + 'static>(&self, entity: &Entity) -> Option<Ref<T>> {
        if let Some(component_id) = self.get_entity_component_id::<T>(entity) {
            if let Some(component_vec) = self.get_component_vec::<T>() {
                if let Some(component) = component_vec.get(component_id as usize) {
                    return Some(component.borrow());
                }
            }
        }
        None
    }

    pub fn get_entity_component_mut<T: Component + 'static>(&self, entity: &Entity) -> Option<RefMut<T>> {
        if let Some(component_id) = self.get_entity_component_id::<T>(entity) {
            if let Some(component_vec) = self.get_component_vec::<T>() {
                if let Some(component) = component_vec.get(component_id as usize) {
                    return Some(component.borrow_mut());
                }
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct Entity(pub u32);
