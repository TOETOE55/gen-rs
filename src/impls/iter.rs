use crate::Gen;
use std::pin::Pin;
use crate::impls::helper::Resume;

impl<A> Iterator for Pin<Box<Gen<'_, (), A>>> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        Gen::resume(&mut self.as_mut(), ())
    }
}

impl<A> Iterator for Resume<'_, '_, (), A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        self.resume(())
    }
}
