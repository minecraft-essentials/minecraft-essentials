use std::future::Future;

pub trait Async<O>: Future<Output = O> {}
impl<T, O> Async<O> for T where T: Future<Output = O> {}
pub trait AsyncSend<O>: Send + Async<O> {}
impl<T, O> AsyncSend<O> for T where T: Async<O> + Send {}
pub trait AsyncSync<O>: Sync + Async<O> {}
impl<T, O> AsyncSync<O> for T where T: Async<O> + Sync {}
pub trait AsyncSendSync<O>: Send + Async<O> + Sync {}
impl<T, O> AsyncSendSync<O> for T where T: Async<O> + Send + Sync {}
