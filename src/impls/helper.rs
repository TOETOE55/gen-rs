use crate::Gen;
use std::pin::Pin;

pub struct Resume<'a, 'b, Send, Recv> {
    gen: Pin<&'a mut Gen<'b, Send, Recv>>,
}

impl<'a, 'b, Send, Recv> Resume<'a, 'b, Send, Recv> {
    pub fn new(gen: Pin<&'a mut Gen<'b, Send, Recv>>) -> Self {
        Self { gen }
    }

    pub fn resume(&mut self, send: Send) -> Option<Recv> {
        Gen::resume(&mut self.gen, send)
    }
}

pub fn generator<'a, Send, Recv, F>(f: F) -> Pin<Box<Gen<'a, Send, Recv>>>
where
    F: for<'g> FnOnce(Pin<&'g mut Gen<'a, Recv, Send>>, Send) + 'a
{
    Gen::new(f)
}