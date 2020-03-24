//! A lightweight metrics facade.
//!
//! The `metrics` crate provides a single metrics API that abstracts over the actual metrics
//! implementation.  Libraries can use the metrics API provided by this crate, and the consumer of
//! those libraries can choose the metrics implementation that is most suitable for its use case.
//!
//! If no metrics implementation is selected, the facade falls back to a "noop" implementation that
//! ignores all metrics.  The overhead in this case is very small - an atomic load and comparison.
//!
//! # Use
//! The basic use of the facade crate is through the four metrics macros: [`counter!`], [`gauge!`],
//! [`timing!`], and [`value!`].  These macros correspond to updating a counter, updating a gauge,
//! updating a histogram based on a start/end, and updating a histogram with a single value.
//!
//! Both [`timing!`] and [`value!`] are effectively identical in so far as that they both translate
//! to recording a single value to an underlying histogram, but [`timing!`] is provided for
//! contextual consistency: if you're recording a measurement of the time passed during an
//! operation, the end result is a single value, but it's more of a "timing" value than just a
//! "value".  The [`timing!`] macro also has a branch to accept the start and end values which
//! allows for a potentially clearer invocation.
//!
//! ## In libraries
//! Libraries should link only to the `metrics` crate, and use the provided macros to record
//! whatever metrics will be useful to downstream consumers.
//!
//! ### Examples
//!
//! ```rust
//! use metrics::{timing, counter};
//!
//! # use std::time::Instant;
//! # pub fn run_query(_: &str) -> u64 { 42 }
//! pub fn process(query: &str) -> u64 {
//!     let start = Instant::now();
//!     let row_count = run_query(query);
//!     let end = Instant::now();
//!
//!     timing!("process.query_time", start, end);
//!     counter!("process.query_row_count", row_count);
//!
//!     row_count
//! }
//! # fn main() {}
//! ```
//!
//! ## In executables
//!
//! Executables should choose a metrics implementation and initialize it early in the runtime of
//! the program.  Metrics implementations will typically include a function to do this.  Any
//! metrics recordered before the implementation is initialized will be ignored.
//!
//! The executable itself may use the `metrics` crate to record metrics well.
//!
//! ### Warning
//!
//! The metrics system may only be initialized once.
//!
//! # Available metrics implementations
//!
//! * # Native recorder:
//!     * [metrics-runtime]
//!
//! # Implementing a Recorder
//!
//! Recorders implement the [`Recorder`] trait.  Here's a basic example which writes the
//! metrics in text form via the `log` crate.
//!
//! ```rust
//! # use std::collections::HashMap;
//! use log::info;
//! use metrics::{Identifier, Key, Recorder};
//!
//! struct LogRecorder {
//!     keys: HashMap<Identifier, Key>,
//! }
//!
//!  FIX THIS EXAMPLE
//!
//! impl Recorder for LogRecorder {
//!     fn register_counter(&self, key: Key) -> Identifier {
//!         Identifier::default()
//!     }
//!
//!     fn register_gauge(&self, key: Key) -> Identifier {
//!         Identifier::default()
//!     }
//!
//!     fn register_histogram(&self, key: Key) -> Identifier {
//!         Identifier::default()
//!     }
//!
//!     fn increment_counter(&self, id: &Identifier, value: u64) {
//!         let key = self.keys.get(id).expect("invalid identifier");
//!         info!("counter '{}' -> {}", key, value);
//!     }
//!
//!     fn update_gauge(&self, id: &Identifier value: i64) {
//!         let key = self.keys.get(id).expect("invalid identifier");
//!         info!("gauge '{}' -> {}", key, value);
//!     }
//!
//!     fn record_histogram(&self, id: &Identifier, value: u64) {
//!         let key = self.keys.get(id).expect("invalid identifier");
//!         info!("histogram '{}' -> {}", key, value);
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! Recorders are installed by calling the [`set_recorder`] function.  Recorders should provide a
//! function that wraps the creation and installation of the recorder:
//!
//! ```rust
//! # use metrics::{Recorder, Key, Identifier};
//! # use metrics::Key;
//! # struct LogRecorder;
//! # impl Recorder for LogRecorder {
//! #     fn register_counter(&self, _key: Key) -> Identifier { Identifier::default() }
//! #     fn register_gauge(&self, _key: Key) -> Identifier { Identifier::default() }
//! #     fn register_histogram(&self, _key: Key) -> Identifier { Identifier::default() }
//! #     fn increment_counter(&self, _id: &Identifier, _value: u64) {}
//! #     fn update_gauge(&self, _id: &Identifier, _value: i64) {}
//! #     fn record_histogram(&self, _id: &Identifier, _value: u64) {}
//! # }
//! use metrics::SetRecorderError;
//!
//! static RECORDER: LogRecorder = LogRecorder;
//!
//! pub fn init() -> Result<(), SetRecorderError> {
//!     metrics::set_recorder(&RECORDER)
//! }
//! # fn main() {}
//! ```
//!
//! # Use with `std`
//!
//! `set_recorder` requires you to provide a `&'static Recorder`, which can be hard to
//! obtain if your recorder depends on some runtime configuration.  The `set_boxed_recorder`
//! function is available with the `std` Cargo feature.  It is identical to `set_recorder` except
//! that it takes a `Box<Recorder>` rather than a `&'static Recorder`:
//!
//! ```rust
//! # use metrics::Recorder;
//! # use metrics_core::Key;
//! # struct LogRecorder;
//! # impl Recorder for LogRecorder {
//! #     fn register_counter(&self, _key: Key) -> Identifier { Identifier::default() }
//! #     fn register_gauge(&self, _key: Key) -> Identifier { Identifier::default() }
//! #     fn register_histogram(&self, _key: Key) -> Identifier { Identifier::default() }
//! #     fn increment_counter(&self, _id: &Identifier, _value: u64) {}
//! #     fn update_gauge(&self, _id: &Identifier, _value: i64) {}
//! #     fn record_histogram(&self, _id: &Identifier, _value: u64) {}
//! # }
//! use metrics::SetRecorderError;
//!
//! # #[cfg(feature = "std")]
//! pub fn init() -> Result<(), SetRecorderError> {
//!     metrics::set_boxed_recorder(Box::new(LogRecorder))
//! }
//! # fn main() {}
//! ```
//!
//! [metrics-runtime]: https://docs.rs/metrics-runtime
#![deny(missing_docs)]
use proc_macro_hack::proc_macro_hack;

mod common;
pub use self::common::*;

mod key;
pub use self::key::*;

mod label;
pub use self::label::*;

mod recorder;
pub use self::recorder::*;

mod macros;
pub use self::macros::*;

/// Increments a counter.
#[proc_macro_hack]
pub use metrics_macros::increment;
