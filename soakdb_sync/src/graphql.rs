use crate::resolvers::{export::ExportMutation, import::ImportQuery};
use async_graphql::{EmptySubscription, MergedObject, MergedSubscription, Schema, SchemaBuilder};

pub fn root_schema_builder() -> SchemaBuilder<RootQuery, RootMutation, EmptySubscription> {
    Schema::build(
        RootQuery::default(),
        RootMutation::default(),
        EmptySubscription::default(),
    )
}

pub type RootSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

#[derive(Debug, Clone, MergedObject, Default)]
pub struct RootQuery(ImportQuery);

#[derive(Debug, Clone, MergedObject, Default)]
pub struct RootMutation(ExportMutation);

#[derive(Debug, Clone, MergedSubscription, Default)]
pub struct RootSubscription;
