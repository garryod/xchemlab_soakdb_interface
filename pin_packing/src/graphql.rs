use crate::resolvers::cane::{CaneMutation, CaneQuery};
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

pub fn root_schema_builder() -> SchemaBuilder<RootQuery, RootMutation, EmptySubscription> {
    Schema::build(
        RootQuery::default(),
        RootMutation::default(),
        EmptySubscription::default(),
    )
}

pub type RootSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

#[derive(Debug, Clone, MergedObject, Default)]
pub struct RootQuery(CaneQuery);

#[derive(Debug, Clone, MergedObject, Default)]
pub struct RootMutation(CaneMutation);
