use reqwest::Client;
use url::Url;
use serde_json::json;
use serde::{Serialize,Deserialize};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use stark_core::utils::constants::MASK_250;
use stark_core::types::request::{
    TransactionRequest,BroadcastedTransaction,Transaction,DeployAccountTransactionProperties,DeclareV1,InvokeTransactionV1,EventEmitter,CommonProperties};
use num_bigint::BigInt;
use std::str::FromStr;
use num_traits::One;
use num_traits::Num;
use ethers::types::U256;
use ethers::utils::keccak256;
use num_traits::Pow;






pub struct Provider  {
    url:Url,
    client:Client,
}

pub fn keccak_hex(value: &str ) -> String {
    let hash = keccak256(value.as_bytes());
    let z:String = hex::encode(hash);
    z
}

pub fn starknet_keccak(value: &str) -> BigInt {
    let hash_hex = keccak_hex(value);
    let value = BigInt::from_str_radix(&hash_hex, 16).expect("Failed to parse hex string to BigInt");
    let mask_250:BigInt = BigInt::from(2).pow(250u64) - BigInt::one();
    value & mask_250
}

pub fn get_selector_from_name(func_name: &str) -> String {
    format!("{:#x}",starknet_keccak(func_name))
}

impl Provider {

pub fn new(url:&str) -> Result<Self, Box<dyn std::error::Error>>{
    let url = Url::parse(url)?;
    let client = Client::new();
    Ok(Self {url,client})
}

pub async fn request<T,R>(&self,method:&str,params:T) -> Result<R,reqwest::Error>
where 
T:Serialize + Send +  Sync ,
R:DeserializeOwned + Debug + Send,
{
    let request_body = json!({
        "jsonrpc":"2.0",
        "id":1,
        "method":method,
        "params":params,
    });

    let response = self.client.post(self.url.clone()).json(&request_body).send().await?;
    let result = response.json::<R>().await?;
    Ok(result)
}

pub async fn stark_block_number(&self) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_blockNumber";
    let params = json!([]);
    self.request(method,params).await
}

pub async fn stark_chain_id(&self) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_chainId";
    let params = json!([]);
    self.request(method,params).await
}

pub async fn stark_getnonce(&self,address:&str,block_number:u64) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getNonce";
    let params = [serde_json::json!({ "block_number": block_number }), serde_json::json!(address)];
    println!("params : {:?}",params);
    self.request(method,params).await
} 
pub async fn get_block_wtih_tx_hashes(&self,block_number:u64) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getBlockWithTxHashes";
    let params = [serde_json::json!({ "block_number": block_number })];
    self.request(method,params).await
}

pub async fn get_block_with_txs(&self,block_number:u64) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getBlockWithTxs";
    let params = [serde_json::json!({"block_number":block_number})];
    self.request(method,params).await
}

pub async fn get_state_update(&self,block_number:u64) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getStateUpdate";
    let params = [serde_json::json!({"block_number":block_number})];
    self.request(method,params).await
}

pub async fn get_storage_at(&self,contract_address:&str,key:&str,block_number:u64) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getStorageAt";
    let params = [serde_json::json!(contract_address), serde_json::json!(key), serde_json::json!({ "block_number": block_number })];
    self.request(method,params).await
}

pub async fn get_transaction_hash(&self,hash:&str) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getTransactionByHash";
    let params = [serde_json::json!(hash)];
    self.request(method,params).await
}

pub async fn get_transaction_by_blockid_and_index(&self,block_number:u64,index:u64) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getTransactionByBlockIdAndIndex";
    let params = [serde_json::json!({"block_number":block_number}),serde_json::json!(index)];
    self.request(method,params).await
}

pub async  fn get_transaction_receipt(&self,hash:&str) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getTransactionReceipt";
    let params = [serde_json::json!(hash)];
    self.request(method,params).await
}

pub async fn get_class(&self,block_number:u64,class_hash:&str) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getClass";
    let params = [serde_json::json!({"block_number":block_number}),serde_json::json!(class_hash)];
    self.request(method,params).await
}

pub async fn get_class_hash_at(&self,block_number:u64,contract_address:&str) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getClassHashAt";
    let params = [serde_json::json!({"block_number":block_number}),serde_json::json!(contract_address)];
    self.request(method,params).await
}

pub async fn get_class_at(&self,block_number:u64,contract_address:&str) -> Result<serde_json::Value,reqwest::Error> {
    let method = "starknet_getClassAt";
    let params = [serde_json::json!({"block_number":block_number}),serde_json::json!(contract_address)];
    self.request(method,params).await
} 

