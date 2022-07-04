use candid::{CandidType, Decode, Encode, Principal};
use clap::{App, Arg};
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::identity::Secp256k1Identity;
use ic_agent::Agent;
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
    let matches = App::new("XTC to Cycle App")
        .version("0.1.0")
        .author("Elie@MixLabs")
        .about("Help Dev to use XTC")
        .arg(
            Arg::with_name("path")
                .short('p')
                .long("path")
                .takes_value(true)
                .help("private key path"),
        )
        .arg(
            Arg::with_name("canister_id")
                .short('c')
                .long("canister")
                .takes_value(true)
                .help("top up to the canister specified by canister's principal"),
        )
        .arg(
            Arg::with_name("cycle_amount")
                .short('a')
                .long("amount")
                .takes_value(true)
                .help("cycle amount, unit : T. If you input 1, the cycle amount will be 1_000_000_000_000"),
        )
        .get_matches();

    let canister_id = Principal::from_text(
        matches
            .value_of("canister_id")
            .expect("please specify the canister's principal"),
    )
    .expect("wrap principal failed");

    let crt_path = String::from(
        matches
            .value_of("path")
            .expect("please specify the path of your plug crt file or pem file"),
    );

    // @param amount : cycle amount, 1 T = 1e12
    let amount = 1e12
        * matches
            .value_of("cycle_amount")
            .expect("please specify how much xtc should be consumed")
            .parse::<f64>()
            .expect("please type in a Integer");
    println!(
        "top up Info: \n\tcanister_id : {} \n\tcycle amount: {}",
        canister_id, amount
    );
    xtc2cycle(crt_path, canister_id, amount as u64).await
}

pub async fn xtc2cycle(crt_path: String, to: Principal, amount: u64) {
    let url = "https://ic0.app";
    let transport = ReqwestHttpReplicaV2Transport::create(url).unwrap();
    let canister_id = Principal::from_text("aanaa-xaaaa-aaaah-aaeiq-cai").unwrap();
    let identity = Secp256k1Identity::from_pem_file(crt_path).unwrap();
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
