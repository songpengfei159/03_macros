use std::{pin::Pin, task::{Context, Poll}};
use std::future::Future;

use futures::future;
use macros::my_ready;

fn main() {
    let mut cx = Context::from_waker(futures::task::noop_waker_ref());
    let ret = poll_fut(&mut cx);
    println!("aa:{:?}",ret)
}

fn poll_fut(cx: &mut Context<'_>) -> Poll<usize>{
    let mut fut = future::ready(42);
    let fut = Pin::new(&mut fut);
    my_ready!(fut.poll(cx))
}