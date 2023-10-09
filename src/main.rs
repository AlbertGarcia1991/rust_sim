#![allow(unused_doc_comments)]
#![allow(dead_code)]

mod genome;
mod definitions;
mod world;


/// Project trying to replicate the amazing word done by https://github.com/davidrmille on his 
/// project biosim4, while I learn to code on Rust :)
fn main() {
    // Pollster is a lightweight library to enable Async
    pollster::block_on(world::run());
}
