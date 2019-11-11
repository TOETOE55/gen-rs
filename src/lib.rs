#![cfg_attr(nightly, feature(generators))]
#![cfg_attr(nightly, feature(generator_trait))]

mod gen;
mod impls;
pub use gen::Gen;
pub use impls::helper;
