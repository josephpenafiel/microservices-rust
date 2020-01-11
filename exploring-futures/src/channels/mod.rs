mod mp_sc;
mod one_shot;
mod multiple;
mod executor;
pub use mp_sc::multiple;
pub use one_shot::single;
pub use multiple::alt_udp_echo;
pub use executor::send_spawn;
use futures::sync::{mpsc, oneshot};
use futures::{future, Future, IntoFuture, Sink, Stream, stream};

fn to_box<T>(fut: T) -> Box<dyn Future<Item=(), Error=()> + Send>
where 
T: IntoFuture,
T::Future: Send + 'static,
T::Item: 'static,
T::Error: 'static,
{
    let fut = fut.into_future().map(drop).map_err(drop);
    Box::new(fut)
} 