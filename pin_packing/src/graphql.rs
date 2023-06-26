use crate::resolvers::{
    cane_mount::{CaneMutation, CaneQuery},
    crystal::{CrystalMutation, CrystalQuery},
    pin_library::{PinLibraryMutation, PinLibraryQuery},
    pin_mount::{PinMountMutation, PinMountQuery},
    puck_library::{PuckLibraryMutation, PuckLibraryQuery},
    puck_mount::{PuckMountMutation, PuckMountQuery},
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
pub struct RootQuery(
    CrystalQuery,
    CaneQuery,
    PuckLibraryQuery,
    PuckMountQuery,
    PinLibraryQuery,
    PinMountQuery,
);

#[derive(Debug, Clone, MergedObject, Default)]
pub struct RootMutation(
    CrystalMutation,
    CaneMutation,
    PuckLibraryMutation,
    PuckMountMutation,
    PinLibraryMutation,
    PinMountMutation,
);
