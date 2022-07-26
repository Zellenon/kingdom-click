use bevy::prelude::*;

// Defines the amount of time that should elapse between each physics step.
pub const TIME_STEP: f32 = 1.0 / 60.0;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Default)]
pub struct CollisionEvent;

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

// pub fn check_for_collisions(
//     mut commands: Commands,
//     mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
//     collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
//     mut collision_events: EventWriter<CollisionEvent>,
// ) {
//     let (mut ball_velocity, ball_transform) = ball_query.single_mut();
//     let ball_size = ball_transform.scale.truncate();

//     // check collision with walls
//     for (collider_entity, transform, maybe_brick) in collider_query.iter() {
//         let collision = collide(
//             ball_transform.translation,
//             ball_size,
//             transform.translation,
//             transform.scale.truncate(),
//         );
//         if let Some(collision) = collision {
//             // Sends a collision event so that other systems can react to the collision
//             collision_events.send_default();

//             // Bricks should be despawned and increment the scoreboard on collision
//             if maybe_brick.is_some() {
//                 scoreboard.score += 1;
//                 commands.entity(collider_entity).despawn();
//             }

//             // reflect the ball when it collides
//             let mut reflect_x = false;
//             let mut reflect_y = false;

//             // only reflect if the ball's velocity is going in the opposite direction of the
//             // collision
//             match collision {
//                 Collision::Left => reflect_x = ball_velocity.x > 0.0,
//                 Collision::Right => reflect_x = ball_velocity.x < 0.0,
//                 Collision::Top => reflect_y = ball_velocity.y < 0.0,
//                 Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
//                 Collision::Inside => { /* do nothing */ }
//             }

//             // reflect velocity on the x-axis if we hit something on the x-axis
//             if reflect_x {
//                 ball_velocity.x = -ball_velocity.x;
//             }

//             // reflect velocity on the y-axis if we hit something on the y-axis
//             if reflect_y {
//                 ball_velocity.y = -ball_velocity.y;
//             }
//         }
//     }
// }
