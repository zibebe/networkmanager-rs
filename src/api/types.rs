use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive)]
pub enum Capability {
    Team = 1,
    Ovs = 2,
}

pub enum State {
    Unknown = 0,
    Asleep = 10,
    Disconnected = 20,
    Disconnecting = 30,
    Connecting = 40,
    ConnectedLocal = 50,
    ConnectedSite = 60,
    ConnectedGlobal = 70,
}

pub enum ReloadFlag {
    None = 0,
    Conf = 0x1,
    DnsRc = 0x2,
    DnsFull = 0x4,
    All = 0x7,
}

#[derive(Debug, FromPrimitive)]
pub enum DeviceType {
    Unknown,
    Ethernet,
    WiFi,
    Unused1,
    Unused2,
    Bt,
    OlcpMesh,
    WiMax,
    Modem,
    Infiniband,
    Bond,
    Vlan,
    Adsl,
    Bridge,
    Generic,
    Team,
    TunTap,
    IpTunnel,
    MacVlan,
    VxLan,
    Veth,
    Macsec,
    Dummy,
    PPP,
    OvsInterface,
    OvsPort,
    OvsBridge,
    Wpan,
    SixLoWpan,
    WireGuard,
    WiFiP2p,
    Vrf,
}
