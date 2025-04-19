use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let world_handle: Handle<TiledMap> = asset_server.load("seconmap.tmx");
    commands.spawn((TiledMapHandle(world_handle), TiledMapAnchor::Center));
} // Define a custom physics backend which will use the TiledPhysicsAvianBackend
// but add an extra RigidBody::Static component on top of the regular collider.
#[derive(Default, Debug, Clone, Reflect)]
#[reflect(Default, Debug)]
pub struct MyCustomAvianPhysicsBackend(TiledPhysicsAvianBackend);

impl TiledPhysicsBackend for MyCustomAvianPhysicsBackend {
    fn spawn_colliders(
        &self,
        commands: &mut Commands,
        tiled_map: &TiledMap,
        filter: &TiledNameFilter,
        collider: &TiledCollider,
    ) -> Vec<TiledColliderSpawnInfos> {
        let colliders = self
            .0
            .spawn_colliders(commands, tiled_map, filter, collider);
        for c in &colliders {
            commands.entity(c.entity).insert(RigidBody::Static);
        }
        colliders
    }
}
