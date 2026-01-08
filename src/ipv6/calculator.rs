use ipnet::{Ipv6Net};
use std::net::Ipv6Addr;
use std::str::FromStr;
use crate::common::calculator::{bits_needed_for_count, collect_subnets};
use crate::ipv6::types::{HierarchyLevel, HierarchyNode, HierarchyResult, Ipv6InputError, MAX_USABLE_SUBNETS, SubnetMode};
use crate::common::types::{CalculationResult, IpSubnetResult, SubnetResultV6 as SubnetResult};

pub const LIMIT: usize = 8192;
pub const LAST_N: usize = 10;

pub fn expand_ipv6(addr: Ipv6Addr) -> String {
    format!("{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}", 
        (addr.octets()[0] as u16) << 8 | addr.octets()[1] as u16,
        (addr.octets()[2] as u16) << 8 | addr.octets()[3] as u16,
        (addr.octets()[4] as u16) << 8 | addr.octets()[5] as u16,
        (addr.octets()[6] as u16) << 8 | addr.octets()[7] as u16,
        (addr.octets()[8] as u16) << 8 | addr.octets()[9] as u16,
        (addr.octets()[10] as u16) << 8 | addr.octets()[11] as u16,
        (addr.octets()[12] as u16) << 8 | addr.octets()[13] as u16,
        (addr.octets()[14] as u16) << 8 | addr.octets()[15] as u16,
    )
}

fn is_unicast_global(addr: Ipv6Addr) -> bool {
    let segments = addr.segments();

    // Exclude documentation ranges: 2001:db8::/32 and 3fff::/20 (ORCHIDv2, but treated similarly)
    if segments[0] == 0x2001 && segments[1] == 0xdb8 { return false; } // 2001:db8::/32
    if segments[0] == 0x3fff && (segments[1] & 0xffc0) == 0x0000 { return false; } // 3fff::/20 (top 20 bits)

    // Exclude benchmarking: 2001:2::/48
    if segments[0] == 0x2001 && segments[1] == 0x0002 && segments[2] == 0x0000 { return false; }

    // Exclude 6to4: 2002::/16
    if segments[0] == 0x2002 { return false; }

    // Exclude Teredo: 2001::/32
    if segments[0] == 0x2001 && segments[1] == 0x0000 { return false; }

    // Everything else in 2000::/3 is global unicast
    (segments[0] & 0xe000) == 0x2000
}

pub fn get_addr_type(addr: Ipv6Addr) -> String {
    let segments = addr.segments();
    if addr.is_unspecified() {
        "Unspecified".to_string()
    } else if addr.is_loopback() {
        "Loopback".to_string()
    } else if addr.is_multicast() {
        "Multicast".to_string()
    } else if segments[0] == 0 && segments[1] == 0 && segments[2] == 0 && segments[3] == 0 && segments[4] == 0 && segments[5] == 0xffff {
        "IPv4-Mapped".to_string()
    } else if is_unicast_global(addr) {
        "Global Unicast".to_string()
    } else if addr.segments()[0] == 0xfc00 {
        "Unique Local".to_string()
    } else if addr.segments()[0] == 0xfe80 {
        "Link-Local".to_string()
    } else {
        "Reserved/Other".to_string()
    }
}

fn build_subnet_result(net: Ipv6Net) -> SubnetResult {
    let compressed = net.to_string();
    let expanded = expand_ipv6(net.network());
    let addr_type = get_addr_type(net.network());
    let first_host = if net.prefix_len() < 128 {
        let addr_u128 = u128::from(net.network()) + 1;
        expand_ipv6(Ipv6Addr::from(addr_u128))
    } else {
        expand_ipv6(net.network())
    };
    let last_host = if net.prefix_len() < 128 {
        expand_ipv6(net.broadcast())
    } else {
        expand_ipv6(net.network())
    };

    SubnetResult {
        network: net,
        compressed,
        expanded,
        addr_type,
        first_host,
        last_host,
    }
}

pub fn parse_network(input: &str) -> Result<Ipv6Net, Ipv6InputError> {
    Ipv6Net::from_str(input.trim()).map_err(|e| Ipv6InputError::ParseError(e.to_string()))
}

