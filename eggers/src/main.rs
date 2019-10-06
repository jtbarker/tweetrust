extern crate tokio;
extern crate egg_mode;

use tokio::*;
use egg_mode::*;
use tokio_core::reactor::Core;
use tokio::runtime::current_thread::block_on_all;
use egg_mode::tweet::DraftTweet;




fn main() {
    println!("Hello, world!");



    let con_token = egg_mode::KeyPair::new("consumer key", "consumer secret");
    let access_token = egg_mode::KeyPair::new("access token key", "access token secret");
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };
    use tokio::runtime::current_thread::{block_on_all};
    use egg_mode::search::{self, ResultType};

    let post = block_on_all(DraftTweet::new("Hey Twitter!").send(&token)).unwrap();
    let rustlang = block_on_all(egg_mode::user::show("rustlang", &token)).unwrap();

    println!("{} (@{})", rustlang.name, rustlang.screen_name);

    let search = block_on_all(search::search("rustlang")
        .result_type(ResultType::Recent)
        .call(&token))
        .unwrap();

    for tweet in &search.statuses {
        println!("(@{}) {}", tweet.user.as_ref().unwrap().screen_name, tweet.text);
    }

// token can be given to any egg_mode method that asks for a token
// user_id and screen_name refer to the user who signed in

//    use tokio::runtime::current_thread::block_on_all;
//    let con_token = egg_mode::KeyPair::new("consumer key", "consumer secret");
//    let token = block_on_all(egg_mode::bearer_token(&con_token)).unwrap();
//
//    let rustlang = block_on_all(egg_mode::user::show("rustlang", &token)).unwrap();
//
//    println!("{} (@{})", rustlang.name, rustlang.screen_name);

//    // NOTE: this assumes you have a Tokio `core` and its `handle` sitting around already
//
//let con_token = egg_mode::KeyPair::new("consumer key", "consumer secret");
//// "oob" is needed for PIN-based auth; see docs for `request_token` for more info
//let request_token = run(&con_token).unwrap();
//let auth_url = egg_mode::authorize_url(&request_token);
//
//// give auth_url to the user, they can sign in to Twitter and accept your app's permissions.
//// they'll receive a PIN in return, they need to give this to your application
//
//let verifier = "123456"; //read the PIN from the user here
//
//// note this consumes con_token; if you want to sign in multiple accounts, clone it here
//let (token, user_id, screen_name) =
//    tokio::core.run(con_token, &request_token).unwrap();

// token can be given to any egg_mode method that asks for a token
// user_id and screen_name
}
