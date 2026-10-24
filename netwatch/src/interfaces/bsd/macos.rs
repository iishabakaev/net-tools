use super::{MessageType, RoutingStack, WireFormat};

// `<net/route.h>` is not in the iOS SDK, so libc defines these for macOS only
// (since 0.2.190). iOS uses the same values (XNU).
#[cfg(target_os = "ios")]
pub(crate) use self::route_h::*;
#[cfg(target_os = "macos")]
pub(crate) use libc::{
    RTA_BRD, RTA_DST, RTA_GATEWAY, RTA_IFA, RTA_IFP, RTA_NETMASK, RTAX_BRD, RTAX_DST, RTAX_GATEWAY,
    RTAX_IFP, RTAX_MAX, RTAX_NETMASK, RTF_GATEWAY, RTF_IFSCOPE, RTM_ADD, RTM_CHANGE, RTM_DELADDR,
    RTM_DELETE, RTM_DELMADDR, RTM_GET, RTM_GET2, RTM_IFINFO, RTM_IFINFO2, RTM_LOCK, RTM_LOSING,
    RTM_MISS, RTM_NEWADDR, RTM_NEWMADDR, RTM_NEWMADDR2, RTM_REDIRECT, RTM_RESOLVE, RTM_VERSION,
};

#[cfg(target_os = "ios")]
mod route_h {
    use libc::c_int;

    pub(crate) const RTF_GATEWAY: c_int = 0x2;
    pub(crate) const RTF_IFSCOPE: c_int = 0x1000000;

    pub(crate) const RTM_VERSION: c_int = 5;
    pub(crate) const RTM_ADD: c_int = 0x1;
    pub(crate) const RTM_DELETE: c_int = 0x2;
    pub(crate) const RTM_CHANGE: c_int = 0x3;
    pub(crate) const RTM_GET: c_int = 0x4;
    pub(crate) const RTM_LOSING: c_int = 0x5;
    pub(crate) const RTM_REDIRECT: c_int = 0x6;
    pub(crate) const RTM_MISS: c_int = 0x7;
    pub(crate) const RTM_LOCK: c_int = 0x8;
    pub(crate) const RTM_RESOLVE: c_int = 0xb;
    pub(crate) const RTM_NEWADDR: c_int = 0xc;
    pub(crate) const RTM_DELADDR: c_int = 0xd;
    pub(crate) const RTM_IFINFO: c_int = 0xe;
    pub(crate) const RTM_NEWMADDR: c_int = 0xf;
    pub(crate) const RTM_DELMADDR: c_int = 0x10;
    pub(crate) const RTM_IFINFO2: c_int = 0x12;
    pub(crate) const RTM_NEWMADDR2: c_int = 0x13;
    pub(crate) const RTM_GET2: c_int = 0x14;

    pub(crate) const RTA_DST: c_int = 0x1;
    pub(crate) const RTA_GATEWAY: c_int = 0x2;
    pub(crate) const RTA_NETMASK: c_int = 0x4;
    pub(crate) const RTA_IFP: c_int = 0x10;
    pub(crate) const RTA_IFA: c_int = 0x20;
    pub(crate) const RTA_BRD: c_int = 0x80;

    pub(crate) const RTAX_DST: c_int = 0;
    pub(crate) const RTAX_GATEWAY: c_int = 1;
    pub(crate) const RTAX_NETMASK: c_int = 2;
    pub(crate) const RTAX_IFP: c_int = 4;
    pub(crate) const RTAX_BRD: c_int = 7;
    pub(crate) const RTAX_MAX: c_int = 8;
}

// Hardcoded based on the generated values here: https://cs.opensource.google/go/x/net/+/master:route/zsys_darwin.go
const SIZEOF_IF_MSGHDR_DARWIN15: usize = 0x70;
const SIZEOF_IFA_MSGHDR_DARWIN15: usize = 0x14;
const SIZEOF_IFMA_MSGHDR_DARWIN15: usize = 0x10;
const SIZEOF_IF_MSGHDR2_DARWIN15: usize = 0xa0;
const SIZEOF_IFMA_MSGHDR2_DARWIN15: usize = 0x14;
const SIZEOF_IF_DATA_DARWIN15: usize = 0x60;
const SIZEOF_IF_DATA64_DARWIN15: usize = 0x80;

const SIZEOF_RT_MSGHDR_DARWIN15: usize = 0x5c;
const SIZEOF_RT_MSGHDR2_DARWIN15: usize = 0x5c;
const SIZEOF_RT_METRICS_DARWIN15: usize = 0x38;

const SIZEOF_SOCKADDR_STORAGE: usize = 0x80;
pub(super) const SIZEOF_SOCKADDR_INET: usize = 0x10;
pub(super) const SIZEOF_SOCKADDR_INET6: usize = 0x1c;

pub(super) fn probe_routing_stack() -> RoutingStack {
    let rtm_version = RTM_VERSION;

    let rtm = WireFormat {
        ext_off: 36,
        body_off: SIZEOF_RT_MSGHDR_DARWIN15,
        typ: MessageType::Route,
    };
    let rtm2 = WireFormat {
        ext_off: 36,
        body_off: SIZEOF_RT_MSGHDR2_DARWIN15,
        typ: MessageType::Route,
    };
    let ifm = WireFormat {
        ext_off: 16,
        body_off: SIZEOF_IF_MSGHDR_DARWIN15,
        typ: MessageType::Interface,
    };
    let ifm2 = WireFormat {
        ext_off: 32,
        body_off: SIZEOF_IF_MSGHDR2_DARWIN15,
        typ: MessageType::Interface,
    };
    let ifam = WireFormat {
        ext_off: SIZEOF_IFA_MSGHDR_DARWIN15,
        body_off: SIZEOF_IFA_MSGHDR_DARWIN15,
        typ: MessageType::InterfaceAddr,
    };
    let ifmam = WireFormat {
        ext_off: SIZEOF_IFMA_MSGHDR_DARWIN15,
        body_off: SIZEOF_IFMA_MSGHDR_DARWIN15,
        typ: MessageType::InterfaceMulticastAddr,
    };
    let ifmam2 = WireFormat {
        ext_off: SIZEOF_IFMA_MSGHDR2_DARWIN15,
        body_off: SIZEOF_IFMA_MSGHDR2_DARWIN15,
        typ: MessageType::InterfaceMulticastAddr,
    };

    let wire_formats = [
        (RTM_ADD, rtm),
        (RTM_DELETE, rtm),
        (RTM_CHANGE, rtm),
        (RTM_GET, rtm),
        (RTM_LOSING, rtm),
        (RTM_REDIRECT, rtm),
        (RTM_MISS, rtm),
        (RTM_LOCK, rtm),
        (RTM_RESOLVE, rtm),
        (RTM_NEWADDR, ifam),
        (RTM_DELADDR, ifam),
        (RTM_IFINFO, ifm),
        (RTM_NEWMADDR, ifmam),
        (RTM_DELMADDR, ifmam),
        (RTM_IFINFO2, ifm2),
        (RTM_NEWMADDR2, ifmam2),
        (RTM_GET2, rtm2),
    ]
    .into_iter()
    .collect();

    RoutingStack {
        rtm_version,
        wire_formats,
        kernel_align: 4,
    }
}
