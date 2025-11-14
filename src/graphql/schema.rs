use async_graphql::{Schema, EmptySubscription};
use super::types::{QueryRoot, MutationRoot};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .finish()
}

