use std::future::Future;
use crate::Gen;
use std::task::{Context, Poll};
use std::pin::Pin;

impl Future for Gen<'_, (), ()> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Gen::resume(&mut self, ()) {
            None => Poll::Ready(()),
            Some(x) => Poll::Ready(x),
        }
    }
}