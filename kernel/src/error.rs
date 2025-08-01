//! Error

pub enum ACPI {
    InvalidSignature,
    InvalidChecksum,
    InvalidRevision,
    InvalidLength,
    InvalidReserved,
}

pub enum IOAPIC {
    MaxCountReached,
    InvalidGSIIndex,
}

pub enum Memory {
    InvalidAllocationSize,
    OutOfMemory,
    InvalidIndex,
}

pub enum Error {
    ACPI(ACPI),
    IOAPIC(IOAPIC),
    Memory(Memory),
}
