use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive)]
pub enum Capability {
    Team = 1,
    Ovs = 2,
}

#[derive(Debug, ToPrimitive)]
pub enum ReloadFlag {
    None = 0,
    Conf = 1,
    DnsRc = 2,
    DnsFull = 4,
    All = 7,
}

#[derive(Debug, FromPrimitive)]
pub enum DeviceInterfaceFlag {
    None = 0,
    Up = 1,
    LowerUp = 2,
    Promisc = 4,
    Carrier = 0x10000,
    LldpClientEnabled = 0x20000,
}

#[derive(Debug, FromPrimitive)]
pub enum ConnectivityState {
    Unknown = 0,
    None = 1,
    Portal = 2,
    Limited = 3,
    Full = 4,
}

#[derive(Debug, FromPrimitive)]
pub enum DeviceType {
    Unknown = 0,
    Ethernet = 1,
    Wifi = 2,
    Unused1 = 3,
    Unused2 = 4,
    Bt = 5,
    OlpcMesh = 6,
    Wimax = 7,
    Modem = 8,
    Infiniband = 9,
    Bond = 10,
    Vlan = 11,
    Adsl = 12,
    Bridge = 13,
    Generic = 14,
    Team = 15,
    Tun = 16,
    IpTunnel = 17,
    Macvlan = 18,
    Vxlan = 19,
    Veth = 20,
    Macsec = 21,
    Dummy = 22,
    Ppp = 23,
    OvsInterface = 24,
    OvsPort = 25,
    OvsBridge = 26,
    Wpan = 27,
    SixLowpan = 28,
    Wireguard = 29,
    WifiP2p = 30,
    Vrf = 31,
    Loopback = 32,
    Hsr = 33,
    IpVlan = 34,
}

#[derive(Debug, FromPrimitive)]
pub enum ActivationStateFlags {
    None = 0,
    IsMaster = 1,
    IsSlave = 2,
    Layer2Ready = 4,
    Ip4Ready = 8,
    Ip6Ready = 16,
    MasterHasSlaves = 32,
    LifetimeBoundToProfileVisibility = 64,
    External = 128,
}

#[derive(Debug, FromPrimitive)]
pub enum ActiveConnectionState {
    Unknown = 0,
    Activating = 1,
    Activated = 2,
    Deactivating = 3,
    Deactivated = 4,
}

// pub enum NMState {
//     NmStateUnknown = 0,
//     NmStateAsleep = 10,
//     NmStateDisconnected = 20,
//     NmStateDisconnecting = 30,
//     NmStateConnecting = 40,
//     NmStateConnectedLocal = 50,
//     NmStateConnectedSite = 60,
//     NmStateConnectedGlobal = 70,
// }

// pub enum NMDeviceCapabilities {
//     NmDeviceCapNone = 0,
//     NmDeviceCapNmSupported = 1,
//     NmDeviceCapCarrierDetect = 2,
//     NmDeviceCapIsSoftware = 4,
//     NmDeviceCapSriov = 8,
// }

// pub enum NMDeviceWifiCapabilities {
//     NmWifiDeviceCapNone = 0,
//     NmWifiDeviceCapCipherWep40 = 1,
//     NmWifiDeviceCapCipherWep104 = 2,
//     NmWifiDeviceCapCipherTkip = 4,
//     NmWifiDeviceCapCipherCcmp = 8,
//     NmWifiDeviceCapWpa = 16,
//     NmWifiDeviceCapRsn = 32,
//     NmWifiDeviceCapAp = 64,
//     NmWifiDeviceCapAdhoc = 128,
//     NmWifiDeviceCapFreqValid = 256,
//     NmWifiDeviceCapFreq2ghz = 512,
//     NmWifiDeviceCapFreq5ghz = 1024,
//     NmWifiDeviceCapMesh = 4096,
//     NmWifiDeviceCapIbssRsn = 8192,
// }

// pub enum NM80211ApFlags {
//     Nm802_11ApFlagsNone = 0,
//     Nm802_11ApFlagsPrivacy = 1,
//     Nm802_11ApFlagsWps = 2,
//     Nm802_11ApFlagsWpsPbc = 4,
//     Nm802_11ApFlagsWpsPin = 8,
// }

