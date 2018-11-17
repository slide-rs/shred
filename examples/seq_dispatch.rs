extern crate shred;
#[macro_use]
extern crate shred_derive;

use shred::{DispatcherBuilder, Read, Resources, System, Write};

#[derive(Debug, Default)]
struct ResA;

#[derive(Debug, Default)]
struct ResB;

#[derive(SystemData)]
struct Data {
    a: Read<ResA>,
    b: Write<ResB>,
}

struct EmptySystem;

impl System for EmptySystem {
    type SystemData = Data;

    fn run(&mut self, bundle: Data) {
        println!("{:?}", &*bundle.a);
        println!("{:?}", &*bundle.b);
    }
}

fn main() {
    let mut resources = Resources::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(EmptySystem, "empty", &[])
        .build();
    dispatcher.setup(&mut resources);

    dispatcher.dispatch_seq(&resources);
}
