use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Activity {
    pub user_id: String,
    pub activity: String,
    pub datetime: String,
}
