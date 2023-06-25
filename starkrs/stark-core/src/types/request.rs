use crate::{
    Address,Selector
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]  // Notice we added Serialize here
pub struct TransactionRequest {
    pub contract_address: Option<String>,
    pub entry_point_selector:Option<String>,
    pub calldata: Vec<String>
}
 
#[derive(Serialize, Deserialize,Debug,Clone)] // Notice we added Serialize
pub struct CommonProperties{
    pub max_fee:u64,
    pub version:u32,
    pub signature:Vec<String>,
    pub nonce: u32
}


#[derive(Serialize, Deserialize,Debug,Clone)] 
pub struct EventEmitter{
    #[serde(rename = "type")]
    pub type_:String,
    pub contract_class:String,
    pub sender_address:String,
    pub compiler_class_hash:String
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct InvokeTransactionV1{
    pub sender_address:String,
    pub calldata: Vec<String>
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct DeclareV1{
    #[serde(rename = "type")]
    pub type_:String,
    pub contract_class:String,
    pub sender_address:String
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct DeployAccountTransactionProperties{
    #[serde(rename = "type")]
    pub type_:String,
    pub contract_address_salt:String,
    pub constructor_calldata: Vec<String>,
    pub class_hash:String
}


#[derive(Serialize, Deserialize,Debug,Clone)]
pub enum Transaction{
    EventEmitter(EventEmitter,CommonProperties),
    InvokeTransactionV1(InvokeTransactionV1, CommonProperties),
    DeclareV1(DeclareV1, CommonProperties),
    DeployAccountTransactionProperties(DeployAccountTransactionProperties,CommonProperties)
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct BroadcastedTransaction{
    transactions: Vec<Transaction>
}
