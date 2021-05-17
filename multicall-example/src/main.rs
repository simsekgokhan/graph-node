
use ethers::{
    abi::Abi,
    contract::{Contract, Multicall},
    providers::{Middleware, Http, Provider, PendingTransaction},
    types::{Address, H256, U256},
};

use std::{convert::TryFrom, sync::Arc};

// Full example from ether-rs github:
// 
// async fn bar() -> Result<(), Box<dyn std::error::Error>> {
//     // this is a dummy address used for illustration purpose
//     let address = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".parse::<Address>()?;
//     // (ugly way to write the ABI inline, you can otherwise read it from a file)
//     let abi: Abi = serde_json::from_str(r#"[{"inputs":[{"internalType":"string","name":"value","type":"string"}],"stateMutability":"nonpayable","type":"constructor"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"author","type":"address"},{"indexed":true,"internalType":"address","name":"oldAuthor","type":"address"},{"indexed":false,"internalType":"string","name":"oldValue","type":"string"},{"indexed":false,"internalType":"string","name":"newValue","type":"string"}],"name":"ValueChanged","type":"event"},{"inputs":[],"name":"getValue","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"lastSender","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"string","name":"value","type":"string"}],"name":"setValue","outputs":[],"stateMutability":"nonpayable","type":"function"}]"#)?;
//     // connect to the network
//     let client = Provider::<Http>::try_from("https://kovan.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27")?;
//     // create the contract object. This will be used to construct the calls for multicall
//     let client = Arc::new(client);
//     let contract = Contract::<Provider<Http>>::new(address, abi, Arc::clone(&client));
//     // note that these [`ContractCall`]s are futures, and need to be `.await`ed to resolve.
//     // But we will let `Multicall` to take care of that for us
//     let first_call = contract.method::<_, String>("getValue", ())?;
//     let second_call = contract.method::<_, Address>("lastSender", ())?;
//     // since this example connects to the Kovan testnet, we need not provide an address for
//     // the Multicall contract and we set that to `None`. If you wish to provide the address
//     // for the Multicall contract, you can pass the `Some(multicall_addr)` argument.
//     // Construction of the `Multicall` instance follows the builder pattern
//     let mut multicall = Multicall::new(Arc::clone(&client), None).await?;
//     multicall
//         .add_call(first_call)
//         .add_call(second_call);
//     // `await`ing on the `call` method lets us fetch the return values of both the above calls
//     // in one single RPC call
//     let _return_data: (String, Address) = multicall.call().await?;
//     // the same `Multicall` instance can be re-used to do a different batch of transactions.
//     // Say we wish to broadcast (send) a couple of transactions via the Multicall contract.
//     let first_broadcast = contract.method::<_, H256>("setValue", "some value".to_owned())?;
//     let second_broadcast = contract.method::<_, H256>("setValue", "new value".to_owned())?;
//     let multicall = multicall
//         .clear_calls()
//         .add_call(first_broadcast)
//         .add_call(second_broadcast);
//     // `await`ing the `send` method waits for the transaction to be broadcast, which also
//     // returns the transaction hash
//     let tx_hash = multicall.send().await?;
//     let _tx_receipt = PendingTransaction::new(tx_hash, &client).await?;
//     // you can also query ETH balances of multiple addresses
//     let address_1 = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".parse::<Address>()?;
//     let address_2 = "ffffffffffffffffffffffffffffffffffffffff".parse::<Address>()?;
//     let multicall = multicall
//         .clear_calls()
//         .eth_balance_of(address_1)
//         .eth_balance_of(address_2);
//     let _balances: (U256, U256) = multicall.call().await?;
//     Ok(())
// }

#[tokio::main]
async fn main() {

    // ------ Init client
    let client = Provider::<Http>::try_from(
        "http://10.32.0.218:8545"   // mainnet
        // "https://ropsten.infura.io/v3/8bd21807c528456ba7f7596fb6e5b61a"
        // "https://kovan.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27"
    ).unwrap();    
    let client = Arc::new(client);
    
    // ------ Make multi eth_balance_of calls
    let address_1 = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".parse::<Address>().unwrap();
    let address_2 = "ffffffffffffffffffffffffffffffffffffffff".parse::<Address>().unwrap();    
    let mut multicall : Multicall<Provider<Http>> = Multicall::new(Arc::clone(&client), None).await.unwrap();
    let multicall = multicall
        .clear_calls()
        .eth_balance_of(address_1)
        .eth_balance_of(address_2);

    let balances_result: (U256, U256) = multicall.call().await.unwrap();
    println!("balances_result: {:?}", balances_result);

    // ------ Make multi const fn calls    
    let abi: Abi = serde_json::from_str(r#"[{"constant":true,"inputs":[{"name":"","type":"address"}],"name":"participants","outputs":[{"name":"whitelisted","type":"bool"},{"name":"tokens","type":"uint256"},{"name":"tokensClaimed","type":"bool"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"tokensPerWei","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[],"name":"withdrawFunds","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"addresses","type":"address[]"}],"name":"removeAddressesFromWhitelist","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"icoDeadline","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"tokensSold","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"tokenReward","outputs":[{"name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"amountRaisedInWei","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"addr","type":"address"}],"name":"removeFromWhitelist","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[],"name":"withdrawTokens","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"owner","outputs":[{"name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[],"name":"burnUnsoldTokens","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"toAddress","type":"address"}],"name":"transferUnsoldTokens","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"tokensClaimed","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"tokensClaimableAfter","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"addresses","type":"address[]"}],"name":"addAddressesToWhitelist","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"addr","type":"address"}],"name":"addToWhitelist","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"fundRaiser","outputs":[{"name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"},{"inputs":[{"name":"fundRaiserAccount","type":"address"},{"name":"durationOfIcoInDays","type":"uint256"},{"name":"durationTokensClaimableAfterInDays","type":"uint256"},{"name":"tokensForOneWei","type":"uint256"},{"name":"addressOfToken","type":"address"}],"payable":false,"stateMutability":"nonpayable","type":"constructor"},{"payable":true,"stateMutability":"payable","type":"fallback"},{"anonymous":false,"inputs":[{"indexed":false,"name":"to","type":"address"},{"indexed":false,"name":"amount","type":"uint256"}],"name":"FundTransfer","type":"event"}]"#).unwrap();
    let address = "1773c9fbb20d00ded851b8bc8b8b9b8860c4a9e0".parse::<Address>().unwrap();
    let contract = Contract::<Provider<Http>>::new(address, abi, Arc::clone(&client));
    let first_call = contract.method::<_, Address>("tokenReward", ()).unwrap();    
    let second_call = contract.method::<_, Address>("fundRaiser", ()).unwrap();
    
    let multicall = multicall
         .clear_calls()
         .add_call(first_call)
         .add_call(second_call);

    let multicall_result: (Address, Address) = multicall.call().await.unwrap();
    println!("multicall_result: {:?}", multicall_result);

}