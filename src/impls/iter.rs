use crate::Gen;
use std::pin::Pin;

impl<A> Iterator for Pin<Box<Gen<'_, (), A>>> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        Gen::resume(&mut self.as_mut(), ())
    }
}
