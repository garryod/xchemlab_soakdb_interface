use crate::resolvers::{
    cane_mount::{CaneMutation, CaneQuery},
    crystal::{CrystalMutation, CrystalQuery},
    pin_mount::{PinMutation, PinQuery},
    puck_mount::{PuckMutation, PuckQuery},
};
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
pub struct RootQuery(CrystalQuery, CaneQuery, PuckQuery, PinQuery);

#[derive(Debug, Clone, MergedObject, Default)]
pub struct RootMutation(CrystalMutation, CaneMutation, PuckMutation, PinMutation);
