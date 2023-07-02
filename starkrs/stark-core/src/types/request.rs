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
pub struct TxHashClassHash{
    pub transaction_hash:String,
    pub class_hash:String
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct InvokeTransactionV1{
    #[serde(rename = "type")]
    pub type_ :TypeTx,
    pub sender_address:String,
    pub calldata: Vec<String>,
    pub version:u64,
    pub signature:Vec<String>,
    pub nonce: u32,
    pub max_fee:u64,
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct InvokeTransactionV0{
pub contract_address:String,
pub entry_point_selector:String,
pub calldata:Vec<String>
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
pub struct TxHash{
    pub transaction_hash:String
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct TypeOfTx{
    #[serde(rename = "type")]
    pub type_ :TypeTx
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub enum BlockTag{
    pending,
    latest
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub enum BlockNumber{
    BlockTag(BlockTag),
    Number(u64),
    Hash(String)
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub enum TypeTx{
    INVOKE
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct L1HandlerTransaction{
    pub transaction_hash:String,
    pub version:String,
    #[serde(rename = "type")]
    type_:String,
    nonce:String
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct EventFilter{
    pub from_block:String,
    pub to_block:String,
    pub address:String,
    pub keys:Vec<String>
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct ResultPageRequest{
    pub continuation_token:String,
    pub chunk_size:i128
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub enum Transaction{
    EventEmitter(EventEmitter,CommonProperties),
    InvokeTransactionV1(InvokeTransactionV1),
    DeclareV1(DeclareV1, CommonProperties),
    DeployAccountTransactionProperties(DeployAccountTransactionProperties,CommonProperties),
    TxHash(TxHash,DeployAccountTransactionProperties),
    L1HandlerTransaction(L1HandlerTransaction),
    TxHashClassHash(TxHashClassHash),
    InvokeTransactionV0(TypeOfTx,CommonProperties,InvokeTransactionV0),
    EventFilter(EventFilter,ResultPageRequest)
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct BroadcastedTransaction{
    transactions: Vec<Transaction>
}
