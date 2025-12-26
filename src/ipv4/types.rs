// src/ipv4/types.rs
use ipnet::{AddrParseError, Ipv4Net};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Ipv4InputError {
    ParseError(String),
    InvalidPrefix,
}

#[derive(Clone, Debug)]
pub struct Ipv4Result {
    pub network: Ipv4Net,
    pub subnet_mask: String,
    pub wildcard_mask: String,
    pub broadcast: String,
    pub first_host: Option<String>,
    pub last_host: Option<String>,
    pub total_hosts: u32,
    pub usable_hosts: u32,
}

pub fn calculate_from_cidr(input: &str) -> Result<Ipv4Result, Ipv4InputError> {
    let net: Ipv4Net = input
        .trim()
        .parse()
        .map_err(|e: AddrParseError| Ipv4InputError::ParseError(e.to_string()))?;

    let hosts = net.hosts();
    let mut hosts_iter = hosts;
    let first_host = hosts_iter.next().map(|ip| ip.to_string());
    let last_host = hosts_iter.last().map(|ip| ip.to_string());

    let total_hosts = 2u32.pow(32 - net.prefix_len() as u32);
    let usable_hosts = if total_hosts >= 2 { total_hosts - 2 } else { 0 };

    Ok(Ipv4Result {
        network: net,
        subnet_mask: net.netmask().to_string(),
        wildcard_mask: (!net.netmask()).to_string(),
        broadcast: net.broadcast().to_string(),
        first_host,
        last_host,
        total_hosts,
        usable_hosts,
    })
}