use compact_str::CompactString;
use derive_builder::Builder;
use serde::Serialize;

#[derive(Builder, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into))]
pub struct InstanceDetails {
    availability_domain: CompactString,
    compartment_id:      CompactString,
    display_name:        CompactString,
    subnet_id:           CompactString,
    shape:               CompactString,
    shape_config:        ShapeConfig,
    source_details:      SourceDetails,
    create_vnic_details: CreateVnicDetails,
    availability_config: AvailabilityConfig,
    #[serde(rename = "is_pv_encryption_in_transit_enabled")]
    is_pv_enabled:       bool,
    #[builder(default)]
    metadata:            Option<Metadata>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShapeConfig {
    ocpus:         u16,
    memory_in_gbs: u16,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourceDetails {
    source_type: CompactString,
    image_id:    CompactString,
}

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateVnicDetails {
    assign_public_ip:          bool,
    assign_private_dns_record: bool,
    subnet_id:                 CompactString,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityConfig {
    recovery_action: CompactString,
}

#[derive(Debug, Serialize, Clone)]
pub struct Metadata {
    ssh_authorized_keys: CompactString,
}

impl Metadata {
    pub fn new(ssh_key: impl Into<CompactString>) -> Self {
        Self {
            ssh_authorized_keys: ssh_key.into(),
        }
    }
}

impl CreateVnicDetails {
    pub fn new(subnet_id: impl Into<CompactString>) -> Self {
        Self {
            assign_public_ip:          true,
            assign_private_dns_record: true,
            subnet_id:                 subnet_id.into(),
        }
    }
}

impl ShapeConfig {
    #[must_use]
    pub const fn new(ocpus: u16, memory_gb: u16) -> Self {
        Self {
            ocpus,
            memory_in_gbs: memory_gb,
        }
    }
}

impl SourceDetails {
    pub fn image(image_id: impl Into<CompactString>) -> Self {
        Self {
            source_type: CompactString::from("image"),
            image_id:    image_id.into(),
        }
    }
}

impl AvailabilityConfig {
    #[must_use]
    pub fn restore() -> Self {
        Self {
            recovery_action: CompactString::from("RESTORE_INSTANCE"),
        }
    }
}
