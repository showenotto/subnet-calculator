// src/common/types.rs

use ipnet::{Ipv4Net, Ipv6Net};
use std::net::{Ipv4Addr, Ipv6Addr};
use ipnet::PrefixLenError;

use crate::ipv6::types::HierarchyResult;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubnetResultV4 {
    pub network: Ipv4Net,
    pub netmask: String,
    pub wildcard: String,
    pub broadcast: String,
    pub first_host: Option<String>,
    pub last_host: Option<String>,
    pub usable_hosts: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubnetResultV6 {
    pub network: Ipv6Net,
    pub compressed: String,
    pub expanded: String,
    pub addr_type: String,
    pub first_host: String,
    pub last_host: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IpSubnetResult {
    V4(SubnetResultV4),
    V6(SubnetResultV6),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalculationResult<T> {
    pub base_network: T,
    pub summary: IpSubnetResult,
    pub subnets: Vec<IpSubnetResult>,
    pub new_prefix: Option<u8>,
    pub total_subnets: u128, // u128 to support huge IPv6 counts
    pub hierarchy: Option<HierarchyResult>, // Now shared, but IPv6-only in practice
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubnetMode {
    Inspect,
    BySubnets,
    ByHosts,     // IPv4 only
    ByPrefix,    // IPv6 only
    ByHierarchy, // IPv6 only
}


pub trait IpAddrTrait {
    fn to_u128(&self) -> u128;
    fn from_u128(v: u128) -> Self;
}

impl IpAddrTrait for Ipv4Addr {
    fn to_u128(&self) -> u128 {
        u32::from(*self) as u128
    }
    fn from_u128(v: u128) -> Self {
        Self::from(v as u32)
    }
}

impl IpAddrTrait for Ipv6Addr {
    fn to_u128(&self) -> u128 {
        u128::from(*self)
    }
    fn from_u128(v: u128) -> Self {
        Self::from(v)
    }
}

pub trait IpNetTrait: Clone {
    type Addr: IpAddrTrait + Copy;
    type Subnets: Iterator<Item = Self>;

    fn network(&self) -> Self::Addr;
    fn prefix_len(&self) -> u8;
    fn new(addr: Self::Addr, prefix: u8) -> Result<Self, PrefixLenError>;
    fn subnets(&self, new_prefix: u8) -> Result<Self::Subnets, PrefixLenError>;
}

pub type Ipv4Subnets = ipnet::Ipv4Subnets;
pub type Ipv6Subnets = ipnet::Ipv6Subnets;

impl IpNetTrait for Ipv4Net {
    type Addr = Ipv4Addr;
    type Subnets = ipnet::Ipv4Subnets;

    fn network(&self) -> Self::Addr {
        self.network()
    }

    fn prefix_len(&self) -> u8 {
        self.prefix_len()
    }

    fn new(addr: Self::Addr, prefix: u8) -> Result<Self, PrefixLenError> {
        Ipv4Net::new(addr, prefix)
    }

    fn subnets(&self, new_prefix: u8) -> Result<Self::Subnets, PrefixLenError> {
        self.subnets(new_prefix)
    }
}

impl IpNetTrait for Ipv6Net {
    type Addr = Ipv6Addr;
    type Subnets = ipnet::Ipv6Subnets;

    fn network(&self) -> Self::Addr {
        self.network()
    }

    fn prefix_len(&self) -> u8 {
        self.prefix_len()
    }

    fn new(addr: Self::Addr, prefix: u8) -> Result<Self, PrefixLenError> {
        Ipv6Net::new(addr, prefix)
    }

    fn subnets(&self, new_prefix: u8) -> Result<Self::Subnets, PrefixLenError> {
        self.subnets(new_prefix)
    }
}