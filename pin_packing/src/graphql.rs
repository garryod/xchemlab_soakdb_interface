use async_graphql::{extensions::Tracing, EmptyMutation, EmptySubscription, MergedObject, Schema};

pub fn build_schema() -> RootSchema {
    Schema::build(
        RootQuery::default(),
        EmptyMutation::default(),
        EmptySubscription::default(),
    )
    .extension(Tracing)
    .finish()
}

pub type RootSchema = Schema<RootQuery, EmptyMutation, EmptySubscription>;

#[derive(Debug, Clone, MergedObject, Default)]
pub struct RootQuery;
