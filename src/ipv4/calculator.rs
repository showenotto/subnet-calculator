use ipnet::{Ipv4Net};
use std::net::Ipv4Addr;
use crate::ipv4::types::{CalculationResult, Ipv4InputError};
use crate::common::types::{IpSubnetResult, SubnetResultV4 as SubnetResult};
use crate::common::calculator::{bits_needed_for_count, collect_subnets};

pub const LIMIT: usize = 4096;
pub const LAST_N: usize = 10;

pub fn parse_network(ip: &str, mask_or_prefix: &str) -> Result<Ipv4Net, Ipv4InputError> {
    let ip: Ipv4Addr = ip.trim()
        .parse()
        .map_err(|e: std::net::AddrParseError| Ipv4InputError::ParseError(e.to_string()))?;

    let trimmed = mask_or_prefix.trim();

    if let Ok(prefix) = trimmed.strip_prefix('/').unwrap_or(trimmed).parse::<u8>() {
        if prefix > 32 {
            return Err(Ipv4InputError::InvalidPrefix);
        }
        return Ok(Ipv4Net::new(ip, prefix).map_err(|_| Ipv4InputError::InvalidPrefix)?);
    }

    if let Ok(mask) = trimmed.parse::<Ipv4Addr>() {
        return Ipv4Net::with_netmask(ip, mask)
            .map_err(|_| Ipv4InputError::InvalidMask);
    }

    Err(Ipv4InputError::ParseError("Invalid CIDR or subnet mask".into()))
}

fn build_subnet_result(net: Ipv4Net) -> SubnetResult {
    let total = 2u32.pow(32 - net.prefix_len() as u32);
    let hosts = net.hosts();
    let mut iter = hosts;
    let first = iter.next().map(|h| h.to_string());
    let last = iter.last().map(|h| h.to_string());
    let usable = if total >= 2 { total - 2 } else { 0 };

    SubnetResult {
        network: net,
        netmask: net.netmask().to_string(),
        wildcard: (!net.netmask()).to_string(),
        broadcast: net.broadcast().to_string(),
        first_host: first,
        last_host: last,
        usable_hosts: usable,
    }
}

pub fn calculate(
    ip: &str,
    mask_or_prefix: &str,
    needed_hosts: Option<u32>,
    needed_subnets: Option<u32>,
) -> Result<CalculationResult, Ipv4InputError> {
    let base_network = parse_network(ip, mask_or_prefix)?;

    let (new_prefix, mut subnet_iter): (Option<u8>, Box<dyn Iterator<Item = Ipv4Net>>) = if let Some(hosts) = needed_hosts {
        let required = hosts + 2;
        let bits_for_hosts = (required as u64).next_power_of_two().trailing_zeros() as u8;
        let new_prefix = 32 - bits_for_hosts;

        let available_usable = if 2u32.pow(32 - base_network.prefix_len() as u32) >= 2 {
            2u32.pow(32 - base_network.prefix_len() as u32) - 2
        } else {
            0
        };

        if hosts > available_usable {
            return Err(Ipv4InputError::ParseError("Too many hosts requested".into()));
        }

        // Fix: Manual map error from PrefixLenError to Ipv4InputError
        let iter = base_network.subnets(new_prefix)
            .map_err(|_| Ipv4InputError::InvalidPrefix)?;

        (Some(new_prefix), Box::new(iter))
    } else if let Some(count) = needed_subnets {
        if count == 0 || (count as u128) > 1u128 << (32 - base_network.prefix_len() as u32) {
            return Err(Ipv4InputError::ParseError("Too many subnets requested".into()));
        }
        let bits_needed = bits_needed_for_count(count as u128);
        let new_prefix = base_network.prefix_len() + bits_needed;

        // Fix: Manual map error from PrefixLenError to Ipv4InputError
        let iter = base_network.subnets(new_prefix)
            .map_err(|_| Ipv4InputError::InvalidPrefix)?;

        (Some(new_prefix), Box::new(iter))
    } else {
        // Fix: Manual map error from PrefixLenError to Ipv4InputError
        let iter = base_network.subnets(base_network.prefix_len())
            .map_err(|_| Ipv4InputError::InvalidPrefix)?;

        (None, Box::new(iter))
    };

    let total_subnets: u128 = if let Some(np) = new_prefix {
        1u128 << (np - base_network.prefix_len()) as u32
    } else {
        1
    };

    let subnet_prefix = new_prefix.unwrap_or(base_network.prefix_len());
    let iter_ref: &mut dyn Iterator<Item = Ipv4Net> = &mut *subnet_iter;
    let subnets = collect_subnets(
        iter_ref,
        total_subnets,
        subnet_prefix,
        base_network,
        LIMIT,
        LAST_N,
        32,
        |net| IpSubnetResult::V4(build_subnet_result(net)),
    );

    Ok(CalculationResult {
        base_network,
        summary: IpSubnetResult::V4(build_subnet_result(base_network)),
        subnets,
        new_prefix,
        total_subnets,
        hierarchy: None,
    })
}