pub async fn get_block_transaction_count(&self,block_number:u64) -> Result<serde_json::Value,reqwest::Error> {
    let method = "starknet_getBlockTransactionCount";
    let params = [serde_json::json!({"block_number":block_number})];
    self.request(method,params).await
}


pub async fn call(&self,tx:TransactionRequest,block_number:u64) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_call";
    let params = [serde_json::json!(tx),serde_json::json!({"block_number":block_number})];
    self.request(method,params).await
}

pub async fn estimate_fee(&self,tx:Vec<Transaction>,block_number:u64) -> Vec<Result<serde_json::Value,reqwest::Error>>{
    let method = "starknet_estimateFee";
    let mut results:Vec<Result<serde_json::Value,reqwest::Error>> = Vec::new();
    for transaction in tx {
      let result =  match transaction {
            Transaction::EventEmitter(ev,common) => {
                let params = [serde_json::json!(ev),serde_json::json!({"block_number":block_number})];
                self.request(method,params).await
            },
            Transaction::DeclareV1(dec,common) => {
                let params = [serde_json::json!(dec),serde_json::json!({"block_number":block_number})];
                self.request(method,params).await
            },
            Transaction::DeployAccountTransactionProperties(dep,common) => {
                let params = [serde_json::json!(dep),serde_json::json!({"block_number":block_number})];
                self.request(method,params).await
            },
            Transaction::InvokeTransactionV1(inv,commom ) => {
                let params = [serde_json::json!(inv),serde_json::json!({"block_number":block_number})];
                self.request(method,params).await
            }
        };
        results.push(result);
    }
    results
}


}




#[cfg(test)]
mod tests {
    use super::Provider;
    use serde_json::Value;
    use ethers::{utils::keccak256};
    use stark_core::types::request::TransactionRequest;
    use primitive_types::{H256};
    use sha3::{Digest,Keccak256};
    use crate::rpc::get_selector_from_name;
    extern crate hex;
    use hex::encode;
    use stark_core::utils::constants::MASK_250;
    use ethers::types::U256;
    use stark_core::types::request::{
        BroadcastedTransaction,Transaction,DeployAccountTransactionProperties,DeclareV1,InvokeTransactionV1,EventEmitter,CommonProperties};
    use serde::{Deserialize, Serialize};

    // Helper function to create a Provider instance for testing
    fn setup_provider() -> Provider {
        let url = "https://starknet-mainnet.public.blastapi.io";
        Provider::new(url).unwrap()
    }

    #[tokio::test]
    async fn test_provider_creation() {
        let url = "https://opt-goerli.g.alchemy.com/v2/P17HzVr6oLOZpfyUNLNL9yOe0tICFqbW";
        let provider = Provider::new(url).unwrap();
        assert_eq!(provider.url.as_str(), url);
    }

    #[tokio::test]
    async fn test_request() {
        let provider = setup_provider();

        // Example: get the balance of an Ethereum address
        let method = "starknet_blockNumber";
        let address = "0x0424701df5d425e6a4e2577004cb4d412076b730899ad734223996cdabc11e2a";
        let block_number = "latest";

        let params = vec![address, block_number];

        let result: Result<Value, _> = provider.request(method,params).await;
        println!("result value : {}",result.unwrap());
        // assert!result.is_ok());
    }

