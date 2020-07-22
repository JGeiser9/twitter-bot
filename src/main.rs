use std::fs;

mod config;
mod twitter;

const CONF_FILENAME: &'static str = ".twitter-bot.conf";

// @TODO: replace this with a data from HNScan endpoints
pub fn build_message () -> String {
    let message = String::from("[bot]: testing a twitter bot built in Rust");
    
    message
}

fn main() {
    let path = match fs::canonicalize(CONF_FILENAME) {
        Ok(_p) => _p,
        Err(_err) => panic!("Could not find the .conf file containing twitter keys")
    };

    let config = match config::Config::read(&path) {
        Some(v) => v,
        None => panic!("Could not load the configuration file"),
    };

    let twitter = twitter::Twitter::new(
        config.oauth_consumer_key, 
        config.oauth_consumer_secret, 
        config.oauth_token, 
        config.oauth_token_secret
    );

    let msg = build_message();
    println!("{}", msg);
    // twitter.tweet(msg);

    println!("Tweet Sent! Check Your Twitter");
}
