use ipnet::{Ipv4Net};
use std::net::Ipv4Addr;

use crate::ipv4::types::{CalculationResult, Ipv4InputError, SubnetResult};

pub fn parse_network(ip: &str, mask_or_prefix: &str) -> Result<Ipv4Net, Ipv4InputError> {
    let ip: Ipv4Addr = ip.trim()
    .parse()
    .map_err(|e: std::net::AddrParseError| Ipv4InputError::ParseError(e.to_string()))?;

    let trimmed = mask_or_prefix.trim();

    // Try as CIDR prefix first (e.g. "24")
    if let Ok(prefix) = trimmed.strip_prefix('/').unwrap_or(trimmed).parse::<u8>() {
        if prefix > 32 {
            return Err(Ipv4InputError::InvalidPrefix);
        }
        return Ok(Ipv4Net::new(ip, prefix).map_err(|e| Ipv4InputError::InvalidPrefix)?);
    }

    if let Ok(mask) = trimmed.parse::<Ipv4Addr>() {
        return Ipv4Net::with_netmask(ip, mask)
            .map_err(|_| Ipv4InputError::InvalidMask);
    }

    Err(Ipv4InputError::ParseError("Invalid CIDR or subnet mask".into()))
}

fn build_subnet_result(net: Ipv4Net) -> SubnetResult {
    let hosts = net.hosts();
    let mut iter = hosts;
    let first = iter.next().map(|h| h.to_string());
    let last = iter.last().map(|h| h.to_string());

    let total = 2u32.pow(32 - net.prefix_len() as u32);
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

    let mut subnets = Vec::new();

    let (new_prefix, subnet_iter): (Option<u8>, Box<dyn Iterator<Item = Ipv4Net>>) = if let Some(hosts) = needed_hosts {
        // Find smallest prefix that gives at least 'hosts' usable
        let required = hosts + 2; // include network + broadcast
        let new_prefix = 32 - (required.next_power_of_two().trailing_zeros());


        let available_usable = if 2u32.pow(32 - base_network.prefix_len() as u32) >= 2 {
                2u32.pow(32 - base_network.prefix_len() as u32) - 2
            } else {
                0
            };

        if hosts > available_usable {
                return Err(Ipv4InputError::ParseError(format!(
                    "Too many hosts requested",
                )));
            }

        (Some(new_prefix as u8), Box::new(base_network.subnets(new_prefix as u8).unwrap()))
    } else if let Some(count) = needed_subnets {
        if count == 0 || count > 2u32.pow(32 - base_network.prefix_len() as u32) {
            return Err(Ipv4InputError::ParseError("Too many subnets requested".into()));
        }
        let bits_needed = (count as f32).log2().ceil() as u8;
        let new_prefix = base_network.prefix_len() + bits_needed;

        (Some(new_prefix), Box::new(base_network.subnets(new_prefix).unwrap()))
    } else {
        // Basic mode
        (None, Box::new(base_network.subnets(base_network.prefix_len()).unwrap()))
    };

    for net in subnet_iter.take(10000) {  // Limit to prevent huge lists
        subnets.push(build_subnet_result(net));
    }

    Ok(CalculationResult {
        base_network,
        summary: build_subnet_result(base_network),
        subnets,
        new_prefix,
    })
}