// CyberDEM Enumerations Module
// CyberDEM Rust Translation

/// CyberDEM AcknowledgeResponseType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AcknowledgeResponseType {
    AbleToComply,
    UnableToComply,
}

/// CyberDEM AdminType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AdminType {
    Administration,
    Assessment,
    Collection,
    Configuration,
    Evaluation,
    Forensics,
    Investigation,
    Operations,
    Provisioning,
    Testing,
}

/// CyberDEM CyberEventPhaseType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CyberEventPhaseType {
    Start,
    Suspend,
    Continue,
    ContinueWithChanges,
    End,
    Cancel,
}

/// CyberDEM DataLinkProtocolType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataLinkProtocolType {
    Ethernet,
    WiFi,
    ATM,
    LocalTalk,
    PPP,
    TokenRing,
    VLAN,
    Bluetooth,
    #[allow(non_camel_case_types)]
    _1553Bus,
}

/// CyberDEM DataStatusType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataStatusType {
    Intact,
    Compromised,
    Corrupted,
    Manipulated,
    NonDecryptable,
    Erased,
}

/// CyberDEM DataType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataType {
    File,
    Code,
    Credentials,
    Communications,
    SystemConfiguration,
}

/// CyberDEM DeviceType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DeviceType {
    Generic,
    Networking,
    ComputerNode,
    PortableComputer,
    Controller,
    Storage,
    Sensor,
    Printer,
    Scanner,
    Communications,
    HMI,
    Monitoring,
    IoT,
    Security,
}

/// CyberDEM EncryptionType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EncryptionType {
    NotEncrypted,
    DES,
    TripleDES,
    RSA,
    AES,
    TwoFish,
    SHA,
}

/// CyberDEM HardwareDamageType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HardwareDamageType {
    BootLoop,
    PhysicalDestruction,
    HardDriveErased,
}

/// CyberDEM HardwareDegradeType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HardwareDegradeType {
    Keyboard,
    Mouse,
    Display,
    Sound,
}

/// CyberDEM LoadRateType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LoadRateType {
    Upload,
    Download,
}

/// CyberDEM ManipulationType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ManipulationType {
    Packet,
    File,
    Database,
}

/// CyberDEM MessageType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageType {
    Email,
    Chat,
    Text,
    SocialMedia,
}

/// CyberDEM NetworkProtocolType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NetworkProtocolType {
    InternetProtocol,
    NAT,
    ICMP,
    ARP,
    RIP,
    OSPF,
    IPsec,
}

/// CyberDEM OperatingSystemType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OperatingSystemType {
    MicrosoftDOS,
    MicrosoftWindows,
    AppleMacOS,
    DECVMS,
    #[allow(non_camel_case_types)]
    IBMOS_2,
    Android,
    AppleiOS,
    CiscoIOS,
    Firmware,
    #[allow(non_camel_case_types)]
    UNIX_Linux,
}

/// CyberDEM PacketManipulationType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PacketManipulationType {
    Duplication,
    Corruption,
    Redordering,
    Dropped,
}

/// CyberDEM PhysicalLayerType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PhysicalLayerType {
    Wired,
    Wireless,
}

/// CyberDEM ReconType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReconType {
    AccountDiscovery,
    AdMalware,
    AntivirusTrojan,
    ApplicationWindowDiscovery,
    ARPScan,
    BannerGrabbing,
    BounceScan,
    BrowserBookmarkDiscovery,
    CloudInfrastructureDiscovery,
    CloudServiceDashboard,
    CloudServiceDiscovery,
    Compliance,
    CSRF,
    DatabaseInjection,
    DatabaseStructure,
    DBManufactureVersion,
    Device,
    DNS,
    Domain,
    DomainTrustDiscovery,
    FileAndDirectoryDiscovery,
    FINScan,
    FTP,
    HTTP,
    IdleScan,
    IGMP,
    InputValidation,
    IP,
    LDAPScan,
    NetBiosScan,
    NetworkMap,
    NetworkServiceScanning,
    NetworkShareDiscovery,
    NetworkSniffing,
    NTP,
    NULLScan,
    OSScan,
    PasswordPolicyDiscovery,
    PatchHistory,
    PeripheralDeviceDiscovery,
    PermissionGroupsDiscovery,
    Ping,
    PingScan,
    PortScan,
    PortSweep,
    PPP,
    ProcessDiscovery,
    QueryRegistry,
    RARP,
    RemoteSystemDiscovery,
    Rootkit,
    RPCScan,
    Service,
    SLIP,
    SMTP,
    SNMPSweep,
    SoftwareDiscover,
    SYNScan,
    SystemlnformationDiscovery,
    SystemNetworkConfigurationDiscovery,
    SystemNetworkConnectionsDiscovery,
    SystemOwnerUserDiscovery,
    SystemServiceDiscovery,
    SystemTimeDiscovery,
    TCPConnect,
    TraceRoute,
    #[allow(non_camel_case_types)]
    UNIX_Linux,
    VirtualizationSandboxEvasion,
    Vulnerability,
    WebCrawler,
    Windows,
    WirelessActive,
    WirelessPassive,
    XMASScan,
    XSS,
}

/// CyberDEM RelationshipType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RelationshipType {
    Administers,
    AdministeredBy,
    ComponentOf,
    HasComponent,
    ContainedIn,
    Contains,
    ProvidedBy,
    Provides,
    ResidesOn,
    HasResident,
}

/// CyberDEM SensitivityType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SensitivityType {
    Unclassified,
    Confidential,
    FOUO,
    Secret,
    SecretNoForn,
    TS,
    #[allow(non_camel_case_types)]
    TS_SCI,
    NATORestricted,
    NATOConfidential,
    NATOSecret,
    CosmicTopSecret,
    FVEYProprietary,
    Proprietary,
    PII,
    HIPAA,
    GDPR,
    Public,
}

/// CyberDEM ServiceType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ServiceType {
    DNS,
    Email,
    Web,
    Database,
    File,
    Chat,
    Forum,
    SocialMedia,
    Containerization,
    Virtualization,
    NetworkTime,
}

/// CyberDEM SystemType enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SystemType {
    Generic,
    SCADA,
    C2,
    ICS,
}
