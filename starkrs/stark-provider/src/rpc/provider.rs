use reqwest::Client;
use url::Url;
use serde_json::json;
use serde::{Serialize};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use stark_core::types::request::{
    TransactionRequest,Transaction,DeclareV1,InvokeTransactionV1,EventEmitter,CommonProperties,TypeTx,BlockNumber};
use num_bigint::BigInt;
use num_traits::One;
use num_traits::Num;
use ethers::types::{U256 };
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

pub async fn stark_blockhash_and_number(&self) ->Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_blockHashAndNumber";
    let params = json!([]);
    self.request(method,params).await
} 

pub async fn stark_getnonce(&self,address:&str,val:BlockNumber) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getNonce";

    let result: Result<serde_json::Value, reqwest::Error>;
    match val{
        BlockNumber::Number(val) =>{
            let params = [serde_json::json!({ "block_number": val }),serde_json::json!(address)];
            result = self.request(method,params).await
        }
        BlockNumber::BlockTag(val) =>{
            let params = [serde_json::json!(val),serde_json::json!(address)];
            result= self.request(method,params).await
        }
        BlockNumber::Hash(val) =>{
            let params = [serde_json::json!({"block_hash":val}),serde_json::json!(address)];
            result = self.request(method,params).await
        }
    }
    result

   
} 

pub async fn get_block_with_tx_hashes(&self,val:BlockNumber) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getBlockWithTxHashes";
    let result: Result<serde_json::Value, reqwest::Error>;
    match val{
        BlockNumber::Number(val) =>{
            let params = [serde_json::json!({ "block_number": val })];
            result = self.request(method,params).await
        }
        BlockNumber::BlockTag(val) =>{
            let params = [serde_json::json!(val)];
            result= self.request(method,params).await
        }
        BlockNumber::Hash(val) =>{
            let params = [serde_json::json!({"block_hash":val})];
            result = self.request(method,params).await
        }
    }
    result
}

pub async fn get_block_with_txs(&self,val:BlockNumber) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getBlockWithTxs";
    let result: Result<serde_json::Value, reqwest::Error>;
    match val{
        BlockNumber::BlockTag(val) =>{
            let params  = [serde_json::json!(val)];
            result =self.request(method,params).await
        }
        BlockNumber::Hash(val)=>{
            let params  = [serde_json::json!({"block_hash":val})];
            result= self.request(method,params).await
        }
        BlockNumber::Number(val) =>{
            let params = [serde_json::json!({"block_number":val})];
            result = self.request(method,params).await
        }
    }
   result
}

pub async fn get_state_update(&self,val:BlockNumber) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getStateUpdate";
    let result: Result<serde_json::Value, reqwest::Error>;
    match val{
        BlockNumber::Number(val) =>{
            let params = [serde_json::json!({"block_number":val})];
            result = self.request(method,params).await;
        }
        BlockNumber::Hash(val) =>{
            let params = [serde_json::json!({"block_hash":val})];
            result = self.request(method,params).await;

        },
        _=>{
            result = Ok(serde_json::Value::Null)
        }
    };
    result
    
}

pub async fn get_storage_at(&self,contract_address:&str,key:&str,val:BlockNumber) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getStorageAt";
    let result:Result<serde_json::Value,reqwest::Error>;
    result = match val {
        BlockNumber::BlockTag(val) =>{
            let params  = [serde_json::json!(contract_address),serde_json::json!(key),serde_json::json!(val)];
            self.request(method,params).await
        }
        BlockNumber::Hash(val)=>{
            let params  = [serde_json::json!(contract_address),serde_json::json!(key),serde_json::json!({"block_hash":val})];
            self.request(method,params).await
        }
        BlockNumber::Number(val) =>{
            let params = [serde_json::json!(contract_address),serde_json::json!(key),serde_json::json!({"block_number":val})];
            self.request(method,params).await
        }
    };
 result
}

pub async fn get_transaction_by_hash(&self,hash:String) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getTransactionByHash";
    let params = [serde_json::json!(hash)];
    self.request(method,params).await
}

pub async fn get_transaction_by_blockid_and_index(&self,block_number:u64,index:u64) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getTransactionByBlockIdAndIndex";
    let params = [serde_json::json!({"block_number":block_number}),serde_json::json!(index)];
    self.request(method,params).await
}

pub async  fn get_transaction_receipt(&self,hash:String) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getTransactionReceipt";
    let params = [serde_json::json!(hash)];
    self.request(method,params).await
}

