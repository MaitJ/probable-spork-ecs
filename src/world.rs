use crate::component::ComponentStorage;

pub struct GameWorld {
    pub component_storage: ComponentStorage
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            component_storage: ComponentStorage::new()
        }
    }
}
