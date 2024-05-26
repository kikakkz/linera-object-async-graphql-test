use linera_sdk::{
    base::{ContractAbi, ServiceAbi},
    graphql::GraphQLMutationRoot,
};
use async_graphql::{InputObject, Request, Response, SimpleObject};
use serde::{Deserialize, Serialize};

pub struct ApplicationAbi;

impl ContractAbi for ApplicationAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for ApplicationAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, SimpleObject, InputObject)]
pub struct InstantiationArgument {
    value: u32
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq, InputObject)]
pub struct InnerObject {
    values: Vec<u32>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq, InputObject)]
pub struct MyObject {
    inner_object: InnerObject,
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    // MyObject { my_object: MyObject },
    AnotherMyObject,
}