// pub enum NM80211ApSecurityFlags {
//     Nm802_11ApSecNone = 0,
//     Nm802_11ApSecPairWep40 = 1,
//     Nm802_11ApSecPairWep104 = 2,
//     Nm802_11ApSecPairTkip = 4,
//     Nm802_11ApSecPairCcmp = 8,
//     Nm802_11ApSecGroupWep40 = 16,
//     Nm802_11ApSecGroupWep104 = 32,
//     Nm802_11ApSecGroupTkip = 64,
//     Nm802_11ApSecGroupCcmp = 128,
//     Nm802_11ApSecKeyMgmtPsk = 256,
//     Nm802_11ApSecKeyMgmt802_1x = 512,
//     Nm802_11ApSecKeyMgmtSae = 1024,
//     Nm802_11ApSecKeyMgmtOwe = 2048,
// }

// pub enum NM80211Mode {
//     Nm802_11ModeUnknown = 0,
//     Nm802_11ModeAdhoc = 1,
//     Nm802_11ModeInfra = 2,
//     Nm802_11ModeAp = 3,
//     Nm802_11ModeMesh = 4,
// }

#[derive(Debug, FromPrimitive)]
pub enum NMBluetoothCapabilities {
    NmBtCapabilityNone = 0,
    NmBtCapabilityDun = 1,
    NmBtCapabilityNap = 2,
}

#[derive(Debug, FromPrimitive)]
pub enum NMDeviceModemCapabilities {
    NmDeviceModemCapabilityNone = 0,
    NmDeviceModemCapabilityPots = 0x1,
    NmDeviceModemCapabilityCdmaEvdo = 0x2,
    NmDeviceModemCapabilityGsmUmts = 0x4,
    NmDeviceModemCapabilityLte = 0x8,
    NmDeviceModemCapability5GNR = 0x40,
}

// pub enum NMWimaxNspNetworkType {
//     NmWimaxNspNetworkTypeUnknown = 0,
//     NmWimaxNspNetworkTypeHome = 1,
//     NmWimaxNspNetworkTypePartner = 2,
//     NmWimaxNspNetworkTypeRoamingPartner = 3,
// }

// pub enum NMDeviceState {
//     NmDeviceStateUnknown = 0,
//     NmDeviceStateUnmanaged = 10,
//     NmDeviceStateUnavailable = 20,
//     NmDeviceStateDisconnected = 30,
//     NmDeviceStatePrepare = 40,
//     NmDeviceStateConfig = 50,
//     NmDeviceStateNeedAuth = 60,
//     NmDeviceStateIpConfig = 70,
//     NmDeviceStateIpCheck = 80,
//     NmDeviceStateSecondaries = 90,
//     NmDeviceStateActivated = 100,
//     NmDeviceStateDeactivating = 110,
//     NmDeviceStateFailed = 120,
// }

