mod vec3;
mod gjk;
mod epa;
mod phys;
mod shape;
mod collision;
mod transform;
use crate::vec3::{Vec3, cross, dot};
use shape::Shape;
use shape::SphereStruct;
use std::env;


use phys::PhysState;
use phys::PhysObj;
use std::time::Duration;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut state = PhysState::new();
    //let obj_key1 = state.add_obj(Shape::Sphere( SphereStruct { center: Vec3 {x:40.0,y:50.0,z:40.0}, radius: 10.0} ), 10.0);
    //let obj_key2 = state.add_obj(Shape::Sphere( SphereStruct { center: Vec3 {x:50.0,y:40.0,z:50.0}, radius: 10.0} ), 10.0);
    /*
    let shape1 = Shape::Sphere( SphereStruct { center: Vec3 {x:0.0,y:40.0,z:50.0}, radius: 10.0} );
    let shape2 = Shape::Sphere( SphereStruct { center: Vec3 {x:0.0,y:50.0,z:50.0}, radius: 10.0} );
    println!("{}", gjk::gjk(&shape1, &shape2).0);
    */

    /*
    loop {
//    state.apply_obj_force(obj_key, Vec3 {x:100.0,y:0.0,z:0.0});
    state.update(1.0);
    let obj1 = state.get_obj(obj_key1);
    let obj2 = state.get_obj(obj_key2);
    println!("Object 1: {}", &obj1.shape().pos());
    println!("");
    println!("Object 2: {}", &obj2.shape().pos());
    println!("");
    std::thread::sleep(Duration::new(2, 0));
    }
    */


}

