#[repr(u8)]
pub enum AdminCommand {
    DeleteSubmissionQueue,
    CreateSubmissionQueue,
    GetLogPage,
    DeleteCompletionQueue,
    CreateCompletionQueue,
    Identify,
    Abort,
    SetFeatures,
    GetFeatures,
    AsyncEventRequest,
    NamespaceManagement,
    FirmwareCommit,
    FirmwareImageDownload,
    DeviceSelfTest,
    NamespaceAttachment,
    KeepAlive,
    DirectiveSend,
    DirectiveReceive,
    VirtualizationManagement,
    NVMeMISend,
    NVMeMIReceive,
    CapacityManagement,
    Lockdown,
    DoorbellBufferConfig,
    FabricsCommands,
    FormanNVM,
    SecuritySend,
    SecurityReceive,
    Sanitize,
    GetLBAStatus,
}

impl From<AdminCommand> for u8 {
    fn from(a: AdminCommand) -> Self {
        a as u8
    }
}
