use ipnet::{Ipv6Net, Ipv6Subnets};
use std::net::Ipv6Addr;
use std::str::FromStr;
use crate::ipv6::types::{CalculationResult, HierarchyLevel, HierarchyNode, HierarchyResult, Ipv6InputError, SubnetMode, SubnetResult};

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

fn collect_subnets(mut iter: Ipv6Subnets, total: u128, subnet_prefix: u8, base_network: Ipv6Net) -> Vec<SubnetResult> {
    let mut subnets = vec![];
    if (total as usize) <= LIMIT {
        for net in iter.take(LIMIT) {
            subnets.push(build_subnet_result(net));
        }
    } else {
        let first_k = LIMIT - LAST_N;

        // Collect first `first_k` subnets
        for _ in 0..first_k {
            if let Some(net) = iter.next() {
                subnets.push(build_subnet_result(net));
            }
        }

        // Now calculate the last `LAST_N` manually
        let subnet_size = 1u128 << (128 - subnet_prefix as u32);
        let base_u128 = u128::from(base_network.network());

        for k in 0..LAST_N {
            //let n = total - (LAST_N as u128 - k as u128);  // index of the k-th last subnet (1-based)
            let n = total - (LAST_N as u128 - k as u128 - 1);
            let offset = (n - 1) * subnet_size;
            let start = Ipv6Addr::from(base_u128 + offset);
            let net = Ipv6Net::new(start, subnet_prefix).unwrap();
            subnets.push(build_subnet_result(net));
        }
    }
    subnets
}

pub fn get_addr_type(addr: Ipv6Addr) -> String {
    if addr.is_unspecified() {
        "Unspecified".to_string()
    } else if addr.is_loopback() {
        "Loopback".to_string()
    } else if addr.is_unique_local() {
        "Unique Local".to_string()
    } else if addr.is_unicast_link_local() {
        "Link-Local".to_string()
    } else if addr.is_multicast() {
        "Multicast".to_string()
    } else if is_unicast_global(addr) {  // Note: this is the stable name!
        "Global Unicast".to_string()
    } else {
        "Other".to_string()
    }
}

fn build_subnet_result(net: Ipv6Net) -> SubnetResult {
    let addr = net.network();
    let compressed = addr.to_string();
    let expanded = expand_ipv6(addr);
    let addr_type = get_addr_type(addr);
    let first = net.hosts().next().unwrap_or(addr).to_string();
    let last = net.hosts().last().unwrap_or(addr).to_string();
    SubnetResult {
        network: net,
        compressed,
        expanded,
        addr_type,
        first_host: first,
        last_host: last,
    }
}

pub fn calculate(
    addr: &str,
    prefix_str: &str,
    mode: SubnetMode,
    needed_subnets: Option<u32>,
    child_prefix: Option<u8>,
    hierarchy_levels: Vec<HierarchyLevel>,
) -> Result<CalculationResult, Ipv6InputError> {
    let base_network = Ipv6Net::from_str(&format!("{}/{}", addr.trim(), prefix_str.trim().strip_prefix('/').unwrap_or(prefix_str)))
        .map_err(|e| Ipv6InputError::ParseError(e.to_string()))?;
    let base_prefix = base_network.prefix_len();

    let mut subnets = vec![];
    let new_prefix: Option<u8>;
    let total_subnets: u128;

    // New: Optional hierarchy result
    let mut hierarchy: Option<HierarchyResult> = None;

    match mode {
        SubnetMode::Inspect => {
            new_prefix = None;
            total_subnets = 1;
            // subnets remains empty
            subnets.push(build_subnet_result(base_network));
        }

        SubnetMode::BySubnets => {
            let count = needed_subnets.ok_or(Ipv6InputError::ParseError("Missing count".into()))? as u128;
            let bits_needed = count.next_power_of_two().trailing_zeros() as u8;
            let np = base_prefix.checked_add(bits_needed)
                .ok_or(Ipv6InputError::InsufficientBits)?;
            if np > 128 {
                return Err(Ipv6InputError::InsufficientBits);
            }
            new_prefix = Some(np);
            total_subnets = 1u128 << bits_needed as u32;
            let iter = base_network.subnets(np).unwrap();
            subnets = collect_subnets(iter, total_subnets, np, base_network);
        }

        SubnetMode::ByPrefix => {
            let np = child_prefix.ok_or(Ipv6InputError::ParseError("Missing prefix".into()))?;
            if np <= base_prefix || np > 128 {
                return Err(Ipv6InputError::InvalidPrefix);
            }
            new_prefix = Some(np);
            total_subnets = 1u128 << (np - base_prefix) as u32;
            let iter = base_network.subnets(np).unwrap();
            subnets = collect_subnets(iter, total_subnets, np, base_network);
        }

        SubnetMode::ByHierarchy => {
            if hierarchy_levels.is_empty() {
                new_prefix = None;
                total_subnets = 1;
                // subnets remains empty
            } else {
                // Fixed: Properly build nested hierarchy tree with correct prefixes
                // Initialize root
                let mut root = HierarchyNode {
                    prefix: base_network,
                    label: "Original Network".to_string(),
                    children: vec![],
                };

                // Current set of parents to add children to (starts with root)
                let mut current_parents: Vec<&mut HierarchyNode> = vec![&mut root];

                let mut current_prefix = base_prefix;

                for level in hierarchy_levels.iter() {
                    // Calculate min bits needed
                    let bits_needed = (level.num as f64).log2().ceil() as u8;
                    if bits_needed > level.bits {
                        return Err(Ipv6InputError::InsufficientBits);
                    }

                    // Update cumulative prefix for child subnets
                    current_prefix = current_prefix.checked_add(level.bits)
                        .ok_or(Ipv6InputError::InsufficientBits)?;
                    if current_prefix > 128 {
                        return Err(Ipv6InputError::InsufficientBits);
                    }

                    // Prepare next set of parents
                    let mut new_parents = vec![];

                    // For each current parent, generate and attach children
                    for parent in current_parents {
                        let mut children = vec![];

                        // Generate subnets from parent's prefix at current_prefix length
                        if let Ok(iter) = parent.prefix.subnets(current_prefix) {
                            for (i, net) in iter.enumerate().take(level.num as usize) {
                                children.push(HierarchyNode {
                                    prefix: net,
                                    label: format!("{} {}", level.name, i + 1),
                                    children: vec![],
                                });
                            }
                        }

                        // Attach to parent
                        parent.children = children;

                        // Add children as next parents (mutable refs)
                        for child in parent.children.iter_mut() {
                            new_parents.push(child);
                        }
                    }

                    // Update for next level
                    current_parents = new_parents;
                }

                // Use root's children as the top-level tree (skip "Root")
                //let tree = root.children;
                let tree = vec![root];

                hierarchy = Some(HierarchyResult {
                    levels: hierarchy_levels.clone(),
                    tree,
                });

                new_prefix = None;
                total_subnets = 0;
                subnets = vec![];
            }
        }
    }

    Ok(CalculationResult {
        base_network,
        summary: if new_prefix.is_some() || hierarchy.is_some() {
            subnets.first().cloned().unwrap_or(build_subnet_result(base_network))
        } else {
            build_subnet_result(base_network)
        },
        subnets,
        new_prefix,
        total_subnets,
        hierarchy,
    })
}