// pub enum NMDeviceStateReason {
//     NmDeviceStateReasonNone = 0,
//     NmDeviceStateReasonUnknown = 1,
//     NmDeviceStateReasonNowManaged = 2,
//     NmDeviceStateReasonNowUnmanaged = 3,
//     NmDeviceStateReasonConfigFailed = 4,
//     NmDeviceStateReasonIpConfigUnavailable = 5,
//     NmDeviceStateReasonIpConfigExpired = 6,
//     NmDeviceStateReasonNoSecrets = 7,
//     NmDeviceStateReasonSupplicantDisconnect = 8,
//     NmDeviceStateReasonSupplicantConfigFailed = 9,
//     NmDeviceStateReasonSupplicantFailed = 10,
//     NmDeviceStateReasonSupplicantTimeout = 11,
//     NmDeviceStateReasonPppStartFailed = 12,
//     NmDeviceStateReasonPppDisconnect = 13,
//     NmDeviceStateReasonPppFailed = 14,
//     NmDeviceStateReasonDhcpStartFailed = 15,
//     NmDeviceStateReasonDhcpError = 16,
//     NmDeviceStateReasonDhcpFailed = 17,
//     NmDeviceStateReasonSharedStartFailed = 18,
//     NmDeviceStateReasonSharedFailed = 19,
//     NmDeviceStateReasonAutoipStartFailed = 20,
//     NmDeviceStateReasonAutoipError = 21,
//     NmDeviceStateReasonAutoipFailed = 22,
//     NmDeviceStateReasonModemBusy = 23,
//     NmDeviceStateReasonModemNoDialTone = 24,
//     NmDeviceStateReasonModemNoCarrier = 25,
//     NmDeviceStateReasonModemDialTimeout = 26,
//     NmDeviceStateReasonModemDialFailed = 27,
//     NmDeviceStateReasonModemInitFailed = 28,
//     NmDeviceStateReasonGsmApnFailed = 29,
//     NmDeviceStateReasonGsmRegistrationNotSearching = 30,
//     NmDeviceStateReasonGsmRegistrationDenied = 31,
//     NmDeviceStateReasonGsmRegistrationTimeout = 32,
//     NmDeviceStateReasonGsmRegistrationFailed = 33,
//     NmDeviceStateReasonGsmPinCheckFailed = 34,
//     NmDeviceStateReasonFirmwareMissing = 35,
//     NmDeviceStateReasonRemoved = 36,
//     NmDeviceStateReasonSleeping = 37,
//     NmDeviceStateReasonConnectionRemoved = 38,
//     NmDeviceStateReasonUserRequested = 39,
//     NmDeviceStateReasonCarrier = 40,
//     NmDeviceStateReasonConnectionAssumed = 41,
//     NmDeviceStateReasonSupplicantAvailable = 42,
//     NmDeviceStateReasonModemNotFound = 43,
//     NmDeviceStateReasonBtFailed = 44,
//     NmDeviceStateReasonGsmSimNotInserted = 45,
//     NmDeviceStateReasonGsmSimPinRequired = 46,
//     NmDeviceStateReasonGsmSimPukRequired = 47,
//     NmDeviceStateReasonGsmSimWrong = 48,
//     NmDeviceStateReasonInfinibandMode = 49,
//     NmDeviceStateReasonDependencyFailed = 50,
//     NmDeviceStateReasonBr2684Failed = 51,
//     NmDeviceStateReasonModemManagerUnavailable = 52,
//     NmDeviceStateReasonSsidNotFound = 53,
//     NmDeviceStateReasonSecondaryConnectionFailed = 54,
//     NmDeviceStateReasonDcbFcoeFailed = 55,
//     NmDeviceStateReasonTeamdControlFailed = 56,
//     NmDeviceStateReasonModemFailed = 57,
//     NmDeviceStateReasonModemAvailable = 58,
//     NmDeviceStateReasonSimPinIncorrect = 59,
//     NmDeviceStateReasonNewActivation = 60,
//     NmDeviceStateReasonParentChanged = 61,
//     NmDeviceStateReasonParentManagedChanged = 62,
//     NmDeviceStateReasonOvsdbFailed = 63,
//     NmDeviceStateReasonIpAddressDuplicate = 64,
//     NmDeviceStateReasonIpMethodUnsupported = 65,
//     NmDeviceStateReasonSriovConfigurationFailed = 66,
//     NmDeviceStateReasonPeerNotFound = 67,
// }

// pub enum NMMetered {
//     NmMeteredUnknown = 0,
//     NmMeteredYes = 1,
//     NmMeteredNo = 2,
//     NmMeteredGuessYes = 3,
//     NmMeteredGuessNo = 4,
// }

// // pub enum NMConnectionMultiConnect {
// //     NM_CONNECTION_MULTI_CONNECT_DEFAULT = 0,
// //     NM_CONNECTION_MULTI_CONNECT_SINGLE = 1,
// //     NM_CONNECTION_MULTI_CONNECT_MANUAL_MULTIPLE = 2,
// //     NM_CONNECTION_MULTI_CONNECT_MULTIPLE = 3,
// // }

