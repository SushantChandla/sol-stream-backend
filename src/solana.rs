use std::{str::FromStr, thread};

use solana_client::{pubsub_client, rpc_client::RpcClient};
use solana_sdk::{account::Account, pubkey::Pubkey};

use crate::{establish_connection, models::Stream};

pub fn get_all_program_accounts() -> Vec<(Pubkey, Account)> {
    let program_pub_key = Pubkey::from_str("GoKSo1QVBx1jqeA15xSx6vJm3tYBM1586qp58VxXJayZ")
        .expect("program address invalid");
    let url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(url);

    client
        .get_program_accounts(&program_pub_key)
        .expect("Something went wrong")
}

pub fn subscribe_to_program() {
    let url = "ws://api.devnet.solana.com".to_string();
    let program_pub_key = Pubkey::from_str("GoKSo1QVBx1jqeA15xSx6vJm3tYBM1586qp58VxXJayZ")
        .expect("program address invalid");
    let mut subscription =
        pubsub_client::PubsubClient::program_subscribe(&url, &program_pub_key, None)
            .expect("Something went wrong");

    thread::spawn(move || {
        let conn = establish_connection();
        for socket_data in subscription.1.iter() {
            let pda_pubkey = socket_data.value.pubkey;
            let pda_account: Account = socket_data.value.account.decode().unwrap();
            let stream = Stream::new(pda_pubkey, &pda_account.data);
            match stream {
                Some(a) => Stream::insert_or_update(a, &conn),
                _ => continue,
            };
        }
        subscription.0.shutdown().expect("Something went wrong");
        subscribe_to_program();
    });
}
