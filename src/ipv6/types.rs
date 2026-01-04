use ipnet::Ipv6Net;

pub use crate::common::types::{
    IpSubnetResult, SubnetMode, SubnetResultV6 as SubnetResult,
};

pub const MAX_USABLE_SUBNETS: u128 = 131072;

pub const PREFIX_OPTIONS: &[(u8, &'static str)] = &[
    (1, "/1"), (2, "/2"), (3, "/3"), (4, "/4"), (5, "/5"), (6, "/6"), (7, "/7"), (8, "/8"),
    (9, "/9"), (10, "/10"), (11, "/11"), (12, "/12"), (13, "/13"), (14, "/14"), (15, "/15"), (16, "/16"),
    (17, "/17"), (18, "/18"), (19, "/19"), (20, "/20"), (21, "/21"), (22, "/22"), (23, "/23"), (24, "/24"),
    (25, "/25"), (26, "/26"), (27, "/27"), (28, "/28"), (29, "/29"), (30, "/30"), (31, "/31"), (32, "/32"),
    (33, "/33"), (34, "/34"), (35, "/35"), (36, "/36"), (37, "/37"), (38, "/38"), (39, "/39"), (40, "/40"),
    (41, "/41"), (42, "/42"), (43, "/43"), (44, "/44"), (45, "/45"), (46, "/46"), (47, "/47"), (48, "/48"),
    (49, "/49"), (50, "/50"), (51, "/51"), (52, "/52"), (53, "/53"), (54, "/54"), (55, "/55"), (56, "/56"),
    (57, "/57"), (58, "/58"), (59, "/59"), (60, "/60"), (61, "/61"), (62, "/62"), (63, "/63"), (64, "/64"),
];


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ipv6InputError {
    ParseError(String),
    InvalidPrefix,
    InsufficientBits,
}

// Re-export with IPv6 specialization
pub type CalculationResult = crate::common::types::CalculationResult<Ipv6Net>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HierarchyLevel {
    pub name: String,
    pub num: u32,
    pub bits: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HierarchyResult {
    pub levels: Vec<HierarchyLevel>,
    pub tree: Vec<HierarchyNode>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HierarchyNode {
    pub prefix: Ipv6Net,
    pub label: String,
    pub children: Vec<HierarchyNode>,
}