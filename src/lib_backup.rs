use std::{any::{Any, TypeId}, collections::HashMap, error::Error, fmt::Display, time::Instant};
use script_gen_macro::ScriptComponentUpdater;

trait Component {}

trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Default, Debug, Clone)]
struct Transform {
    x: f32,
    y: f32,
    z: f32
}

#[derive(Debug, Clone)]
struct Mesh {
    label: &'static str
}

impl Component for Mesh {}
impl Component for Transform {}

trait ComponentArray: AsAny {}

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
    component_vectors: Vec<Box<dyn ComponentArray>>
}

impl ComponentStorage {
    fn get_component_vec<T: Component + 'static>(&self) -> Option<&Vec<T>> {
        for component_vec in self.component_vectors.iter() {
            let component_vec_ref = component_vec.as_ref();
            if let Some(cv) = component_vec_ref.as_any().downcast_ref::<Vec<T>>() {
                return Some(cv);
            }
        }
        None
    }

    fn get_component_vec_mut<T: Component + 'static>(&mut self) -> Option<&mut Vec<T>> {
        for component_vec in self.component_vectors.iter_mut() {
            let component_vec_ref = component_vec.as_mut();
            if let Some(cv) = component_vec_ref.as_any_mut().downcast_mut::<Vec<T>>() {
                return Some(cv);
            }
        }
        None
    }

    fn add_component_vec<T: Component + 'static>(&mut self, component_vec: Vec<T>) {
        self.component_vectors.push(Box::new(component_vec));
    }

    fn add_component<T: Component + 'static>(&mut self, component: T) -> u32 {
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
pub struct Entity(u32);

pub struct World {
    pub component_storage: ComponentStorage,
    component_table: Vec<Option<HashMap<TypeId, u32>>>,
    entities: u32,
}

impl World {
    fn new() -> Self {
        Self {
            component_storage: ComponentStorage { component_vectors: vec![] },
            entities: 0,
            component_table: vec![]
        }
    }

    fn create_entity(&mut self) -> Entity {
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

    fn register_component<T: Component + 'static>(&mut self, entity: &Entity, component: T) {
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

    fn get_entity_component<T: Component + 'static>(&self, entity: &Entity) -> Option<&T> {
        if let Some(component_id) = self.get_entity_component_id::<T>(entity) {
            if let Some(component_vec) = self.component_storage.get_component_vec::<T>() {
                if let Some(component) = component_vec.get(component_id as usize) {
                    return Some(component);
                }
            }
        }
        None
    }

    fn get_entity_component_mut<T: Component + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
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

pub trait ScriptComponentUpdater {
    fn pre_setup(&mut self, entity: Entity, world: &mut World);
    fn pre_user_update(&mut self, world: &World);
    fn post_user_update(&mut self, world: &mut World);
}

trait Script: ScriptComponentUpdater {
    fn setup(&mut self);
    fn update(&mut self);
}

#[derive(ScriptComponentUpdater)]
struct TestScript {
    entity: Entity,
    #[SyncComponent]
    transform: Transform,
    #[SyncComponent]
    mesh: Mesh
}

impl Script for TestScript {
    fn setup(&mut self) {
        self.transform.y = 90.0;
    }
    fn update(&mut self) {
        self.transform.x += 1.0;
    }
}

fn main() {
    let mut world = World::new();

    let entity = world.create_entity();

    let mut script: Box<dyn Script> = Box::new(TestScript {
        entity: entity.clone(),
        transform: Transform { x: 0.0, y: 0.0, z: 0.0 },
        mesh: Mesh { label: "test_script mesh" }
    });

    script.setup();
    script.pre_setup(entity.clone(), &mut world);

    for _ in 0..90 {
        script.pre_user_update(&world);
        script.update();
        script.post_user_update(&mut world);

        match world.get_entity_component::<Transform>(&entity) {
            Some(t) => println!("Transform: {:?}", t),
            None => println!("Couldn't find transform")
        }
    }


}
