use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NetworkProtocolEnum {
    InternetProtocol,
    NAT,
    ICMP,
    ARP,
    RIP,
    OSPF,
    IPsec,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Network {
    pub protocol: NetworkProtocolEnum,
    pub mask: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DataLinkProtocolEnum {
    Ethernet,
    WiFi,
    ATM,
    LocalTalk,
    PPP,
    TokenRing,
    VLAN,
    Bluetooth,
    Bus1553,
    LLC,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PhysicalLayerEnum {
    Wired,
    Wireless,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NetworkInterfaceStruct {
    pub related_object_id: Uuid,
    pub name: Option<String>,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NetworkLink {
    pub data_link_protocol: DataLinkProtocolEnum,
    pub bandwidth: Option<i64>,
    pub latency: Option<i64>,
    pub jitter: Option<i64>,
    pub physical_layer: PhysicalLayerEnum,
    pub network_interfaces: NetworkInterfaceStruct,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DeviceTypeEnum {
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Device {
    pub device_types: Vec<DeviceTypeEnum>,
    pub is_virtual: Option<bool>,
    pub is_role: Option<String>,
    pub device_identifier: Option<String>,
    pub network_interfaces: Vec<NetworkInterfaceStruct>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ServiceTypeEnum {
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Service {
    pub service_type: ServiceTypeEnum,
    pub address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum OperatingSystemEnum {
    MicrosoftDOS,
    MicrosoftWindows,
    AppleMacOS,
    DECVMS,
    IBMOS2,
    Android,
    AppleiOS,
    CiscoIOS,
    Firmware,
    UnixLinux,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Application {
    pub version: Option<String>,
    pub company: Option<String>,
    pub service: Service,
    pub os_type: OperatingSystemEnum,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SensitivityEnum {
    Unclassified,
    Confidential,
    FOUO,
    Secret,
    SecretNotForn,
    TS,
    TSSO,
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
    CUI,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DataTypeEnum {
    File,
    Code,
    Credentials,
    Communications,
    SystemConfiguration,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EncryptionTypeEnum {
    NotEncrypted,
    DES,
    TripleDES,
    RSA,
    AES,
    TwoFish,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DataStatusEnum {
    Intact,
    Compromised,
    Corrupted,
    Manipulated,
    NonDecryptable,
    Erased,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CommunicationsData {
    pub packet_size: Option<i64>,
    pub packet_standard_deviation: Option<f64>,
    pub frequency: Option<i64>,
    pub is_duplex: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Data {
    pub sensitivity: SensitivityEnum,
    pub data_type: DataTypeEnum,
    pub encrypred: EncryptionTypeEnum,
    pub status: DataStatusEnum,
    pub confidentiality: Option<f64>,
    pub communications_data: Option<CommunicationsData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SystemTypeEnum {
    Generic,
    SCADA,
    C2,
    ICS,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CyberObjectEnum {
    Network(Network),
    NetworkLink(NetworkLink),
    Device(Device),
    Applicaion(Application),
    Persona,
    Data(Data),
    System(SystemTypeEnum),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RelationshipTypeEnum {
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RelatedObjectStruct {
    pub related_object_id: Uuid,
    pub relationship_type: RelationshipTypeEnum,
    pub related_object_privileges: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CyberObject {
    pub object_id: Uuid,
    pub object: CyberObjectEnum,
    pub name: Option<String>,
    pub description: Option<String>,
    pub related_objects: Vec<RelatedObjectStruct>,
}
