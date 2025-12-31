use ipnet::{Ipv4Net};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ipv4InputError {
    ParseError(String),
    InvalidMask,
    InvalidPrefix,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubnetResult {
    pub network: Ipv4Net,
    pub netmask: String,
    pub wildcard: String,
    pub broadcast: String,
    pub first_host: Option<String>,
    pub last_host: Option<String>,
    pub usable_hosts: u32,
}

#[derive(Clone, PartialEq)]
pub enum SubnetMode {
    ByHosts,
    BySubnets,
    Inspect,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalculationResult {
    pub base_network: Ipv4Net,
    pub summary: SubnetResult,
    pub subnets: Vec<SubnetResult>,  // Empty if no splitting
    pub new_prefix: Option<u8>,     // Only when splitting
    pub total_subnets: u64,
}

pub const CIDR_OPTIONS: &[(u8, &str, &str)] = &[
    (1,  "/1",   "128.0.0.0"),
    (2,  "/2",   "192.0.0.0"),
    (3,  "/3",   "224.0.0.0"),
    (4,  "/4",   "240.0.0.0"),
    (5,  "/5",   "248.0.0.0"),
    (6,  "/6",   "252.0.0.0"),
    (7,  "/7",   "254.0.0.0"),
    (8,  "/8",   "255.0.0.0"),
    (9,  "/9",   "255.128.0.0"),
    (10, "/10",  "255.192.0.0"),
    (11, "/11",  "255.224.0.0"),
    (12, "/12",  "255.240.0.0"),
    (13, "/13",  "255.248.0.0"),
    (14, "/14",  "255.252.0.0"),
    (15, "/15",  "255.254.0.0"),
    (16, "/16",  "255.255.0.0"),
    (17, "/17",  "255.255.128.0"),
    (18, "/18",  "255.255.192.0"),
    (19, "/19",  "255.255.224.0"),
    (20, "/20",  "255.255.240.0"),
    (21, "/21",  "255.255.248.0"),
    (22, "/22",  "255.255.252.0"),
    (23, "/23",  "255.255.254.0"),
    (24, "/24",  "255.255.255.0"),
    (25, "/25",  "255.255.255.128"),
    (26, "/26",  "255.255.255.192"),
    (27, "/27",  "255.255.255.224"),
    (28, "/28",  "255.255.255.240"),
    (29, "/29",  "255.255.255.248"),
    (30, "/30",  "255.255.255.252"),
    (31, "/31",  "255.255.255.254"),
];