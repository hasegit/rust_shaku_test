use shaku::Interface;
use shaku::Component;


pub trait UserRepository: Interface {
    fn find_user(&self, id:String);
}

#[derive(Component)]
#[shaku(interface = Connector)]
pub struct ApiConnector {
    #[shaku(default)]
    pub target_date: String
}

pub trait Connector: Interface {
    fn get(&self);
}
