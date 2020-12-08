use web3::Web3;
use std::env;
use web3::transports::Http;
use dotenv::dotenv;

pub fn init_web3() -> Web3<Http> {
    dotenv().ok();

    let http_address = env::var("WEB3_URL")
        .expect("WEB3_URL must be set");
    let transport = web3::transports::Http::new(&http_address);
    let web3 = web3::Web3::new(transport.unwrap());
    return web3
}