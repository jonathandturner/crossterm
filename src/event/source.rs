use std::time::Duration;

use super::InternalEvent;

#[cfg(feature = "event-stream")]
use super::sys::Waker;

#[cfg(unix)]
pub(crate) mod unix;
#[cfg(windows)]
pub(crate) mod windows;

/// An interface for trying to read an `InternalEvent` within an optional `Duration`.
pub(crate) trait EventSource: Sync + Send {
    /// Tries to read an `InternalEvent` within the given duration.
    ///
    /// This function takes in an optional duration.
    /// * `None`: blocks indefinitely until an event is able to be read.
    /// * `Some(duration)`: blocks for the given duration.
    ///
    /// Returns:
    /// `Ok(Some(event))`: in case an event is ready.
    /// `Ok(None)`: in case an event is not ready.
    fn try_read(&mut self, timeout: Option<Duration>) -> crate::Result<Option<InternalEvent>>;

    #[cfg(feature = "event-stream")]
    fn waker(&self) -> Waker;
}
