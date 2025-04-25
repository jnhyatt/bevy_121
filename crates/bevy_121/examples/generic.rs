//! This example is just `simple` modified slightly to demonstrate the use of generic types. This is
//! currently broken since putting generics on the structs requires `PhantomData` to be used, which
//! means our structs are no longer single-field tuple structs! Arrrrgh! Well, I'll let that
//! motivate me to fix it soon.

use std::marker::PhantomData;

use bevy_121::AsymmetricOneToOne;
use bevy_ecs::prelude::*;

fn main() {
    let mut world = World::new();
    let alice = world.spawn(()).id();
    let cart = world.spawn(()).id();
    let jasmine = world.spawn(()).id();
    let bsn = world.spawn(Assignee::<Ecs>(alice, PhantomData)).id();
    let meshlets = world
        .spawn(Assignee::<Rendering>(jasmine, PhantomData))
        .id();
    // Alice is in over her head, have Cart take over
    world
        .entity_mut(cart)
        .insert(Assignment::<Ecs>(bsn, PhantomData));
    assert_eq!(world.entity(bsn).get::<Assignee<Ecs>>().unwrap().0, cart);
    // You know what, give Cart meshlets too
    world
        .entity_mut(cart)
        .insert(Assignment::<Rendering>(meshlets, PhantomData));
    assert_eq!(
        world
            .entity(meshlets)
            .get::<Assignee<Rendering>>()
            .unwrap()
            .0,
        cart
    );
}

#[derive(AsymmetricOneToOne)]
#[target(Assignee<T>)]
struct Assignment<T>(Entity, PhantomData<T>);

#[derive(AsymmetricOneToOne)]
#[target(Assignment<T>)]
struct Assignee<T>(Entity, PhantomData<T>);

trait AssignmentType {}

struct Ecs;

impl AssignmentType for Ecs {}

struct Rendering;

impl AssignmentType for Rendering {}
