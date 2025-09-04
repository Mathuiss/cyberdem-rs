pub mod enumerations;
// pub mod filesystem;
// pub mod widgets;

// Object/Event classes for Cyber DEM
// Cyber DEM Rust Translation

// --- Prelude ---
// Assuming the enums from the previous file are in a module named `enumerations`.
// use crate::enumerations::*;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// --- Base Structs (Representing Python Superclasses) ---

/// Base fields for all Cyber DEM Objects and Events.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CyberDemBase {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
}

/// Base fields for persistent objects on a network or other cyber infrastructure.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CyberObject {
    #[serde(flatten)]
    pub base: CyberDemBase,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Base fields for non-persistent cyber events.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CyberEvent {
    #[serde(flatten)]
    pub base: CyberDemBase,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ids: Option<Vec<Uuid>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_modifiers: Option<HashMap<String, String>>, // Assuming string->string mapping

    #[serde(default)] // If the field is missing in JSON, it will default to None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_ids: Option<Vec<Uuid>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids: Option<Vec<Uuid>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Vec<u8>>, // Blob represented as a byte vector

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_acknowledgement: Option<bool>,
}

// --- Top-Level Enum for Deserialization ---

/// Represents any concrete Cyber DEM Object or Event.
/// The `#[serde(tag = "_type")]` attribute replicates the Python factory's logic.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "_type")]
pub enum CyberDemEntity {
    // --- Objects ---
    Application(Application),
    CommunicationsData(CommunicationsData),
    Data(Data),
    Device(Device),
    Network(Network),
    NetworkLink(NetworkLink),
    OperatingSystem(OperatingSystem),
    Persona(Persona),
    Service(Service),
    System(System),

    // --- Events, Actions, and Effects ---
    BlockTrafficEffect(BlockTrafficEffect),
    CPULoadEffect(CPULoadEffect),
    CyberAcknowledge(CyberAcknowledge),
    CyberAdmin(CyberAdmin),
    CyberAttack(CyberAttack),
    CyberDefend(CyberDefend),
    CyberOrder(CyberOrder),
    CyberRecon(CyberRecon),
    DataExfiltration(DataExfiltration),
    Degrade(Degrade),
    DelayEffect(DelayEffect),
    Deny(Deny),
    Destroy(Destroy),
    Detect(Detect),
    Disrupt(Disrupt),
    DropEffect(DropEffect),
    HardwareDamageEffect(HardwareDamageEffect),
    HardwareDegradeEffect(HardwareDegradeEffect),
    JitterEffect(JitterEffect),
    LoadRateEffect(LoadRateEffect),
    Manipulate(Manipulate),
    ManipulationAttack(ManipulationAttack),
    ManipulationEffect(ManipulationEffect),
    MemoryUseEffect(MemoryUseEffect),
    OtherDegradeEffect(OtherDegradeEffect),
    PhishingAttack(PhishingAttack),
}