// // pub enum NMActiveConnectionStateReason {
// //     NM_ACTIVE_CONNECTION_STATE_REASON_UNKNOWN = 0,
// //     NM_ACTIVE_CONNECTION_STATE_REASON_NONE = 1,
// //     NM_ACTIVE_CONNECTION_STATE_REASON_USER_DISCONNECTED = 2,
// //     NM_ACTIVE_CONNECTION_STATE_REASON_DEVICE_DISCONNECTED = 3,
// //     NM_ACTIVE_CONNECTION_STATE_REASON_SERVICE_STOPPED = 4,
// //     NM_ACTIVE_CONNECTION_STATE_REASON_IP_CONFIG_INVALID = 5,
// //     NM_ACTIVE_CONNECTION_STATE_REASON_CONNECT_TIMEOUT = 6,
// //     NM_ACTIVE_CONNECTION_STATE_REASON_SERVICE_START_TIMEOUT = 7,
// //     NM_ACTIVE_CONNECTION_STATE_REASON_SERVICE_START_FAILED = 8,
// //     NM_ACTIVE_CONNECTION_STATE_REASON_NO_SECRETS = 9,
// //     NM_ACTIVE_CONNECTION_STATE_REASON_LOGIN_FAILED = 10,
// //     NM_ACTIVE_CONNECTION_STATE_REASON_CONNECTION_REMOVED = 11,
// //     NM_ACTIVE_CONNECTION_STATE_REASON_DEPENDENCY_FAILED = 12,
// //     NM_ACTIVE_CONNECTION_STATE_REASON_DEVICE_REALIZE_FAILED = 13,
// //     NM_ACTIVE_CONNECTION_STATE_REASON_DEVICE_REMOVED = 14,
// // }

// // pub enum NMSecretAgentGetSecretsFlags {
// //     NM_SECRET_AGENT_GET_SECRETS_FLAG_NONE = 0,
// //     NM_SECRET_AGENT_GET_SECRETS_FLAG_ALLOW_INTERACTION = 1,
// //     NM_SECRET_AGENT_GET_SECRETS_FLAG_REQUEST_NEW = 2,
// //     NM_SECRET_AGENT_GET_SECRETS_FLAG_USER_REQUESTED = 4,
// //     NM_SECRET_AGENT_GET_SECRETS_FLAG_WPS_PBC_ACTIVE = 8,
// //     NM_SECRET_AGENT_GET_SECRETS_FLAG_ONLY_SYSTEM = 2147483648,
// //     NM_SECRET_AGENT_GET_SECRETS_FLAG_NO_ERRORS = 1073741824,
// // }

// // pub enum NMSecretAgentCapabilities {
// //     NM_SECRET_AGENT_CAPABILITY_NONE = 0,
// //     NM_SECRET_AGENT_CAPABILITY_VPN_HINTS = 1,
// // }

// // pub enum NMIPTunnelMode {
// //     NM_IP_TUNNEL_MODE_UNKNOWN = 0,
// //     NM_IP_TUNNEL_MODE_IPIP = 1,
// //     NM_IP_TUNNEL_MODE_GRE = 2,
// //     NM_IP_TUNNEL_MODE_SIT = 3,
// //     NM_IP_TUNNEL_MODE_ISATAP = 4,
// //     NM_IP_TUNNEL_MODE_VTI = 5,
// //     NM_IP_TUNNEL_MODE_IP6IP6 = 6,
// //     NM_IP_TUNNEL_MODE_IPIP6 = 7,
// //     NM_IP_TUNNEL_MODE_IP6GRE = 8,
// //     NM_IP_TUNNEL_MODE_VTI6 = 9,
// //     NM_IP_TUNNEL_MODE_GRETAP = 10,
// //     NM_IP_TUNNEL_MODE_IP6GRETAP = 11,
// // }

// // pub enum NMCheckpointCreateFlags {
// //     NM_CHECKPOINT_CREATE_FLAG_NONE = 0,
// //     NM_CHECKPOINT_CREATE_FLAG_DESTROY_ALL = 1,
// //     NM_CHECKPOINT_CREATE_FLAG_DELETE_NEW_CONNECTIONS = 2,
// //     NM_CHECKPOINT_CREATE_FLAG_DISCONNECT_NEW_DEVICES = 4,
// //     NM_CHECKPOINT_CREATE_FLAG_ALLOW_OVERLAPPING = 8,
// // }

// // pub enum NMRollbackResult {
// //     NM_ROLLBACK_RESULT_OK = 0,
// //     NM_ROLLBACK_RESULT_ERR_NO_DEVICE = 1,
// //     NM_ROLLBACK_RESULT_ERR_DEVICE_UNMANAGED = 2,
// //     NM_ROLLBACK_RESULT_ERR_FAILED = 3,
// // }

