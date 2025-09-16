# cyberdem-rs
Rust implementation of the [Cyber DEM (SISO-REF-072-2024)](https://cdn.ymaws.com/www.sisostandards.org/resource/resmgr/reference_documents_/siso-ref-072-2024.pdf).

This Data Exchange Model or DEM can be used to model cyber-physical systems.

## Installation

To install, type the following command:

`cargo add cyberdem`

## Objects & Events

The library is divided into 2 base items:
- `CyberObject`
- `CyberEvent`

```rust
pub struct CyberObject {
    pub object_id: Uuid,
    pub object: CyberObjectEnum,
    pub name: Option<String>,
    pub description: Option<String>,
    pub related_objects: Vec<RelatedObjectStruct>,
}
```

```rust
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
```

