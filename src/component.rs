use std::{
    any::{Any, TypeId},
    borrow::BorrowMut,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};
pub trait Component: PartialEq {
    fn setup(&mut self, world: &ComponentStorage);
    fn update(&mut self, world: &ComponentStorage);
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ComponentArray: AsAny {
    fn setup_components(&self, world: &ComponentStorage);
    fn update_components(&self, world: &ComponentStorage);
}

impl<T: Component + 'static> ComponentArray for Vec<RefCell<T>> {
    fn setup_components(&self, world: &ComponentStorage) {
        self.iter().for_each(|c| {
            let mut component = c.borrow_mut();
            component.setup(world);
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
    pub entities: u32,
    alive_entities: Vec<Entity>,
}

impl ComponentStorage {
    pub fn new() -> Self {
        Self {
            component_vectors: vec![],
            component_table: vec![],
            entities: 0,
            alive_entities: vec![],
        }
    }
    pub fn setup_components(&self) {
        self.component_vectors
            .iter()
            .for_each(|component_vec| component_vec.setup_components(self));
    }

    pub fn update_components(&self) {
        for component in self.component_vectors.iter() {
            component.update_components(self);
        }
    }

    pub fn get_component_vec<T: Component + 'static>(&self) -> Option<&Vec<RefCell<T>>> {
        self.component_vectors.iter().find_map(|component_vec| {
            let component_vec_ref = component_vec.as_ref();
            component_vec_ref.as_any().downcast_ref::<Vec<RefCell<T>>>()
        })
    }

    pub fn get_component_vec_mut<T: Component + 'static>(
        &mut self,
    ) -> Option<&mut Vec<RefCell<T>>> {
        self.component_vectors.iter_mut().find_map(|component_vec| {
            let component_vec_ref = component_vec.as_mut();
            component_vec_ref.as_any_mut().downcast_mut::<Vec<RefCell<T>>>()
        })
    }

    // Up to user to be careful with accessing entities that are "destroyed"
    pub fn remove_entity(&mut self, entity: Entity) {
        self.alive_entities.remove(entity.0 as usize);
    }

    pub fn get_entities(&self) -> Vec<Entity> {
        self.alive_entities.clone()
    }

    pub fn add_component_vec<T: Component + 'static>(&mut self, component_vec: Vec<RefCell<T>>) {
        self.component_vectors.push(Box::new(component_vec));
    }

    fn add_component<T: Component + 'static>(&mut self, component: T) -> u32 {
        let Some(comp_vec) = self.get_component_vec_mut::<T>() else {
            let component_vec: Vec<RefCell<T>> = vec![RefCell::new(component)];
            self.add_component_vec(component_vec);

            return 0;
        };

        comp_vec.push(RefCell::new(component));
        comp_vec.len() as u32
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity = Entity(self.entities);
        self.component_table.push(Some(HashMap::new()));
        self.entities += 1;
        self.alive_entities.push(entity.clone());
        entity
    }

    fn get_entity_component_table_mut(
        &mut self,
        entity: &Entity,
    ) -> Option<&mut HashMap<TypeId, u32>> {
        self.component_table.get_mut(entity.0 as usize)?.as_mut()
    }

    pub fn register_component<T: Component + 'static>(&mut self, entity: &Entity, component: T) {
        let component_id = self.add_component(component);
        self.get_entity_component_table_mut(entity)
            .and_then(|table| table.insert(TypeId::of::<T>(), component_id));
    }

    fn get_entity_component_id<T: Component + 'static>(&self, entity: &Entity) -> Option<u32> {
        let row = self.component_table.get(entity.0 as usize)?.as_ref();
        row.and_then(|component_table| {
            let type_id = TypeId::of::<T>();
            let component_id = component_table.get(&type_id)?;
            Some(*component_id)
        })
    }

    pub fn get_entity_component<T: Component + 'static>(&self, entity: &Entity) -> Option<Ref<T>> {
        self.get_component_vec::<T>().and_then(|component_vec| {
            let component_id = self.get_entity_component_id::<T>(entity)?;
            let component = component_vec.get(component_id as usize)?;

            Some(component.borrow())
        })
    }

    pub fn get_entity_component_mut<T: Component + 'static>(
        &self,
        entity: &Entity,
    ) -> Option<RefMut<T>> {
        self.get_component_vec::<T>().and_then(|component_vec| {
            let component_id = self.get_entity_component_id::<T>(entity)?;
            let component = component_vec.get(component_id as usize)?;

            Some(component.borrow_mut())
        })
    }
}

#[derive(Clone, Default)]
pub struct Entity(pub u32);