pub async fn get_class(&self,val:BlockNumber,class_hash:&str) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getClass";

    let result:Result<serde_json::Value,reqwest::Error>;
    result = match val {
        BlockNumber::BlockTag(val) =>{
            let params  = [serde_json::json!(class_hash),serde_json::json!(val)];
            self.request(method,params).await
        }
        BlockNumber::Hash(val)=>{
            let params  = [serde_json::json!(class_hash),serde_json::json!({"block_hash":val})];
            self.request(method,params).await
        }
        BlockNumber::Number(val) =>{
            let params = [serde_json::json!(class_hash),serde_json::json!({"block_number":val})];
            self.request(method,params).await
        }
    };
 result

    
}

pub async fn get_class_hash_at(&self,val:BlockNumber,contract_address:&str) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getClassHashAt";
    let result:Result<serde_json::Value,reqwest::Error>;
    result = match val {
        BlockNumber::BlockTag(val) =>{
            let params  = [serde_json::json!(val),serde_json::json!(contract_address)];
            self.request(method,params).await
        }
        BlockNumber::Hash(val)=>{
            let params  = [serde_json::json!({"block_hash":val}),serde_json::json!(contract_address)];
            self.request(method,params).await
        }
        BlockNumber::Number(val) =>{
            let params = [serde_json::json!({"block_number":val}),serde_json::json!(contract_address)];
            self.request(method,params).await
        }
    };
 result 
}

pub async fn get_class_at(&self,val:BlockNumber,contract_address:&str) -> Result<serde_json::Value,reqwest::Error> {
    let method = "starknet_getClassAt";
    let result:Result<serde_json::Value,reqwest::Error>;
    result = match val {
        BlockNumber::BlockTag(val) =>{
            let params  = [serde_json::json!(val),serde_json::json!(contract_address)];
            self.request(method,params).await
        }
        BlockNumber::Hash(val)=>{
            let params  = [serde_json::json!({"block_hash":val}),serde_json::json!(contract_address)];
            self.request(method,params).await
        }
        BlockNumber::Number(val) =>{
            let params = [serde_json::json!({"block_number":val}),serde_json::json!(contract_address)];
            self.request(method,params).await
        }
    };
 result
    
} 

pub async fn get_block_transaction_count(&self,val:BlockNumber) -> Result<serde_json::Value,reqwest::Error> {
    let method = "starknet_getBlockTransactionCount";
    let result:Result<serde_json::Value,reqwest::Error>;
    result = match val {
        BlockNumber::BlockTag(val) =>{
            let params  = [serde_json::json!(val),];
            self.request(method,params).await
        }
        BlockNumber::Hash(val)=>{
            let params  = [serde_json::json!({"block_hash":val})];
            self.request(method,params).await
        }
        BlockNumber::Number(val) =>{
            let params = [serde_json::json!({"block_number":val})];
            self.request(method,params).await
        }
    };
 result
    
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
                let params = [serde_json::json!({"transaction":ev, "common":common}),serde_json::json!({"block_number":block_number})];
                self.request(method,params).await
            },
            Transaction::DeclareV1(dec,common) => {
                let params = [serde_json::json!({"transaction":dec,"common":common}),serde_json::json!({"block_number":block_number})];
                self.request(method,params).await
            },
            Transaction::DeployAccountTransactionProperties(dep,common) => {
                let params = [serde_json::json!({"transaction":dep,"common":common}),serde_json::json!({"block_number":block_number})];
                self.request(method,params).await
            },
            Transaction::InvokeTransactionV0(tp,cp,iv) => {
                let params = [serde_json::json!(tp),serde_json::json!(cp),serde_json::json!(iv),serde_json::json!({"block_number":block_number})];
                self.request(method,params).await
            },
            Transaction::InvokeTransactionV1(iv) => {
                let params = [serde_json::json!(iv),serde_json::json!({"block_number":block_number})];
                self.request(method,params).await
            },
            _=>{
                Ok(serde_json::Value::Null)
            }
        };
        results.push(result);
    }
    results
}

