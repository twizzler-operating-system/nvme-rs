use crate::ds::{
    cmd::admin::AdminCommand,
    controller::ControllerId,
    namespace::NamespaceId,
    queue::{
        subentry::{CommandDword0, CommonCommand, Dptr, FuseSpec},
        CommandId,
    },
    uuid::UuidIndex,
};

use modular_bitfield::prelude::*;
#[bitfield(bits = 32)]
#[repr(u32)]
struct IdentifyDword10 {
    cntid: ControllerId,
    #[skip]
    res: B8,
    cns: B8,
}

#[bitfield(bits = 32)]
#[repr(u32)]
struct IdentifyDword11 {
    cns_specific_id: u16,
    #[skip]
    res: B8,
    csi: u8,
}

#[bitfield(bits = 32)]
#[repr(u32)]
struct IdentifyDword14 {
    uuid_idx: UuidIndex,
    #[skip]
    res: B25,
}

struct Identify {
    dw10: IdentifyDword10,
    dw11: IdentifyDword11,
    dw14: IdentifyDword14,
    cdw0: CommandDword0,
    dptr: Dptr,
    nsid: NamespaceId,
}

impl From<Identify> for CommonCommand {
    fn from(i: Identify) -> Self {
        Self::new()
            .with_cdw0(i.cdw0)
            .with_cdw10(i.dw10.into())
            .with_cdw11(i.dw11.into())
            .with_cdw14(i.dw14.into())
            .with_dptr(i.dptr)
            .with_nsid(i.nsid)
    }
}

impl Identify {
    fn new(
        cid: CommandId,
        cns: IdentifyCNSValue,
        dptr: Dptr,
        uuid_index: Option<UuidIndex>,
    ) -> Self {
        Self {
            dw10: IdentifyDword10::new()
                .with_cns(cns.cns_value())
                .with_cntid(cns.cntid_value().into()),
            dw11: IdentifyDword11::new()
                .with_csi(cns.csi_value())
                .with_cns_specific_id(cns.specific_id_value()),
            dw14: IdentifyDword14::new().with_uuid_idx(uuid_index.into()),
            cdw0: CommandDword0::build(
                AdminCommand::Identify.into(),
                cid,
                FuseSpec::Normal,
                dptr.psdt(false),
            ),
            dptr,
            nsid: cns.nsid().unwrap_or(NamespaceId::default()),
        }
    }
}

#[derive(BitfieldSpecifier, Clone, Copy)]
#[bits = 8]
enum CommandSetIdentifier {
    NVM,
    KeyValue,
    Zoned,
}

enum IdentifyCNSValue {
    IdentifyNamespace(NamespaceId),
    IdentifyController,
    ActiveNamespaceIdList(NamespaceId),
    NamespaceIdentificationDescriptorList(NamespaceId),
    IOCommandSetSpecificIdentifyNamespace(NamespaceId, CommandSetIdentifier),
}

impl IdentifyCNSValue {
    fn nsid(&self) -> Option<NamespaceId> {
        match self {
            IdentifyCNSValue::IdentifyNamespace(n) => Some(*n),
            IdentifyCNSValue::IdentifyController => None,
            IdentifyCNSValue::ActiveNamespaceIdList(n) => Some(*n),
            IdentifyCNSValue::NamespaceIdentificationDescriptorList(n) => Some(*n),
            IdentifyCNSValue::IOCommandSetSpecificIdentifyNamespace(n, _) => Some(*n),
        }
    }
    fn cns_value(&self) -> u8 {
        match self {
            IdentifyCNSValue::IdentifyNamespace(_) => 0,
            IdentifyCNSValue::IdentifyController => 1,
            IdentifyCNSValue::ActiveNamespaceIdList(_) => 2,
            IdentifyCNSValue::NamespaceIdentificationDescriptorList(_) => 3,
            IdentifyCNSValue::IOCommandSetSpecificIdentifyNamespace(_, _) => 5,
        }
    }

    fn csi_value(&self) -> u8 {
        match self {
            IdentifyCNSValue::IdentifyNamespace(_) => 0,
            IdentifyCNSValue::IdentifyController => 0,
            IdentifyCNSValue::ActiveNamespaceIdList(_) => 0,
            IdentifyCNSValue::NamespaceIdentificationDescriptorList(_) => 0,
            IdentifyCNSValue::IOCommandSetSpecificIdentifyNamespace(_, c) => *c as u8,
        }
    }

    fn cntid_value(&self) -> u16 {
        match self {
            IdentifyCNSValue::IdentifyNamespace(_) => 0,
            IdentifyCNSValue::IdentifyController => 0,
            IdentifyCNSValue::ActiveNamespaceIdList(_) => 0,
            IdentifyCNSValue::NamespaceIdentificationDescriptorList(_) => 0,
            IdentifyCNSValue::IOCommandSetSpecificIdentifyNamespace(_, _) => 0,
        }
    }

    fn specific_id_value(&self) -> u16 {
        match self {
            IdentifyCNSValue::IdentifyNamespace(_) => 0,
            IdentifyCNSValue::IdentifyController => 0,
            IdentifyCNSValue::ActiveNamespaceIdList(_) => 0,
            IdentifyCNSValue::NamespaceIdentificationDescriptorList(_) => 0,
            IdentifyCNSValue::IOCommandSetSpecificIdentifyNamespace(_, _) => 0,
        }
    }
}
