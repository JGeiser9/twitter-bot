use job_scheduler::{JobScheduler, Job};
use std::time::Duration;
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
    // println!("{}", msg);
    // twitter.tweet(msg);

    let mut sched = JobScheduler::new();

    // Test Job - replace with actual jobs to API endpoints for data
    // Runs every hour - second | minute | hour | day of month | month | day of week | year
    sched.add(Job::new("0 0 0/1 * * *".parse().unwrap(), || {
        println!("===== New Tweet =====");
        println!("{}", msg);
        println!("...waiting on new tweet");
    }));

    loop {
        sched.tick();

        // Put the current thread to sleep for a specified time
        std::thread::sleep(Duration::from_millis(500));

        // @TODO: may need to implement re-running missed / delayed jobs
    }
}
