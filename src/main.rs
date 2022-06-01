use candid::{Decode, Encode, Principal, CandidType};
use ic_agent::Agent;
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::identity::Secp256k1Identity;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
enum BurnError {
    InsufficientBalance,
    InvalidTokenContract,
    NotSufficientLiquidity,
}

#[derive(CandidType, Deserialize, Debug)]
enum BurnResult {
    Ok(u64),
    Err(BurnError),
}

#[derive(CandidType, Deserialize, Debug)]
struct BurnArgs {
    canister_id: Principal,
    amount: u64,
}

#[tokio::main]
async fn main() {
    // canister id写给哪个 canister充cycle
    let canister_id = Principal::from_text("").expect("wrap principal failed");
    // crt path 写 plug 导出的密钥的绝对路径
    let crt_path = String::from("");
    // amount : 冲多少cycle，如果 1T 就写 1e12
    let amount = 1e12 as u64;
    xtc2cycle(crt_path, canister_id, amount).await
}

pub async fn xtc2cycle(crt_path : String, to: Principal, amount : u64) {
    let url = "https://ic0.app";
    let transport = ReqwestHttpReplicaV2Transport::create(url).unwrap();
    let canister_id = Principal::from_text("aanaa-xaaaa-aaaah-aaeiq-cai").unwrap();
    let identity =
        Secp256k1Identity::from_pem_file(crt_path)
            .unwrap();
    let agent = Agent::builder()
        .with_identity(identity)
        .with_transport(transport)
        .build()
        .unwrap();
    let waiter = garcon::Delay::builder()
        .throttle(std::time::Duration::from_millis(10))
        .timeout(std::time::Duration::from_secs(20))
        .build();
    let res = agent
        .update(&canister_id, "burn")
        .with_arg(
            &Encode!(&BurnArgs {
                    canister_id: to,
                    amount
                })
                .unwrap(),
        )
        .call_and_wait(waiter)
        .await
        .expect("call to canister failed");
    println!("{:#?}", Decode!(&res, BurnResult).unwrap());
}
