use async_trait::async_trait;
use crate::errors::ApiError;

/// Command trait
#[async_trait]
pub trait Command: Send + Sync {
    type Result: Send;
    
    async fn execute(&self) -> Result<Self::Result, ApiError>;
}

/// Query trait
#[async_trait]
pub trait Query: Send + Sync {
    type Result: Send;
    
    async fn execute(&self) -> Result<Self::Result, ApiError>;
}

/// Command handler
#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    async fn handle(&self, command: C) -> Result<C::Result, ApiError>;
}

/// Query handler
#[async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync {
    async fn handle(&self, query: Q) -> Result<Q::Result, ApiError>;
}

/// Command bus
pub struct CommandBus {
    // Placeholder for command routing
}

impl CommandBus {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn dispatch<C: Command>(&self, command: C) -> Result<C::Result, ApiError> {
        command.execute().await
    }
}

impl Default for CommandBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Query bus
pub struct QueryBus {
    // Placeholder for query routing
}

impl QueryBus {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn dispatch<Q: Query>(&self, query: Q) -> Result<Q::Result, ApiError> {
        query.execute().await
    }
}

impl Default for QueryBus {
    fn default() -> Self {
        Self::new()
    }
}

