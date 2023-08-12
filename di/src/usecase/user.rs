use crate::domain::{api::IConnector, storage::IUserRepository};
use shaku::Component;
use shaku::Interface;
use std::sync::Arc;

#[derive(Component)]
#[shaku(interface = IUserUpdater)]
pub struct UserUpdater {
    #[shaku(inject)]
    api: Arc<dyn IConnector>,
    #[shaku(inject)]
    repository: Arc<dyn IUserRepository>,
}

pub trait IUserUpdater: Interface {
    fn update(&self, user_name: String);
}

impl IUserUpdater for UserUpdater {
    fn update(&self, user_name: String) {
        let user = self.api.get_id(user_name);

        self.repository.update(user);
    }
}
