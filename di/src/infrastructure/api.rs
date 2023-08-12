use crate::domain::api::{ApiConnector, IConnector};

impl ApiConnector {}

impl IConnector for ApiConnector {
    fn get_id(&self, user_name: String) -> String {
        if user_name == "john" {
            return "0002".to_owned();
        }
        "0001".to_owned()
    }
}
