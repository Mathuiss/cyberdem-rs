use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TimeStruct {
    pub time: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CyberEventPhaseEnum {
    Start,
    Suspend,
    Continue,
    ContinueWithChanges,
    End,
    Cancel,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AcknowledgeResponseEnum {
    AbleToComply,
    UnableToComply,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CyberAcknowledge {
    pub related_event_id: Uuid,
    pub acknowledge_response: AcknowledgeResponseEnum,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BlockTrafficEffect {
    // Will probably contain values in future versions of the SISO CyberDEM
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum HardwareDamageTypeEnum {
    BootLoop,
    PhysicalDestruction,
    HardDriveErased,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum LoadRateTypeEnum {
    Upload,
    Download,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum HardwareDegradeTypeEnum {
    Keyboard,
    Mouse,
    Display,
    Sound,
    KernelPanic,
    RandomText,
    Reboot,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DegradeEffectType {
    LoadRateEffect(LoadRateTypeEnum),
    DelayEffect,
    JitterEffect,
    CPULoadEffect,
    MemoryUseEffect,
    DropEffect,
    HardwareDegradeEffect(HardwareDegradeTypeEnum),
    OtherDegradeEffect,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DegradeEffect {
    pub is_random: Option<bool>,
    pub percentage: Option<f64>,
    pub degrade_effect_type: DegradeEffectType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DenyEffect {
    Disrupt(BlockTrafficEffect),
    Destroy(HardwareDamageTypeEnum),
    Degrade(DegradeEffect),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ManipulationTypeEnum {
    Packet,
    File,
    Database,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PacketManipulationTypeEnum {
    Duplication,
    Corruption,
    Reordering,
    Dropped,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ManipulationEffect {
    pub manipulation_type: Option<ManipulationTypeEnum>,
    pub packet_manipulation_type: Option<PacketManipulationTypeEnum>,
    pub percentage: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CyberEffect {
    Deny(DenyEffect),
    Detect(HashMap<String, Option<Vec<u8>>>),
    Manipulate(ManipulationEffect),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MitreId {
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MessageTypeEnum {
    Email,
    Chat,
    Text,
    SocialMedia,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PhishingAttack {
    pub message_type: Option<MessageTypeEnum>,
    pub header: Option<String>,
    pub body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AttackTypeEnum {
    PhishingAttack(PhishingAttack),
    ManipulationAttack,
    DataExfiltration,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CyberAttack {
    pub mitre_sub_technique_ids: Vec<MitreId>,
    pub cyber_attack_type: AttackTypeEnum,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ReconTypeEnum {
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
    DomainTructDiscovery,
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
    SoftwareDiscovery,
    SYNScan,
    SystemInformationDiscovery,
    SystemNetworkConfiguration,
    SystemNetworkConnections,
    SystemOwnerUserDiscovery,
    SystemServiceDiscovery,
    SystemTimeDiscovery,
    TCPConnect,
    TraceRoute,
    UnitLinux,
    VirtualizationSandboxEvasion,
    Vulnerability,
    WebCrawler,
    Windows,
    WirelessActive,
    WirelessPassive,
    XMASScan,
    XSS, // Should be XSSScan - but XSS according to the SISO document
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CyberRecon {
    pub recon_type: Option<ReconTypeEnum>,
    pub mitre_detection_ids: Vec<MitreId>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AdminTypeEnum {
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CyberAction {
    CyberAttack(CyberAttack),
    CyberDefend(Vec<MitreId>),
    CyberRecon(CyberRecon),
    CyberAdmin(AdminTypeEnum),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CyberEventType {
    CyberAcknowledge(CyberAcknowledge),
    CyberEffect(CyberEffect),
    CyberAction(CyberAction),
    CyberOrder,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CyberEvent {
    pub event_id: Uuid,
    pub description: Option<String>,
    pub event_time: TimeStruct,
    pub target_ids: Vec<Uuid>,
    pub target_modifiers: HashMap<String, Option<Vec<u8>>>,
    pub phase: CyberEventPhaseEnum,
    pub duration: f64,
    pub actor_ids: Vec<Uuid>,
    pub source_ids: Vec<Uuid>,
    pub payload: Option<Vec<u8>>,
    pub request_acknowledgement: bool,
    pub cyber_event_type: CyberEventType,
}
