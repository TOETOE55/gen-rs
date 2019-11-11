use std::ops::{Generator, GeneratorState};
use crate::Gen;
use std::pin::Pin;

impl<A> Generator for Gen<'_, (), A> {
    type Yield = A;
    type Return = ();

    fn resume(mut self: Pin<&mut Self>) -> GeneratorState<Self::Yield, Self::Return> {
        match Gen::resume(&mut self, ()) {
            None => GeneratorState::Complete(()),
            Some(x) => GeneratorState::Yielded(x),
        }
    }
}