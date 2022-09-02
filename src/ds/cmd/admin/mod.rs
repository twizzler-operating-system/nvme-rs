pub mod features;

#[repr(u8)]
pub enum AdminCommand {
    DeleteSubmissionQueue,
    CreateSubmissionQueue,
    GetLogPage,
    DeleteCompletionQueue = 0x4,
    CreateCompletionQueue,
    Identify,
    Abort = 0x8,
    SetFeatures,
    GetFeatures,
    AsyncEventRequest = 0xc,
    NamespaceManagement,
    FirmwareCommit = 0x10,
    FirmwareImageDownload,
    DeviceSelfTest = 0x14,
    NamespaceAttachment,
    KeepAlive = 0x18,
    DirectiveSend,
    DirectiveReceive,
    VirtualizationManagement = 0x1c,
    NVMeMISend,
    NVMeMIReceive,
    CapacityManagement = 0x20,
    Lockdown = 0x24,
    DoorbellBufferConfig = 0x7c,
    FabricsCommands = 0x7f,
    FormatNVM,
    SecuritySend,
    SecurityReceive,
    Sanitize = 0x84,
    GetLBAStatus = 0x86,
}

impl From<AdminCommand> for u8 {
    fn from(a: AdminCommand) -> Self {
        a as u8
    }
}
