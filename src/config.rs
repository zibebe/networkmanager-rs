pub enum Config {
    Ip4(Ip4Config),
    Ip6(Ip6Config),
    Dhcp4(Dhcp4Config),
    Dhcp6(Dhcp6Config),
}

pub struct Ip4Config {}
pub struct Ip6Config {}
pub struct Dhcp4Config {}
pub struct Dhcp6Config {}
