pub mod common;
pub mod hlist;

pub fn main() {
    println!("Hello world!");
}

use std::marker::PhantomData;

use hlist::indexed::lhlist;


pub struct Entry<const INDEX: usize, U>(PhantomData<U>);

#[derive(Default)]
pub struct Registry<US = (), INDS = ()> {
    uniforms: US,
    indices: INDS
}

impl<US> Registry<US, ()> {
    pub fn add<const INDEX: usize, U>(self, value: U) -> Registry<(US, U), ()> {
        let Self { uniforms, indices } = self;
        Registry {
            uniforms: (uniforms, value), 
            indices
        }
    }
}

impl<US: , INDS: lhlist::Base> Registry<US, INDS> {
    pub fn get<const INDEX: usize>(&self) {
        // self.uniforms.get()
    }
}


pub fn hlist() -> HList {
    (
        ((((), 1i32), 2u32), 3f32 ),
        (
            ((((), 4i16), 5u16), 6f64 ),
            (
                ((((), 7i8 ), 8u8 ), 9u128),
                ()
            )
        ),
    )  
}

pub struct Shader<US>(PhantomData<US>);

pub struct Builder<UUS, IUS>(UnInitUniforms<UUS>, InitUniforms<IUS>);

type HList = (
    ((((), i32), u32), f32),
    (
        ((((), i16), u16), f64),
        (
            ((((), i8), u8), u128),
            ()
        )
    ),
);

pub struct UnInitUniforms<US>(US);
pub struct InitUniforms<US>(US);

impl Builder<(), ()> {
    pub fn new() -> Self {
        Self(UnInitUniforms(()), InitUniforms(()))
    }
}