pub fn calculate(
    addr: &str,
    prefix_str: &str,
    mode: SubnetMode,
    needed_subnets: Option<u128>,
    target_prefix: Option<u8>,
    hierarchy_levels: Vec<HierarchyLevel>,
) -> Result<CalculationResult<Ipv6Net>, Ipv6InputError> {
    // Merge address and prefix, handling potential leading slashes in the prefix_str
    let full_input = format!(
        "{}/{}", 
        addr.trim(), 
        prefix_str.trim().strip_prefix('/').unwrap_or(prefix_str.trim())
    );
    
    let base_network = parse_network(&full_input)?;
    let base_prefix = base_network.prefix_len();
    let mut new_prefix = None;
    let mut total_subnets = 1u128;
    let mut subnets: Vec<IpSubnetResult> = vec![];
    let mut hierarchy: Option<HierarchyResult> = None;

    match mode {
        SubnetMode::Inspect => {
            subnets.push(IpSubnetResult::V6(build_subnet_result(base_network)));
        }
        SubnetMode::ByPrefix => {
            let target_prefix = target_prefix.ok_or(Ipv6InputError::InvalidPrefix)?;
            if target_prefix <= base_prefix || target_prefix > 128 {
                return Err(Ipv6InputError::InvalidPrefix);
            }
            let bits = target_prefix - base_prefix;
            total_subnets = 1u128 << bits as u32;

            let mut iter = base_network.subnets(target_prefix).map_err(|_| Ipv6InputError::InvalidPrefix)?;
            subnets = collect_subnets(
                &mut iter,
                total_subnets,
                target_prefix,
                base_network,
                LIMIT,
                LAST_N,
                128,
                |net| IpSubnetResult::V6(build_subnet_result(net)),
            );

            new_prefix = Some(target_prefix);
        }
        SubnetMode::BySubnets => {
            let needed_subnets = needed_subnets.ok_or(Ipv6InputError::ParseError("Missing subnets count".into()))?;
            if needed_subnets == 0 || needed_subnets > MAX_USABLE_SUBNETS {
                return Err(Ipv6InputError::ParseError("Invalid subnet count".into()));
            }
            let available_bits = 128 - base_prefix;
            let bits_needed = bits_needed_for_count(needed_subnets);
            if bits_needed > available_bits {
                return Err(Ipv6InputError::InsufficientBits);
            }
            let target_prefix = base_prefix + bits_needed;
            total_subnets = 1u128 << bits_needed as u32;

            let mut iter = base_network.subnets(target_prefix).map_err(|_| Ipv6InputError::InvalidPrefix)?;
            subnets = collect_subnets(
                &mut iter,
                total_subnets,
                target_prefix,
                base_network,
                LIMIT,
                LAST_N,
                128,
                |net| IpSubnetResult::V6(build_subnet_result(net)),
            );

            new_prefix = Some(target_prefix);
        }
        SubnetMode::ByHierarchy => {
            if hierarchy_levels.is_empty() {
                return Err(Ipv6InputError::ParseError("No hierarchy levels provided".into()));
            }

            let mut root = HierarchyNode {
                prefix: base_network,
                label: "Original Network".to_string(),
                children: vec![],
            };

            let mut current_parents: Vec<&mut HierarchyNode> = vec![&mut root];
            let mut current_prefix = base_prefix;

            for level in hierarchy_levels.iter() {
                let bits_needed = bits_needed_for_count(level.num as u128);
                if bits_needed > level.bits {
                    return Err(Ipv6InputError::InsufficientBits);
                }

                current_prefix = current_prefix.checked_add(level.bits)
                    .ok_or(Ipv6InputError::InsufficientBits)?;
                if current_prefix > 128 {
                    return Err(Ipv6InputError::InsufficientBits);
                }

                let mut new_parents = vec![];

                for parent in current_parents {
                    let mut children = vec![];

                    if let Ok(iter) = parent.prefix.subnets(current_prefix) {
                        for (i, net) in iter.enumerate().take(level.num as usize) {
                            children.push(HierarchyNode {
                                prefix: net,
                                label: format!("{} {}", level.name, i + 1),
                                children: vec![],
                            });
                        }
                    }

                    parent.children = children;

                    for child in parent.children.iter_mut() {
                        new_parents.push(child);
                    }
                }

                current_parents = new_parents;
            }

            let tree = vec![root];

            hierarchy = Some(HierarchyResult {
                levels: hierarchy_levels.clone(),
                tree,
            });

            new_prefix = None;
            total_subnets = 0;
            subnets = vec![];
        }
        SubnetMode::ByHosts => todo!(),
    }

    Ok(CalculationResult {
        base_network,
        summary: if !subnets.is_empty() {
            subnets[0].clone()
        } else {
            IpSubnetResult::V6(build_subnet_result(base_network))
        },
        subnets,
        new_prefix,
        total_subnets,
        hierarchy,
    })
}