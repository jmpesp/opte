//! The Oxide Network configuration.
//!
//! This module contains configuration that is specific to the "Oxide
//! Network" -- i.e., the underlay/overlay that we implement on an
//! Oxide Rack. OPTE itself is a generic engine for performing packet
//! transformations in a flow-centric manner. While it does provide
//! primitve building blocks for implementing network functions, like
//! rules and header transpositions, it does not dictate a specific
//! network configuration. This module configures OPTE in a manner
//! consistent with the definition of The Oxide Network [^rfd63].
//!
//! This should probably be in its own crate, separate from OPTE
//! itself. For now keeping it here is convenient.
//!
//! [rfd63]: [RFD 63 Network
//! Architecture](https://rfd.shared.oxide.computer/rfd/0063)
use std::ops::Range;

use crate::ether::EtherAddr;
use crate::geneve::Vni;
use crate::ip4::Ipv4Addr;
use crate::ip6::Ipv6Addr;
use crate::vpc::VpcSubnet4;

pub mod arp;
pub mod dyn_nat4;
pub mod firewall;
pub mod overlay;
pub mod router;

#[derive(Clone, Debug)]
pub struct DynNat4Config {
    pub public_mac: EtherAddr,
    pub public_ip: Ipv4Addr,
    pub ports: Range<u16>,
}

#[derive(Clone, Debug)]
pub struct OverlayConfig {
    pub boundary_services: overlay::PhysNet,
    pub vni: Vni,
    // NOTE: The phys_mac_{src,dst} currently stand in for the
    // physical routing service. The src should be the host NIC MAC
    // address, and the dst should be the physical gateway MAC address
    // on your home/lab network.
    pub phys_mac_src: EtherAddr,
    pub phys_mac_dst: EtherAddr,
    pub phys_ip_src: Ipv6Addr,
}

#[derive(Clone, Debug)]
pub struct PortConfig {
    pub vpc_subnet: VpcSubnet4,
    pub private_mac: EtherAddr,
    pub private_ip: Ipv4Addr,
    pub gw_mac: EtherAddr,
    pub gw_ip: Ipv4Addr,
    pub dyn_nat: DynNat4Config,
    // XXX For the moment we allow the overlay to be optional. This
    // allows the Oxide Network to continue functioning on a local
    // IPv4 network until the IPv6 underlay support is more flushed
    // out and development/testing strategies are determined.
    pub overlay: Option<OverlayConfig>,
}