// pub async fn pending_Transactions(&self, tx:Vec<Transaction>) -> Vec<Result<serde_json::Value,reqwest::Error>>{
//     let method = "starknet_pendingTransactions";
//     let mut results:Vec<Result<serde_json::Value,reqwest::Error>> = Vec::new();
//     for transaction in tx {
//         let result = match transaction {
//             Transaction::TxHash(hash,dp) => {
//                 let params = [serde_json::json!({"transaction":hash,"common":dp})];
//                 self.request(method,params).await
//             }
//             Transaction::InvokeTransactionV1(inv,cp)=>{
//                 let params = [serde_json::json!(inv)];
//                 self.request(method,params).await
//             }
//             Transaction::L1HandlerTransaction(lht) =>{
//                 let params = [serde_json::json!(lht)];
//                 self.request(method,params).await
//             }
//             Transaction::DeployAccountTransactionProperties(tp ,cp) =>{
//                 let params = [serde_json::json!({"transaction":tp,"commom":cp})];
//                 self.request(method,params).await
//             }
//             Transaction::TxHashClassHash(tc) =>{
//                 let params  = [serde_json::json!(tc)];
//                 self.request(method,params).await
//             }
//             Transaction::InvokeTransactionV0(iv0)=>{
//                 let params = [serde_json::json!(iv0)];
//                 self.request(method,params).await
//             }
//             _=>{
//                 Ok(serde_json::Value::Null)
//             }
//         };
//         results.push(result);
//     }
//     results
// }


pub async fn syncing(&self) ->Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_syncing";
    let params = json!([]);
    self.request(method,params).await   
}

pub async fn getEvents(&self, tx:Transaction) -> Result<serde_json::Value,reqwest::Error>{
    let method = "starknet_getEvents";
     if let Transaction::EventFilter(ev,rpp) = tx {
    let params = [serde_json::json!({"ev":ev,"rp":rpp})];
    self.request(method,params).await
     } else {
        Ok(serde_json::Value::Null)
     }
}

}

#[cfg(test)]
mod tests {
    use super::Provider;
    use serde_json::Value;
    use stark_core::types::request::TransactionRequest;
    use primitive_types::{H256};
    use crate::rpc::get_selector_from_name;
    extern crate hex;
  
    use stark_core::types::request::{
        BroadcastedTransaction,Transaction,DeployAccountTransactionProperties,DeclareV1,InvokeTransactionV1,InvokeTransactionV0,EventEmitter,CommonProperties,TypeTx,TypeOfTx,BlockNumber};
    use stark_core::types::request::BlockTag;  
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

        let block_number_result = provider.stark_getnonce("0x03a76598b598d9b611dfb611ad8ececa09cec9f1fb3a41f7ad79e1a134018199",BlockNumber::Number(90822)).await;
        assert!(block_number_result.is_ok());
        println!("nonce result using block number  : {}",block_number_result.unwrap());

        let block_hash_result = provider.stark_getnonce("0x03a76598b598d9b611dfb611ad8ececa09cec9f1fb3a41f7ad79e1a134018199",BlockNumber::Hash("0x046e5fd2095a1f30b99756ff209740a3893b31f5a1198347a23c3e7fc8ff9e5c".to_string())).await;
        assert!(block_hash_result.is_ok());
        println!(" nonce result using block  hash : {} ", block_hash_result.unwrap());

        let latest_block_result = provider.stark_getnonce("0x03a76598b598d9b611dfb611ad8ececa09cec9f1fb3a41f7ad79e1a134018199", BlockNumber::BlockTag(BlockTag::latest)).await;
        assert!(latest_block_result.is_ok());
        println!("nonce result using latest block hash : {}", latest_block_result.unwrap());

