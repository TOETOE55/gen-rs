use crate::Gen;

impl<A> Iterator for Gen<'_, (), A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        self.resume(())
    }
}
