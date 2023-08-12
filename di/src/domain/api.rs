use shaku::Component;
use shaku::Interface;

#[derive(Component)]
#[shaku(interface = IConnector)]
pub struct ApiConnector {
    #[shaku(default)]
    pub target_user: String,
}

pub trait IConnector: Interface {
    fn get_id(&self, user_name: String) -> String;
}