    #[tokio::test]
    async fn test_block_number() {
        let provider = setup_provider();
        let result  = provider.stark_block_number().await;
        assert!(result.is_ok());
        println!("block number : {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_chain_id(){
        let provider = setup_provider();
        let result  = provider.stark_chain_id().await;
        assert!(result.is_ok());
        println!("chain id : {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_getnonce() {
        let provider = setup_provider();
        let result = provider.stark_getnonce("0x06c3731d9669dc2928e44978afe237a862f10774aa2a1876fbaddb604a50d968",52668).await;
        assert!(result.is_ok());
        println!("nonce : {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_get_block_with_tx_hashes() {
        let provider = setup_provider();
        let result = provider.get_block_wtih_tx_hashes(52668).await;
        assert!(result.is_ok());
        println!("block with tx hashes : {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_get_block_with_txs() {
        let provider = setup_provider();
        let result = provider.get_block_with_txs(52668).await;
        assert!(result.is_ok());
        println!("block with txs : {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_get_state_update() {
        let provider = setup_provider();
        let result = provider.get_state_update(52668).await;
        assert!(result.is_ok());
        println!("state update : {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_get_storage_at() {
        let provider = setup_provider();
        let result = provider.get_storage_at("0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892","0x04",52668).await;
        assert!(result.is_ok());
        println!("storage at : {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_get_transaction_hash() {
        let provider = setup_provider();
        let result = provider.get_transaction_hash("0x035475b21b0bc1799053bbf41f191d480e81bdb8eea6874d214dc5cc9882092e").await;
        assert!(result.is_ok());
        println!("transaction details {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_get_transaction_by_blockid_and_index() {
        let provider = setup_provider();
        let result = provider.get_transaction_by_blockid_and_index(54980,1).await;
        assert!(result.is_ok());
        println!("transaction details {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_get_transaction_receipt() {
        let provider = setup_provider();
        let result = provider.get_transaction_receipt("0x035475b21b0bc1799053bbf41f191d480e81bdb8eea6874d214dc5cc9882092e").await;
        assert!(result.is_ok());
        println!("transaction details {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_get_class() {
        let provider = setup_provider();
        let result = provider.get_class(54980,"0x0108a32ec851d37c8f15387dadc87dc80c302c5278b24211ea5b227a4bfdc752").await;
        assert!(result.is_ok());
        println!("class details {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_get_class_hash_at() {
        let provider = setup_provider();
        let result = provider.get_class_hash_at(54980,"0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892").await;
        assert!(result.is_ok());
        println!("class hash details {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_get_class_at(){
        let provider = setup_provider();
        let result = provider.get_class_at(54980,"0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892").await;
        assert!(result.is_ok());
        println!("class details {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_get_block_transaction_count() {
        let provider = setup_provider();
        let result = provider.get_block_transaction_count(54980).await;
        assert!(result.is_ok());
        println!("block transaction count {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_call() {
        let provider = setup_provider();
        let func_name = "approve";
        let calldatavec: Vec<String> = vec![
            "0x7a6f98c03379b9513ca84cca1373ff452a7462a3b61598f0af5bb27ad7f76d1".to_string(),
            "5840374794613697".to_string()
        ];
        let res = get_selector_from_name(func_name);
        println!("entry point selector is : {}",res);
            let tx: TransactionRequest = TransactionRequest { 
            contract_address:Some("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7".to_string()),
            entry_point_selector:Some("0x0219209e083275171774dab1df80982e9df2096516f06319c5c6d71ae0a8480c".to_string()),
            calldata:calldatavec
        };
        let result = provider.call(tx,87673).await;
        assert!(result.is_ok());
        println!("call res {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_estimate_fee() {
        let provider = setup_provider();
        let common_properties = CommonProperties{
            max_fee: 1837758943871232,
            version:1,
            signature:vec!["0x27c41433bb3263a2e7d357a6d7d2b856b19a27b5c6089fa5ec4a55de507c49a".to_string(),"0x18794acabc26005629b1d857cd6e4e7e850614aa3e594b39650858836418d8e".to_string()],
            nonce:7
        };
        let event_emitter = EventEmitter{
            type_:"sf".to_string(),
            contract_class: "sdf".to_string(),
            sender_address:"dfd".to_string(),
            compiler_class_hash:"fsd".to_string()
        };
        let invoke_transaction = InvokeTransactionV1{
            sender_address:"0x00451d8a3566c18929c6c7312b7fee619dcb4210870ad5534680da0b17cee431".to_string(),
            calldata: vec!["0xb9b1a4373de5b1458e598df53195ea3204aa926f46198b50b32ed843ce508b".to_string(),
            "0x4563918244f40000".to_string()
            ]
        };
        let declare_v1 = DeclareV1{
            type_:"fd".to_string(),
            contract_class:"dfsd".to_string(),
            sender_address:"dfsdfdsf".to_string()
        };
        let deploy_account_tx_properties: DeployAccountTransactionProperties = DeployAccountTransactionProperties{
            type_:"sfds".to_string(),
            contract_address_salt:"dfds".to_string(),
            constructor_calldata:vec!["dfdsfsd".to_string()],
            class_hash:"fsdfdsfsd".to_string()
        }; 
        let transactions = vec![
            // Transaction::EventEmitter(event_emitter,common_properties.clone())
            Transaction::InvokeTransactionV1(invoke_transaction,common_properties.clone())
            // Transaction::DeclareV1(declare_v1,common_properties.clone()),
            // Transaction::DeployAccountTransactionProperties(deploy_account_tx_properties,common_properties.clone()),
        ];
        let result = provider.estimate_fee(transactions,59268).await;
        assert!(result[0].is_ok());
        // assert!(result[1].is_ok());
        // assert!(result[2].is_ok());
        // assert!(result[3].is_ok());
        println!("result index 0 is {}",result[0].as_ref().unwrap());
        // println!("result index 1 is {}",result[1].as_ref().unwrap());
        // println!("result index 2 is {}",result[2].as_ref().unwrap());
        // println!("result index 3 is {}",result[3].as_ref().unwrap());
    }
}
