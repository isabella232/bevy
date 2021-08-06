use bevy_ecs::{
    prelude::{Commands, IntoSystem, World},
    schedule::{Schedule, Stage, SystemStage},
};
// use bevy_ecs_macros::SystemParam;
//
// #[derive(SystemParam)]
// struct MySysParam<'a> {
//     commands: Commands<'a>,
// }

#[test]
fn commands_as_system_param_field() {
    // fn my_system(mut p: MySysParam) {
    fn my_system(mut commands: Commands) {
        commands.spawn().insert(42usize);
    }

    let mut world = World::default();
    run_system(&mut world, my_system);
    assert_eq!(1, world.query::<&usize>().iter(&world).count());
}

fn run_system<Param, S: IntoSystem<(), (), Param>>(world: &mut World, system: S) {
    let mut schedule = Schedule::default();
    let mut update = SystemStage::parallel();
    update.add_system(system);
    schedule.add_stage("update", update);
    schedule.run(world);
}
