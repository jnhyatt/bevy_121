//! Demonstrates a simple example where each contributor can only work on one project at a time, and
//! each project can only be worked on by one contributor at a time.

use bevy_121::AsymmetricOneToOne;
use bevy_ecs::prelude::*;

fn main() {
    let mut world = World::new();
    let alice = world.spawn(()).id();
    let cart = world.spawn(()).id();
    let bsn = world.spawn(Assignee(alice)).id();
    // Alice is in over her head, have Cart take over
    world.entity_mut(cart).insert(Assignment(bsn));
    assert_eq!(world.entity(bsn).get::<Assignee>().unwrap().0, cart);
}

#[derive(AsymmetricOneToOne)]
#[target(Assignee)]
struct Assignment(Entity);

#[derive(AsymmetricOneToOne)]
#[target(Assignment)]
struct Assignee(Entity);