// --- Concrete Object Structs ---

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Application {
    #[serde(flatten)]
    pub object: CyberObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Data {
    #[serde(flatten)]
    pub object: CyberObject,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub sensitivity: Option<SensitivityType>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub data_type: Option<DataType>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub encrypted: Option<EncryptionType>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub status: Option<DataStatusType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidentiality: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CommunicationsData {
    #[serde(flatten)]
    pub data: Data,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_size_standard_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplex: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Device {
    #[serde(flatten)]
    pub object: CyberObject,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub device_types: Option<Vec<DeviceType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_virtual: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<(String, String)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Network {
    #[serde(flatten)]
    pub object: CyberObject,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub protocol: Option<NetworkProtocolType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NetworkLink {
    #[serde(flatten)]
    pub object: CyberObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_logical: Option<bool>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub physical_layer: Option<PhysicalLayerType>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub data_link_protocol: Option<DataLinkProtocolType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jitter: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<(String, String)>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OperatingSystem {
    #[serde(flatten)]
    pub application: Application,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub os_type: Option<OperatingSystemType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Persona {
    #[serde(flatten)]
    pub object: CyberObject,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Service {
    #[serde(flatten)]
    pub application: Application,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub service_type: Option<ServiceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct System {
    #[serde(flatten)]
    pub object: CyberObject,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub system_type: Option<SystemType>,
}

// --- Concrete Event/Action/Effect Structs ---

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CyberAcknowledge {
    #[serde(flatten)]
    pub event: CyberEvent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_event_id: Option<Uuid>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub acknowledge_response: Option<AcknowledgeResponseType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CyberAdmin {
    #[serde(flatten)]
    pub event: CyberEvent, // Based on _CyberAction -> _CyberEvent
                           // #[serde(skip_serializing_if = "Option::is_none")]
                           // pub admin_type: Option<AdminType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CyberAttack {
    #[serde(flatten)]
    pub event: CyberEvent, // Based on _CyberAction -> _CyberEvent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitre_attack_subtechnique_ids: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CyberDefend {
    #[serde(flatten)]
    pub event: CyberEvent, // Based on _CyberAction -> _CyberEvent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitre_attack_mitigation_ids: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CyberOrder {
    #[serde(flatten)]
    pub event: CyberEvent,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CyberRecon {
    #[serde(flatten)]
    pub event: CyberEvent, // Based on _CyberAction -> _CyberEvent
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub recon_type: Option<ReconType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitre_attack_detection_ids: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DataExfiltration {
    #[serde(flatten)]
    pub attack: CyberAttack,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Deny {
    #[serde(flatten)]
    pub event: CyberEvent, // Based on _CyberEffect -> _CyberEvent
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Destroy {
    #[serde(flatten)]
    pub deny: Deny,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HardwareDamageEffect {
    #[serde(flatten)]
    pub destroy: Destroy,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub damage_type: Option<HardwareDamageType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Degrade {
    #[serde(flatten)]
    pub deny: Deny,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_random: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CPULoadEffect {
    #[serde(flatten)]
    pub degrade: Degrade,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DelayEffect {
    #[serde(flatten)]
    pub degrade: Degrade,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DropEffect {
    #[serde(flatten)]
    pub degrade: Degrade,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HardwareDegradeEffect {
    #[serde(flatten)]
    pub degrade: Degrade,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub degrade_type: Option<HardwareDegradeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JitterEffect {
    #[serde(flatten)]
    pub degrade: Degrade,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milliseconds: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LoadRateEffect {
    #[serde(flatten)]
    pub degrade: Degrade,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub rate_type: Option<LoadRateType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MemoryUseEffect {
    #[serde(flatten)]
    pub degrade: Degrade,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OtherDegradeEffect {
    #[serde(flatten)]
    pub degrade: Degrade,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Disrupt {
    #[serde(flatten)]
    pub deny: Deny,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BlockTrafficEffect {
    #[serde(flatten)]
    pub disrupt: Disrupt,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Detect {
    #[serde(flatten)]
    pub event: CyberEvent, // Based on _CyberEffect -> _CyberEvent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquired_information: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Manipulate {
    #[serde(flatten)]
    pub event: CyberEvent, // Based on _CyberEffect -> _CyberEvent
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ManipulationEffect {
    #[serde(flatten)]
    pub manipulate: Manipulate,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub manipulation_type: Option<ManipulationType>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub packet_manipulation_type: Option<PacketManipulationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ManipulationAttack {
    #[serde(flatten)]
    pub attack: CyberAttack,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PhishingAttack {
    #[serde(flatten)]
    pub attack: CyberAttack,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub message_type: Option<MessageType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

// --- Standalone Relationship Struct ---

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Relationship {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub related_object_1: Uuid,
    pub related_object_2: Uuid,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub relationship_type: Option<RelationshipType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileges: Option<Vec<String>>,
}
