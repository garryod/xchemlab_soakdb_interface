#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use async_graphql::futures_util::StreamExt;
use once_cell::sync::Lazy;
use std::fmt::Debug;
use tokio::sync::broadcast::{channel, Sender};
use tokio_stream::wrappers::BroadcastStream;

/// A multi-producer, multi-consumer static event broker.
///
/// # Example
/// ```
/// use graphql_event_broker::EventBroker;
/// use async_graphql::futures_util::StreamExt;
///
/// #[derive(Debug, Clone)]
/// struct MyEvent(i32);
///
/// static MY_EVENT_BROKER: EventBroker<MyEvent> = EventBroker::new();
///
/// fn publish_event() {
///     MY_EVENT_BROKER.publish(MyEvent(42));
/// }
///
/// fn get_events() {
///     MY_EVENT_BROKER
///         .subscribe()
///         .for_each(|event| async move { println!("{event:?}") });
/// }
///
/// ```
#[derive(Debug)]
pub struct EventBroker<E, const CAPACITY: usize = 1024>(Lazy<Sender<E>>);

impl<E: Clone + Send + 'static, const CAPACITY: usize> EventBroker<E, CAPACITY> {
    /// Constructs an [`EventBroker`], creation of the underlying channel is performed at first use.
    pub const fn new() -> Self {
        Self(Lazy::new(|| channel::<E>(CAPACITY).0))
    }

    /// Publishes an event to the subscribers.
    pub fn publish(&self, event: E) {
        let _ = self.0.send(event);
    }

    /// Creates a subscription to the event. Updates are best effort, as such some may be dropped if the capacity of the underlying channel is exceeded.
    pub fn subscribe(&self) -> impl StreamExt<Item = E> {
        BroadcastStream::new(self.0.subscribe()).filter_map(|message| async move { message.ok() })
    }
}