        let pending_block_result = provider.stark_getnonce("0x03a76598b598d9b611dfb611ad8ececa09cec9f1fb3a41f7ad79e1a134018199", BlockNumber::BlockTag(BlockTag::latest)).await;
        assert!(pending_block_result.is_ok());
        println!("nonce result using latest block hash : {}", pending_block_result.unwrap());

    }

    #[tokio::test]
    async fn test_get_block_with_tx_hashes() {
        let provider = setup_provider();

        let block_number_result = provider.get_block_with_tx_hashes(BlockNumber::Number(95812)).await;
        assert!(block_number_result.is_ok());
        println!("block with tx hashes using block number : {}",block_number_result.unwrap());

        let block_hash_result = provider.get_block_with_tx_hashes(BlockNumber::Hash("0x04029c604ad1da801b55ef0ff6ac5d72153564efb415e3e45bd27bd4abdb61bd".to_string())).await;
        assert!(block_hash_result.is_ok());
        println!("block hash with tx hashes using block hashes : {}",block_hash_result.unwrap());

        let pending_block_result = provider.get_block_with_tx_hashes(BlockNumber::BlockTag(BlockTag::pending)).await;
        assert!(pending_block_result.is_ok());
        println!("pending lbock tx hashes:{}",pending_block_result.unwrap());
        
        let latest_block_result = provider.get_block_with_tx_hashes(BlockNumber::BlockTag((BlockTag::latest))).await;
        assert!(latest_block_result.is_ok());
        println!("latest block tx hashes :{}",latest_block_result.unwrap());

    }

    #[tokio::test]
    async fn test_get_block_with_txs() {
        let provider = setup_provider();
        let result = provider.get_block_with_txs(BlockNumber::BlockTag(BlockTag::latest)).await;
        assert!(result.is_ok());
        println!("block with txs using latest block tag  : {}",result.unwrap());

        let pending_block_result = provider.get_block_with_txs(BlockNumber::BlockTag(BlockTag::pending)).await;
        assert!(pending_block_result.is_ok());
        println!("block with txs using pending block tag : {}",pending_block_result.unwrap());

        let block_number_result= provider.get_block_with_txs(BlockNumber::Number(95812)).await;
        assert!(block_number_result.is_ok());
        println!("block txs using block number:{}",block_number_result.unwrap());

        let block_hash_result = provider.get_block_with_txs(BlockNumber::Hash("0x04029c604ad1da801b55ef0ff6ac5d72153564efb415e3e45bd27bd4abdb61bd".to_string())).await;
        assert!(block_hash_result.is_ok());
        println!("block txs using block hash  :{}",block_hash_result.unwrap());
    }

    #[tokio::test]
    async fn test_get_state_update() {
        let provider = setup_provider();

        let block_number_result = provider.get_state_update(BlockNumber::Number(95812)).await;
        assert!(block_number_result.is_ok());
        println!("state update : {}",block_number_result.unwrap());

        let block_hash_result= provider.get_state_update(BlockNumber::Hash(("0x04029c604ad1da801b55ef0ff6ac5d72153564efb415e3e45bd27bd4abdb61bd".to_string()))).await;
        assert!(block_hash_result.is_ok());
        println!("state update with block hash: {}",block_hash_result.unwrap());

        let other_result = provider.get_state_update(BlockNumber::BlockTag((BlockTag::pending))).await;
        assert!(other_result.is_ok());
        assert!(other_result.unwrap() == serde_json::Value::Null);
        }

    #[tokio::test]
    async fn test_get_storage_at() {
        let provider = setup_provider();

        let block_number_result = provider.get_storage_at("0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892","0x04",BlockNumber::Number(52668)).await;
        assert!(block_number_result.is_ok());
        println!("storage at : {}",block_number_result.unwrap());

        let block_hash_result = provider.get_storage_at("0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892","0x04",BlockNumber::Hash("0x0334d5edaf94bffa53f00024985e84f9aa0b4ac1601d0d7e8797fdd7e1af95b8".to_string())).await;
        assert!(block_hash_result.is_ok());
        println!("storage using block hash :{}",block_hash_result.unwrap());

        let block_tag_latest_result = provider.get_storage_at("0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892","0x04", BlockNumber::BlockTag(BlockTag::latest)).await;
        assert!(block_tag_latest_result.is_ok());
        println!("storage at latest block :{}",block_tag_latest_result.unwrap());

        let block_tag_latest_result = provider.get_storage_at("0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892","0x04", BlockNumber::BlockTag(BlockTag::pending)).await;
        assert!(block_tag_latest_result.is_ok());
        println!("storage at latest block :{}",block_tag_latest_result.unwrap());
    }

    #[tokio::test]
    async fn test_get_transaction_by_hash() {
        let provider = setup_provider();
        let result = provider.get_transaction_by_hash("0x035475b21b0bc1799053bbf41f191d480e81bdb8eea6874d214dc5cc9882092e".to_string()).await;
        assert!(result.is_ok());
        println!("transaction details {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_get_transaction_by_blockid_and_index() {
        let provider = setup_provider();
        let result = provider.get_transaction_by_blockid_and_index(54980,0).await;
        assert!(result.is_ok());
        println!("transaction details {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_get_transaction_receipt() {
        let provider = setup_provider();
        let result = provider.get_transaction_receipt("0x035475b21b0bc1799053bbf41f191d480e81bdb8eea6874d214dc5cc9882092e".to_string()).await;
        assert!(result.is_ok());
        println!("transaction details {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_get_class() {
        let provider = setup_provider();

        let block_number_result = provider.get_class(BlockNumber::Number(54980),"0x0108a32ec851d37c8f15387dadc87dc80c302c5278b24211ea5b227a4bfdc752").await;
        assert!(block_number_result.is_ok());
        println!("class details {}",block_number_result.unwrap());

        let block_hash_result = provider.get_class(BlockNumber::Hash("0x00cd5cd215737597129af9a0a3e542423d0640d9b0ab4fd6662a78069dac69ef".to_string()),"0x0108a32ec851d37c8f15387dadc87dc80c302c5278b24211ea5b227a4bfdc752").await;
        assert!(block_hash_result.is_ok());
        println!("class details {}",block_hash_result.unwrap());

        let block_tag_latest_result = provider.get_class(BlockNumber::BlockTag(BlockTag::latest),"0x0108a32ec851d37c8f15387dadc87dc80c302c5278b24211ea5b227a4bfdc752").await;
        assert!(block_tag_latest_result.is_ok());
        println!("class details {}",block_tag_latest_result.unwrap());

        let block_tag_pending_result = provider.get_class(BlockNumber::BlockTag(BlockTag::pending),"0x0108a32ec851d37c8f15387dadc87dc80c302c5278b24211ea5b227a4bfdc752").await;
        assert!(block_tag_pending_result.is_ok());
        println!("class details {}",block_tag_pending_result.unwrap());
    }

    #[tokio::test]
    async fn test_get_class_hash_at() {
        let provider = setup_provider();

        let block_number_result = provider.get_class_hash_at(BlockNumber::Number(54980), "0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892").await;
        assert!(block_number_result.is_ok());
        println!("class hash detail using block number {}",block_number_result.unwrap());

        let block_hash_result = provider.get_class_hash_at(BlockNumber::Hash("0x00cd5cd215737597129af9a0a3e542423d0640d9b0ab4fd6662a78069dac69ef".to_string()), "0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892").await;
        assert!(block_hash_result.is_ok());
        println!("class hash details using block hash : {}", block_hash_result.unwrap());

        let pending_class_hash_result = provider.get_class_hash_at(BlockNumber::BlockTag(BlockTag::pending),"0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892" ).await;
        assert!(pending_class_hash_result.is_ok());
        println!("class hash details using pending block : {}", pending_class_hash_result.unwrap());

        let latest_class_hash_result = provider.get_class_hash_at(BlockNumber::BlockTag(BlockTag::latest),"0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892").await;
        assert!(latest_class_hash_result.is_ok());
        println!("class hash details using latest block ; {}", latest_class_hash_result.unwrap());
    }

    #[tokio::test]
    async fn test_get_class_at(){
        let provider = setup_provider();

        let block_number_result = provider.get_class_at(BlockNumber::Number(54980),"0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892").await;
        assert!(block_number_result.is_ok());
        println!("class details using block number  {}",block_number_result.unwrap());

        let block_hash_result = provider.get_class_at(BlockNumber::Hash("0x00cd5cd215737597129af9a0a3e542423d0640d9b0ab4fd6662a78069dac69ef".to_string()), "0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892").await;
        assert!(block_hash_result.is_ok());
        println!("class details using block hash:{}",block_hash_result.unwrap());

        let latest_block_result = provider.get_class_at(BlockNumber::BlockTag((BlockTag::latest)), "0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892").await;
        assert!(latest_block_result.is_ok());
        println!("latest block result :{}",latest_block_result.unwrap());

        let pending_block_result = provider.get_class_at(BlockNumber::BlockTag((BlockTag::pending)), "0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892").await;
        assert!(pending_block_result.is_ok());
        println!("latest block result :{}",pending_block_result.unwrap());

    }

    #[tokio::test]
    async fn test_get_block_transaction_count() {
        let provider = setup_provider();

        let block_number_result = provider.get_block_transaction_count(BlockNumber::Number((54980))).await;
        assert!(block_number_result.is_ok());
        println!("block transaction count {}",block_number_result.unwrap());

        let block_hash_result = provider.get_block_transaction_count(BlockNumber::Hash("0x00cd5cd215737597129af9a0a3e542423d0640d9b0ab4fd6662a78069dac69ef".to_string())).await;
        assert!(block_hash_result.is_ok());
        println!("block hash result : {}",block_hash_result.unwrap());

        let latest_block_result = provider.get_block_transaction_count(BlockNumber::BlockTag(BlockTag::latest)).await;
        assert!(latest_block_result.is_ok());
        println!("pending  block hash result : {}",latest_block_result.unwrap());

        let pending_block_result = provider.get_block_transaction_count(BlockNumber::BlockTag(BlockTag::pending)).await;
        assert!(pending_block_result.is_ok());
        println!("pending  block hash result : {}",pending_block_result.unwrap());
    }

    #[tokio::test]
    async fn test_call() {
        let provider = setup_provider();
        let func_name = "balanceOf";
        let calldatavec: Vec<String> = vec![
            "0x021c500a9e94f6e4fc6fa8fcf44124d86359ab7b0b732884c4cb42bc0a52cd37".to_string()
        ];
        let res = get_selector_from_name(func_name);
        println!("entry point selector is : {}",res);
            let tx: TransactionRequest = TransactionRequest { 
            contract_address:Some("0x029959a546dda754dc823a7b8aa65862c5825faeaaf7938741d8ca6bfdc69e4e".to_string()),
            entry_point_selector:Some(res),
            calldata:calldatavec
        };
        let result = provider.call(tx,90821).await;
        assert!(result.is_ok());
        println!("call res {}",result.unwrap());
    }

    #[tokio::test]
    async fn test_estimate_fee() {
        let provider = setup_provider();
        let common_properties = CommonProperties{
            max_fee: 0,
            version:1,
            signature:vec!["156a781f12e8743bd07e20a4484154fd0baccee95d9ea791c121c916ad44ee0".to_string(),"7228267473c670cbb86a644f8696973db978c51acde19431d3f1f8f100794c6".to_string()],
            nonce:0
        };
        let event_emitter = EventEmitter{
            type_:"sf".to_string(),
            contract_class: "sdf".to_string(),
            sender_address:"dfd".to_string(),
            compiler_class_hash:"fsd".to_string()
        };
        let invoke_transaction = InvokeTransactionV1{
            type_:TypeTx::INVOKE,
            sender_address:"0x5b5e9f6f6fb7d2647d81a8b2c2b99cbc9cc9d03d705576d7061812324dca5c0".to_string(),
            calldata: vec!["0x1".to_string(),
            "0x7394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10".to_string(),
                "0x2f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354".to_string(),
            ("0x0").to_string(),
            ("0x3").to_string(),
            ("0x3").to_string(),
            "0x5b5e9f6f6fb7d2647d81a8b2c2b99cbc9cc9d03d705576d7061812324dca5c0".to_string(),
            "0x3635c9adc5dea00000".to_string(),
            ("0x0").to_string()
            ],
            version:1,
            signature:vec!["0x156a781f12e8743bd07e20a4484154fd0baccee95d9ea791c121c916ad44ee0".to_string(),"0x7228267473c670cbb86a644f8696973db978c51acde19431d3f1f8f100794c6".to_string()],
            nonce:0x0,
            max_fee: 0x0
        };
        let invoke_transactionv0= InvokeTransactionV0{
            contract_address:"0x00057c4b510d66eb1188a7173f31cccee47b9736d40185da8144377b896d5ff3".to_string(),
            entry_point_selector:"0x02d4c8ea4c8fb9f571d1f6f9b7692fff8e5ceaf73b1df98e7da8c1109b39ae9a".to_string(),
            calldata:vec!["0x4767b873669406d25dddbf67356e385a14480979e5358a411955d692576aa30".to_string(),
            "0x1".to_string()]
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

        let typeoftx = TypeOfTx{
            type_ : TypeTx::INVOKE
        };
        
        let transactions = vec![
            // Transaction::EventEmitter(event_emitter,common_properties.clone())
            Transaction::InvokeTransactionV1(invoke_transaction)
            // Transaction::DeclareV1(declare_v1,common_properties.clone()),
            // Transaction::DeployAccountTransactionProperties(deploy_account_tx_properties,common_properties.clone()),
        ];
            
        let result = provider.estimate_fee(transactions,90821).await;
        println!("result index 0 is {}",result[0].as_ref().unwrap());
        assert!(result[0].is_ok());
        // assert!(result[1].is_ok());
        // assert!(result[2].is_ok());
        // assert!(result[3].is_ok());
        // println!("result index 1 is {}",result[1].as_ref().unwrap());
        // println!("resuxwlt index 2 is {}",result[2].as_ref().unwrap());
        // println!("result index 3 is {}",result[3].as_ref().unwrap());
    }

    #[tokio::test]
    async fn test_blockhash_and_number() {
        let provider = setup_provider();
        let result = provider.stark_blockhash_and_number().await;
        assert!(result.is_ok());
        println!("Block hash and number{}",result.unwrap());
    }

}
