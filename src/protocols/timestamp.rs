//! EFI Timestamp Protocol
//!
//! The Timestamp protocol provides a platform independent interface for
//! retrieving a high resolution timestamp counter.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::new(
    0xafbfde41_u32.to_ne_bytes(),
    0x2e6e_u16.to_ne_bytes(),
    0x4262_u16.to_ne_bytes(),
    0xba,
    0x65,
    [0x62, 0xb9, 0x23, 0x6e, 0x54, 0x95],
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Properties {
    pub frequency: u64,
    pub end_value: u64,
}

pub type ProtocolGetTimestamp = eficall! {fn() -> u64};

pub type ProtocolGetProperties = eficall! {fn(
    *mut Properties,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub get_timestamp: ProtocolGetTimestamp,
    pub get_properties: ProtocolGetProperties,
}
