use linera_sdk::views::{linera_views, RegisterView, RootView, ViewStorageContext};
use object_test::MyObject;

#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct Application {
    pub value: RegisterView<u32>,
    // Add fields here.
    pub my_object: RegisterView<MyObject>,
}
