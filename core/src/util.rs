use std::future::Future;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Unstabilized [`Ready`](https://doc.rust-lang.org/nightly/std/future/struct.Ready.html) future.
pub struct Ready<T>(Option<T>);

impl<T> Unpin for Ready<T> {}

impl<T> Future for Ready<T> {
    type Output = T;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        Poll::Ready(self.0.take().expect("Ready polled after completion"))
    }
}

/// Unstabilized [`ready`](https://doc.rust-lang.org/nightly/std/future/fn.ready.html) function.
pub fn ready<T>(t: T) -> Ready<T> {
    Ready(Some(t))
}
