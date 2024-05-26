#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use linera_sdk::{
    base::WithServiceAbi,
    views::{View, ViewStorageContext},
    Service, ServiceRuntime,
    graphql::GraphQLMutationRoot,
};
use async_graphql::{EmptySubscription, Schema, Object};
use object_test::{MyObject, Operation};

pub struct ApplicationService {
    state: Application,
    runtime: ServiceRuntime<Self>,
}

linera_sdk::service!(ApplicationService);

impl WithServiceAbi for ApplicationService {
    type Abi = object_test::ApplicationAbi;
}

impl Service for ApplicationService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Application::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        ApplicationService { state, runtime }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let schema = Schema::build(
            QueryRoot {
                value: *self.state.value.get(),
                my_object: self.state.my_object.get().clone(),
            },
            Operation::mutation_root(),
            EmptySubscription,
        )
        .finish();
        schema.execute(query).await
    }
}

struct QueryRoot {
    value: u32,
    my_object: MyObject,
}

#[Object]
impl QueryRoot {
    async fn value(&self) -> &u32 {
        &self.value
    }

    async fn my_object(&self) -> &MyObject {
        &self.my_object
    }
}
