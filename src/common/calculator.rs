// src/common/calculator.rs

use crate::common::types::{IpAddrTrait, IpNetTrait, IpSubnetResult};

pub fn collect_subnets<Net>(
    iter: &mut dyn Iterator<Item = Net>,
    total: u128,
    subnet_prefix: u8,
    base_network: Net,
    limit: usize,
    last_n: usize,
    addr_bits: u32,
    build_result: impl Fn(Net) -> IpSubnetResult,
) -> Vec<IpSubnetResult>
where
    Net: IpNetTrait,
{
    let mut subnets = Vec::new();

    if (total as usize) <= limit {
        // Show all within limit
        for _ in 0..limit {
            if let Some(net) = iter.next() {
                subnets.push(build_result(net));
            }
        }
    } else {
        // Truncated: first (limit - last_n) + last last_n
        let first_k = limit - last_n;

        // Collect first chunk
        for _ in 0..first_k {
            if let Some(net) = iter.next() {
                subnets.push(build_result(net));
            }
        }

        // Calculate and collect last chunk manually
        let subnet_size = 1u128 << (addr_bits - subnet_prefix as u32);
        let base_u128 = base_network.network().to_u128();

        for k in 0..last_n {
            let n = total - (last_n as u128 - 1) + k as u128;
            let offset = (n - 1) * subnet_size;
            let start_u128 = base_u128 + offset;
            let start = Net::Addr::from_u128(start_u128);
            let net = Net::new(start, subnet_prefix).unwrap();
            subnets.push(build_result(net));
        }
    }

    subnets
}

/// Shared function to calculate bits needed for at least `count` items
pub fn bits_needed_for_count(count: u128) -> u8 {
    if count == 0 {
        0
    } else {
        (count as f64).log2().ceil() as u8
    }
}