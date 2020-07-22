use oauth_client::Token;
use twitter_api;

pub struct Twitter<'a> {
    consumer_key : Token<'a>,
    auth_token : Token<'a>,
}

impl<'a> Twitter<'a>{
    pub fn new (
        oauth_consumer_key : String,
        oauth_consumer_secret : String,
        oauth_token : String, 
        oauth_token_secret : String
    ) -> Self {
        Twitter {
            consumer_key : Token::new(oauth_consumer_key, oauth_consumer_secret),
            auth_token : Token::new(oauth_token, oauth_token_secret)
        }
    }

    pub fn tweet (&self, message : String)  {
        twitter_api::update_status(
            &self.consumer_key, 
            &self.auth_token, 
            &message
        ).unwrap();
    }
}