use std::{collections::HashMap, any::TypeId};

use crate::component::{ComponentStorage, Entity, Component};


pub struct World {
    pub component_storage: ComponentStorage,
    component_table: Vec<Option<HashMap<TypeId, u32>>>,
    entities: u32,
}

impl World {
    pub fn new() -> Self {

        Self {
            component_storage: ComponentStorage { component_vectors: vec![] },
            entities: 0,
            component_table: vec![]
        }
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
        let component_id = self.component_storage.add_component(component);
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

    pub fn get_entity_component<T: Component + 'static>(&self, entity: &Entity) -> Option<&T> {
        if let Some(component_id) = self.get_entity_component_id::<T>(entity) {
            if let Some(component_vec) = self.component_storage.get_component_vec::<T>() {
                if let Some(component) = component_vec.get(component_id as usize) {
                    return Some(component);
                }
            }
        }
        None
    }

    pub fn get_entity_component_mut<T: Component + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        if let Some(component_id) = self.get_entity_component_id::<T>(entity) {
            if let Some(component_vec) = self.component_storage.get_component_vec_mut::<T>() {
                if let Some(component) = component_vec.get_mut(component_id as usize) {
                    return Some(component);
                }
            }
        }
        None
    }
}
