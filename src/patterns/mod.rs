pub mod event_sourcing;
pub mod cqrs;

#[cfg(feature = "database-postgres")]
pub mod postgres_event_store;

pub use event_sourcing::{Event, EventStore, InMemoryEventStore, Aggregate, EventSourcingRepository, StoredEvent};
pub use cqrs::{Command, Query, CommandHandler, QueryHandler, CommandBus, QueryBus};

#[cfg(feature = "database-postgres")]
pub use postgres_event_store::PostgresEventStore;

