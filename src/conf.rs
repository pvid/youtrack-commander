use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub youtrack_url: String,
    pub auth_token: String,
}
