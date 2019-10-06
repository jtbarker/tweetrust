// use futures::prelude::*;
// use twitter_stream::Token;

// #[tokio::main]
// fn main() {
//     let token = Token::new("consumer_key", "consumer_secret", "access_key", "access_secret");

//     twitter_stream::Builder::filter(token)
//         .track(Some("@Twitter"))
//         .listen()
//         .unwrap()
//         .try_flatten_stream()
//         .try_for_each(|json| {
//             println!("{}", json);
//             future::ok(())
//         })
//         .await
//         .unwrap();
// }

mod common;

use tokio::runtime::current_thread::block_on_all;
use egg_mode::search::{self, ResultType};
use egg_mode::auth::Token;

use std::io::{stdin, BufRead};

fn main() {

let config = common::Config::load();
println!("Search term:");
let line = stdin().lock().lines().next().unwrap().unwrap();


let con_token = egg_mode::KeyPair::new("consumer key", "consumer secret");


    let search = block_on_all(
        search::search(line)
            .result_type(ResultType::Recent)
            .count(10)
            .call(&config.token),
    )
    .unwrap();

    for tweet in &search.statuses {
        common::print_tweet(tweet);
        println!()
    }
}