// // pub enum NMSettingsConnectionFlags {
// //     NM_SETTINGS_CONNECTION_FLAG_NONE = 0,
// //     NM_SETTINGS_CONNECTION_FLAG_UNSAVED = 1,
// //     NM_SETTINGS_CONNECTION_FLAG_NM_GENERATED = 2,
// //     NM_SETTINGS_CONNECTION_FLAG_VOLATILE = 4,
// // }

// // pub enum NMSettingsAddConnection2Flags {
// //     NM_SETTINGS_ADD_CONNECTION2_FLAG_NONE = 0,
// //     NM_SETTINGS_ADD_CONNECTION2_FLAG_TO_DISK = 1,
// //     NM_SETTINGS_ADD_CONNECTION2_FLAG_IN_MEMORY = 2,
// //     NM_SETTINGS_ADD_CONNECTION2_FLAG_BLOCK_AUTOCONNECT = 32,
// // }

// // pub enum NMSettingsUpdate2Flags {
// //     NM_SETTINGS_UPDATE2_FLAG_NONE = 0,
// //     NM_SETTINGS_UPDATE2_FLAG_TO_DISK = 1,
// //     NM_SETTINGS_UPDATE2_FLAG_IN_MEMORY = 2,
// //     NM_SETTINGS_UPDATE2_FLAG_IN_MEMORY_DETACHED = 4,
// //     NM_SETTINGS_UPDATE2_FLAG_IN_MEMORY_ONLY = 8,
// //     NM_SETTINGS_UPDATE2_FLAG_VOLATILE = 16,
// //     NM_SETTINGS_UPDATE2_FLAG_BLOCK_AUTOCONNECT = 32,
// //     NM_SETTINGS_UPDATE2_FLAG_NO_REAPPLY = 64,
// // }

// // pub enum NMTernary {
// //     NM_TERNARY_DEFAULT = -1,
// //     NM_TERNARY_FALSE = 0,
// //     NM_TERNARY_TRUE = 1,
// // }

// // pub enum NMClientPermission {
// //     NM_CLIENT_PERMISSION_NONE = 0,
// //     NM_CLIENT_PERMISSION_ENABLE_DISABLE_NETWORK = 1,
// //     NM_CLIENT_PERMISSION_ENABLE_DISABLE_WIFI = 2,
// //     NM_CLIENT_PERMISSION_ENABLE_DISABLE_WWAN = 3,
// //     NM_CLIENT_PERMISSION_ENABLE_DISABLE_WIMAX = 4,
// //     NM_CLIENT_PERMISSION_SLEEP_WAKE = 5,
// //     NM_CLIENT_PERMISSION_NETWORK_CONTROL = 6,
// //     NM_CLIENT_PERMISSION_WIFI_SHARE_PROTECTED = 7,
// //     NM_CLIENT_PERMISSION_WIFI_SHARE_OPEN = 8,
// //     NM_CLIENT_PERMISSION_SETTINGS_MODIFY_SYSTEM = 9,
// //     NM_CLIENT_PERMISSION_SETTINGS_MODIFY_OWN = 10,
// //     NM_CLIENT_PERMISSION_SETTINGS_MODIFY_HOSTNAME = 11,
// //     NM_CLIENT_PERMISSION_SETTINGS_MODIFY_GLOBAL_DNS = 12,
// //     NM_CLIENT_PERMISSION_RELOAD = 13,
// //     NM_CLIENT_PERMISSION_CHECKPOINT_ROLLBACK = 14,
// //     NM_CLIENT_PERMISSION_ENABLE_DISABLE_STATISTICS = 15,
// //     NM_CLIENT_PERMISSION_ENABLE_DISABLE_CONNECTIVITY_CHECK = 16,
// //     NM_CLIENT_PERMISSION_WIFI_SCAN = 17,
// // }

// // pub enum NMClientPermissionResult {
// //     NM_CLIENT_PERMISSION_RESULT_UNKNOWN = 0,
// //     NM_CLIENT_PERMISSION_RESULT_YES = 1,
// //     NM_CLIENT_PERMISSION_RESULT_AUTH = 2,
// //     NM_CLIENT_PERMISSION_RESULT_NO = 3,
// // }
