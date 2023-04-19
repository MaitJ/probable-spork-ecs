pub mod component;
pub mod world;


//fn main() {
//    let mut world = World::new();
//
//    let entity = world.create_entity();
//
//    let mut script: Box<dyn Script> = Box::new(TestScript {
//        entity: entity.clone(),
//        transform: Transform { x: 0.0, y: 0.0, z: 0.0 },
//        mesh: Mesh { label: "test_script mesh" }
//    });
//
//    script.setup();
//    script.pre_setup(entity.clone(), &mut world);
//
//    for _ in 0..90 {
//        script.pre_user_update(&world);
//        script.update();
//        script.post_user_update(&mut world);
//
//        match world.get_entity_component::<Transform>(&entity) {
//            Some(t) => println!("Transform: {:?}", t),
//            None => println!("Couldn't find transform")
//        }
//    }
//
//
//}
