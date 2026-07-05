#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StationProfile {
    pub station_id: u16,
    pub district: u8,
    pub basin: &'static str,
    pub pump_family: &'static str,
    pub flood_floor_mm: u16,
    pub battery_floor_mv: u16,
    pub max_gate_mm: u16,
    pub radio_class: u8,
}

pub const PROFILE_0000: StationProfile = StationProfile {
    station_id: 0x3000,
    district: 0,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 900,
    battery_floor_mv: 11200,
    max_gate_mm: 700,
    radio_class: 0,
};

pub const PROFILE_0001: StationProfile = StationProfile {
    station_id: 0x3001,
    district: 1,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 937,
    battery_floor_mv: 11229,
    max_gate_mm: 717,
    radio_class: 1,
};

pub const PROFILE_0002: StationProfile = StationProfile {
    station_id: 0x3002,
    district: 2,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 974,
    battery_floor_mv: 11258,
    max_gate_mm: 734,
    radio_class: 2,
};

pub const PROFILE_0003: StationProfile = StationProfile {
    station_id: 0x3003,
    district: 3,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1011,
    battery_floor_mv: 11287,
    max_gate_mm: 751,
    radio_class: 3,
};

pub const PROFILE_0004: StationProfile = StationProfile {
    station_id: 0x3004,
    district: 4,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1048,
    battery_floor_mv: 11316,
    max_gate_mm: 768,
    radio_class: 4,
};

pub const PROFILE_0005: StationProfile = StationProfile {
    station_id: 0x3005,
    district: 5,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1085,
    battery_floor_mv: 11345,
    max_gate_mm: 785,
    radio_class: 5,
};

pub const PROFILE_0006: StationProfile = StationProfile {
    station_id: 0x3006,
    district: 6,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1122,
    battery_floor_mv: 11374,
    max_gate_mm: 802,
    radio_class: 6,
};

pub const PROFILE_0007: StationProfile = StationProfile {
    station_id: 0x3007,
    district: 7,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1159,
    battery_floor_mv: 11403,
    max_gate_mm: 819,
    radio_class: 7,
};

pub const PROFILE_0008: StationProfile = StationProfile {
    station_id: 0x3008,
    district: 8,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1196,
    battery_floor_mv: 11432,
    max_gate_mm: 836,
    radio_class: 0,
};

pub const PROFILE_0009: StationProfile = StationProfile {
    station_id: 0x3009,
    district: 9,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1233,
    battery_floor_mv: 11461,
    max_gate_mm: 853,
    radio_class: 1,
};

pub const PROFILE_0010: StationProfile = StationProfile {
    station_id: 0x300a,
    district: 10,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1270,
    battery_floor_mv: 11490,
    max_gate_mm: 870,
    radio_class: 2,
};

pub const PROFILE_0011: StationProfile = StationProfile {
    station_id: 0x300b,
    district: 11,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1307,
    battery_floor_mv: 11519,
    max_gate_mm: 887,
    radio_class: 3,
};

pub const PROFILE_0012: StationProfile = StationProfile {
    station_id: 0x300c,
    district: 12,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1344,
    battery_floor_mv: 11548,
    max_gate_mm: 904,
    radio_class: 4,
};

pub const PROFILE_0013: StationProfile = StationProfile {
    station_id: 0x300d,
    district: 13,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1381,
    battery_floor_mv: 11577,
    max_gate_mm: 921,
    radio_class: 5,
};

pub const PROFILE_0014: StationProfile = StationProfile {
    station_id: 0x300e,
    district: 14,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1418,
    battery_floor_mv: 11606,
    max_gate_mm: 938,
    radio_class: 6,
};

pub const PROFILE_0015: StationProfile = StationProfile {
    station_id: 0x300f,
    district: 15,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1455,
    battery_floor_mv: 11635,
    max_gate_mm: 955,
    radio_class: 7,
};

pub const PROFILE_0016: StationProfile = StationProfile {
    station_id: 0x3010,
    district: 16,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1492,
    battery_floor_mv: 11664,
    max_gate_mm: 972,
    radio_class: 0,
};

pub const PROFILE_0017: StationProfile = StationProfile {
    station_id: 0x3011,
    district: 17,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1529,
    battery_floor_mv: 11693,
    max_gate_mm: 989,
    radio_class: 1,
};

pub const PROFILE_0018: StationProfile = StationProfile {
    station_id: 0x3012,
    district: 18,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1566,
    battery_floor_mv: 11722,
    max_gate_mm: 1006,
    radio_class: 2,
};

pub const PROFILE_0019: StationProfile = StationProfile {
    station_id: 0x3013,
    district: 19,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1603,
    battery_floor_mv: 11751,
    max_gate_mm: 1023,
    radio_class: 3,
};

pub const PROFILE_0020: StationProfile = StationProfile {
    station_id: 0x3014,
    district: 20,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1640,
    battery_floor_mv: 11780,
    max_gate_mm: 1040,
    radio_class: 4,
};

pub const PROFILE_0021: StationProfile = StationProfile {
    station_id: 0x3015,
    district: 21,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1677,
    battery_floor_mv: 11809,
    max_gate_mm: 1057,
    radio_class: 5,
};

pub const PROFILE_0022: StationProfile = StationProfile {
    station_id: 0x3016,
    district: 22,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1714,
    battery_floor_mv: 11838,
    max_gate_mm: 1074,
    radio_class: 6,
};

pub const PROFILE_0023: StationProfile = StationProfile {
    station_id: 0x3017,
    district: 23,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1751,
    battery_floor_mv: 11867,
    max_gate_mm: 1091,
    radio_class: 7,
};

pub const PROFILE_0024: StationProfile = StationProfile {
    station_id: 0x3018,
    district: 24,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1788,
    battery_floor_mv: 11896,
    max_gate_mm: 1108,
    radio_class: 0,
};

pub const PROFILE_0025: StationProfile = StationProfile {
    station_id: 0x3019,
    district: 25,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1825,
    battery_floor_mv: 11925,
    max_gate_mm: 1125,
    radio_class: 1,
};

pub const PROFILE_0026: StationProfile = StationProfile {
    station_id: 0x301a,
    district: 26,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1862,
    battery_floor_mv: 11954,
    max_gate_mm: 1142,
    radio_class: 2,
};

pub const PROFILE_0027: StationProfile = StationProfile {
    station_id: 0x301b,
    district: 27,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1899,
    battery_floor_mv: 11983,
    max_gate_mm: 1159,
    radio_class: 3,
};

pub const PROFILE_0028: StationProfile = StationProfile {
    station_id: 0x301c,
    district: 28,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1936,
    battery_floor_mv: 12012,
    max_gate_mm: 1176,
    radio_class: 4,
};

pub const PROFILE_0029: StationProfile = StationProfile {
    station_id: 0x301d,
    district: 29,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1973,
    battery_floor_mv: 12041,
    max_gate_mm: 1193,
    radio_class: 5,
};

pub const PROFILE_0030: StationProfile = StationProfile {
    station_id: 0x301e,
    district: 30,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2010,
    battery_floor_mv: 12070,
    max_gate_mm: 1210,
    radio_class: 6,
};

pub const PROFILE_0031: StationProfile = StationProfile {
    station_id: 0x301f,
    district: 31,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2047,
    battery_floor_mv: 12099,
    max_gate_mm: 1227,
    radio_class: 7,
};

pub const PROFILE_0032: StationProfile = StationProfile {
    station_id: 0x3020,
    district: 0,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2084,
    battery_floor_mv: 12128,
    max_gate_mm: 1244,
    radio_class: 0,
};

pub const PROFILE_0033: StationProfile = StationProfile {
    station_id: 0x3021,
    district: 1,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2121,
    battery_floor_mv: 12157,
    max_gate_mm: 1261,
    radio_class: 1,
};

pub const PROFILE_0034: StationProfile = StationProfile {
    station_id: 0x3022,
    district: 2,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2158,
    battery_floor_mv: 12186,
    max_gate_mm: 1278,
    radio_class: 2,
};

pub const PROFILE_0035: StationProfile = StationProfile {
    station_id: 0x3023,
    district: 3,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2195,
    battery_floor_mv: 12215,
    max_gate_mm: 1295,
    radio_class: 3,
};

pub const PROFILE_0036: StationProfile = StationProfile {
    station_id: 0x3024,
    district: 4,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2232,
    battery_floor_mv: 12244,
    max_gate_mm: 1312,
    radio_class: 4,
};

pub const PROFILE_0037: StationProfile = StationProfile {
    station_id: 0x3025,
    district: 5,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2269,
    battery_floor_mv: 12273,
    max_gate_mm: 1329,
    radio_class: 5,
};

pub const PROFILE_0038: StationProfile = StationProfile {
    station_id: 0x3026,
    district: 6,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2306,
    battery_floor_mv: 12302,
    max_gate_mm: 1346,
    radio_class: 6,
};

pub const PROFILE_0039: StationProfile = StationProfile {
    station_id: 0x3027,
    district: 7,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2343,
    battery_floor_mv: 12331,
    max_gate_mm: 1363,
    radio_class: 7,
};

pub const PROFILE_0040: StationProfile = StationProfile {
    station_id: 0x3028,
    district: 8,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2380,
    battery_floor_mv: 12360,
    max_gate_mm: 1380,
    radio_class: 0,
};

pub const PROFILE_0041: StationProfile = StationProfile {
    station_id: 0x3029,
    district: 9,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2417,
    battery_floor_mv: 12389,
    max_gate_mm: 1397,
    radio_class: 1,
};

pub const PROFILE_0042: StationProfile = StationProfile {
    station_id: 0x302a,
    district: 10,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2454,
    battery_floor_mv: 12418,
    max_gate_mm: 1414,
    radio_class: 2,
};

pub const PROFILE_0043: StationProfile = StationProfile {
    station_id: 0x302b,
    district: 11,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2491,
    battery_floor_mv: 12447,
    max_gate_mm: 1431,
    radio_class: 3,
};

pub const PROFILE_0044: StationProfile = StationProfile {
    station_id: 0x302c,
    district: 12,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2528,
    battery_floor_mv: 12476,
    max_gate_mm: 1448,
    radio_class: 4,
};

pub const PROFILE_0045: StationProfile = StationProfile {
    station_id: 0x302d,
    district: 13,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2565,
    battery_floor_mv: 12505,
    max_gate_mm: 1465,
    radio_class: 5,
};

pub const PROFILE_0046: StationProfile = StationProfile {
    station_id: 0x302e,
    district: 14,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2602,
    battery_floor_mv: 12534,
    max_gate_mm: 1482,
    radio_class: 6,
};

pub const PROFILE_0047: StationProfile = StationProfile {
    station_id: 0x302f,
    district: 15,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2639,
    battery_floor_mv: 12563,
    max_gate_mm: 1499,
    radio_class: 7,
};

pub const PROFILE_0048: StationProfile = StationProfile {
    station_id: 0x3030,
    district: 16,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2676,
    battery_floor_mv: 12592,
    max_gate_mm: 1516,
    radio_class: 0,
};

pub const PROFILE_0049: StationProfile = StationProfile {
    station_id: 0x3031,
    district: 17,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2713,
    battery_floor_mv: 11221,
    max_gate_mm: 1533,
    radio_class: 1,
};

pub const PROFILE_0050: StationProfile = StationProfile {
    station_id: 0x3032,
    district: 18,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2750,
    battery_floor_mv: 11250,
    max_gate_mm: 1550,
    radio_class: 2,
};

pub const PROFILE_0051: StationProfile = StationProfile {
    station_id: 0x3033,
    district: 19,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2787,
    battery_floor_mv: 11279,
    max_gate_mm: 1567,
    radio_class: 3,
};

pub const PROFILE_0052: StationProfile = StationProfile {
    station_id: 0x3034,
    district: 20,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 924,
    battery_floor_mv: 11308,
    max_gate_mm: 1584,
    radio_class: 4,
};

pub const PROFILE_0053: StationProfile = StationProfile {
    station_id: 0x3035,
    district: 21,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 961,
    battery_floor_mv: 11337,
    max_gate_mm: 1601,
    radio_class: 5,
};

pub const PROFILE_0054: StationProfile = StationProfile {
    station_id: 0x3036,
    district: 22,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 998,
    battery_floor_mv: 11366,
    max_gate_mm: 1618,
    radio_class: 6,
};

pub const PROFILE_0055: StationProfile = StationProfile {
    station_id: 0x3037,
    district: 23,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1035,
    battery_floor_mv: 11395,
    max_gate_mm: 1635,
    radio_class: 7,
};

pub const PROFILE_0056: StationProfile = StationProfile {
    station_id: 0x3038,
    district: 24,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1072,
    battery_floor_mv: 11424,
    max_gate_mm: 1652,
    radio_class: 0,
};

pub const PROFILE_0057: StationProfile = StationProfile {
    station_id: 0x3039,
    district: 25,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1109,
    battery_floor_mv: 11453,
    max_gate_mm: 1669,
    radio_class: 1,
};

pub const PROFILE_0058: StationProfile = StationProfile {
    station_id: 0x303a,
    district: 26,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1146,
    battery_floor_mv: 11482,
    max_gate_mm: 1686,
    radio_class: 2,
};

pub const PROFILE_0059: StationProfile = StationProfile {
    station_id: 0x303b,
    district: 27,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1183,
    battery_floor_mv: 11511,
    max_gate_mm: 1703,
    radio_class: 3,
};

pub const PROFILE_0060: StationProfile = StationProfile {
    station_id: 0x303c,
    district: 28,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1220,
    battery_floor_mv: 11540,
    max_gate_mm: 1720,
    radio_class: 4,
};

pub const PROFILE_0061: StationProfile = StationProfile {
    station_id: 0x303d,
    district: 29,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1257,
    battery_floor_mv: 11569,
    max_gate_mm: 1737,
    radio_class: 5,
};

pub const PROFILE_0062: StationProfile = StationProfile {
    station_id: 0x303e,
    district: 30,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1294,
    battery_floor_mv: 11598,
    max_gate_mm: 1754,
    radio_class: 6,
};

pub const PROFILE_0063: StationProfile = StationProfile {
    station_id: 0x303f,
    district: 31,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1331,
    battery_floor_mv: 11627,
    max_gate_mm: 1771,
    radio_class: 7,
};

pub const PROFILE_0064: StationProfile = StationProfile {
    station_id: 0x3040,
    district: 0,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1368,
    battery_floor_mv: 11656,
    max_gate_mm: 1788,
    radio_class: 0,
};

pub const PROFILE_0065: StationProfile = StationProfile {
    station_id: 0x3041,
    district: 1,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1405,
    battery_floor_mv: 11685,
    max_gate_mm: 1805,
    radio_class: 1,
};

pub const PROFILE_0066: StationProfile = StationProfile {
    station_id: 0x3042,
    district: 2,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1442,
    battery_floor_mv: 11714,
    max_gate_mm: 1822,
    radio_class: 2,
};

pub const PROFILE_0067: StationProfile = StationProfile {
    station_id: 0x3043,
    district: 3,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1479,
    battery_floor_mv: 11743,
    max_gate_mm: 1839,
    radio_class: 3,
};

pub const PROFILE_0068: StationProfile = StationProfile {
    station_id: 0x3044,
    district: 4,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1516,
    battery_floor_mv: 11772,
    max_gate_mm: 1856,
    radio_class: 4,
};

pub const PROFILE_0069: StationProfile = StationProfile {
    station_id: 0x3045,
    district: 5,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1553,
    battery_floor_mv: 11801,
    max_gate_mm: 1873,
    radio_class: 5,
};

pub const PROFILE_0070: StationProfile = StationProfile {
    station_id: 0x3046,
    district: 6,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1590,
    battery_floor_mv: 11830,
    max_gate_mm: 1890,
    radio_class: 6,
};

pub const PROFILE_0071: StationProfile = StationProfile {
    station_id: 0x3047,
    district: 7,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1627,
    battery_floor_mv: 11859,
    max_gate_mm: 1907,
    radio_class: 7,
};

pub const PROFILE_0072: StationProfile = StationProfile {
    station_id: 0x3048,
    district: 8,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1664,
    battery_floor_mv: 11888,
    max_gate_mm: 1924,
    radio_class: 0,
};

pub const PROFILE_0073: StationProfile = StationProfile {
    station_id: 0x3049,
    district: 9,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1701,
    battery_floor_mv: 11917,
    max_gate_mm: 1941,
    radio_class: 1,
};

pub const PROFILE_0074: StationProfile = StationProfile {
    station_id: 0x304a,
    district: 10,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1738,
    battery_floor_mv: 11946,
    max_gate_mm: 1958,
    radio_class: 2,
};

pub const PROFILE_0075: StationProfile = StationProfile {
    station_id: 0x304b,
    district: 11,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1775,
    battery_floor_mv: 11975,
    max_gate_mm: 1975,
    radio_class: 3,
};

pub const PROFILE_0076: StationProfile = StationProfile {
    station_id: 0x304c,
    district: 12,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1812,
    battery_floor_mv: 12004,
    max_gate_mm: 1992,
    radio_class: 4,
};

pub const PROFILE_0077: StationProfile = StationProfile {
    station_id: 0x304d,
    district: 13,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1849,
    battery_floor_mv: 12033,
    max_gate_mm: 2009,
    radio_class: 5,
};

pub const PROFILE_0078: StationProfile = StationProfile {
    station_id: 0x304e,
    district: 14,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1886,
    battery_floor_mv: 12062,
    max_gate_mm: 2026,
    radio_class: 6,
};

pub const PROFILE_0079: StationProfile = StationProfile {
    station_id: 0x304f,
    district: 15,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1923,
    battery_floor_mv: 12091,
    max_gate_mm: 2043,
    radio_class: 7,
};

pub const PROFILE_0080: StationProfile = StationProfile {
    station_id: 0x3050,
    district: 16,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1960,
    battery_floor_mv: 12120,
    max_gate_mm: 2060,
    radio_class: 0,
};

pub const PROFILE_0081: StationProfile = StationProfile {
    station_id: 0x3051,
    district: 17,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1997,
    battery_floor_mv: 12149,
    max_gate_mm: 2077,
    radio_class: 1,
};

pub const PROFILE_0082: StationProfile = StationProfile {
    station_id: 0x3052,
    district: 18,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2034,
    battery_floor_mv: 12178,
    max_gate_mm: 2094,
    radio_class: 2,
};

pub const PROFILE_0083: StationProfile = StationProfile {
    station_id: 0x3053,
    district: 19,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2071,
    battery_floor_mv: 12207,
    max_gate_mm: 2111,
    radio_class: 3,
};

pub const PROFILE_0084: StationProfile = StationProfile {
    station_id: 0x3054,
    district: 20,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2108,
    battery_floor_mv: 12236,
    max_gate_mm: 2128,
    radio_class: 4,
};

pub const PROFILE_0085: StationProfile = StationProfile {
    station_id: 0x3055,
    district: 21,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2145,
    battery_floor_mv: 12265,
    max_gate_mm: 2145,
    radio_class: 5,
};

pub const PROFILE_0086: StationProfile = StationProfile {
    station_id: 0x3056,
    district: 22,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2182,
    battery_floor_mv: 12294,
    max_gate_mm: 2162,
    radio_class: 6,
};

pub const PROFILE_0087: StationProfile = StationProfile {
    station_id: 0x3057,
    district: 23,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2219,
    battery_floor_mv: 12323,
    max_gate_mm: 2179,
    radio_class: 7,
};

pub const PROFILE_0088: StationProfile = StationProfile {
    station_id: 0x3058,
    district: 24,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2256,
    battery_floor_mv: 12352,
    max_gate_mm: 2196,
    radio_class: 0,
};

pub const PROFILE_0089: StationProfile = StationProfile {
    station_id: 0x3059,
    district: 25,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2293,
    battery_floor_mv: 12381,
    max_gate_mm: 2213,
    radio_class: 1,
};

pub const PROFILE_0090: StationProfile = StationProfile {
    station_id: 0x305a,
    district: 26,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2330,
    battery_floor_mv: 12410,
    max_gate_mm: 2230,
    radio_class: 2,
};

pub const PROFILE_0091: StationProfile = StationProfile {
    station_id: 0x305b,
    district: 27,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2367,
    battery_floor_mv: 12439,
    max_gate_mm: 2247,
    radio_class: 3,
};

pub const PROFILE_0092: StationProfile = StationProfile {
    station_id: 0x305c,
    district: 28,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2404,
    battery_floor_mv: 12468,
    max_gate_mm: 2264,
    radio_class: 4,
};

pub const PROFILE_0093: StationProfile = StationProfile {
    station_id: 0x305d,
    district: 29,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2441,
    battery_floor_mv: 12497,
    max_gate_mm: 2281,
    radio_class: 5,
};

pub const PROFILE_0094: StationProfile = StationProfile {
    station_id: 0x305e,
    district: 30,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2478,
    battery_floor_mv: 12526,
    max_gate_mm: 2298,
    radio_class: 6,
};

pub const PROFILE_0095: StationProfile = StationProfile {
    station_id: 0x305f,
    district: 31,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2515,
    battery_floor_mv: 12555,
    max_gate_mm: 2315,
    radio_class: 7,
};

pub const PROFILE_0096: StationProfile = StationProfile {
    station_id: 0x3060,
    district: 0,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2552,
    battery_floor_mv: 12584,
    max_gate_mm: 2332,
    radio_class: 0,
};

pub const PROFILE_0097: StationProfile = StationProfile {
    station_id: 0x3061,
    district: 1,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2589,
    battery_floor_mv: 11213,
    max_gate_mm: 2349,
    radio_class: 1,
};

pub const PROFILE_0098: StationProfile = StationProfile {
    station_id: 0x3062,
    district: 2,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2626,
    battery_floor_mv: 11242,
    max_gate_mm: 2366,
    radio_class: 2,
};

pub const PROFILE_0099: StationProfile = StationProfile {
    station_id: 0x3063,
    district: 3,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2663,
    battery_floor_mv: 11271,
    max_gate_mm: 2383,
    radio_class: 3,
};

pub const PROFILE_0100: StationProfile = StationProfile {
    station_id: 0x3064,
    district: 4,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2700,
    battery_floor_mv: 11300,
    max_gate_mm: 2400,
    radio_class: 4,
};

pub const PROFILE_0101: StationProfile = StationProfile {
    station_id: 0x3065,
    district: 5,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2737,
    battery_floor_mv: 11329,
    max_gate_mm: 2417,
    radio_class: 5,
};

pub const PROFILE_0102: StationProfile = StationProfile {
    station_id: 0x3066,
    district: 6,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2774,
    battery_floor_mv: 11358,
    max_gate_mm: 2434,
    radio_class: 6,
};

pub const PROFILE_0103: StationProfile = StationProfile {
    station_id: 0x3067,
    district: 7,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 911,
    battery_floor_mv: 11387,
    max_gate_mm: 2451,
    radio_class: 7,
};

pub const PROFILE_0104: StationProfile = StationProfile {
    station_id: 0x3068,
    district: 8,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 948,
    battery_floor_mv: 11416,
    max_gate_mm: 2468,
    radio_class: 0,
};

pub const PROFILE_0105: StationProfile = StationProfile {
    station_id: 0x3069,
    district: 9,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 985,
    battery_floor_mv: 11445,
    max_gate_mm: 2485,
    radio_class: 1,
};

pub const PROFILE_0106: StationProfile = StationProfile {
    station_id: 0x306a,
    district: 10,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1022,
    battery_floor_mv: 11474,
    max_gate_mm: 2502,
    radio_class: 2,
};

pub const PROFILE_0107: StationProfile = StationProfile {
    station_id: 0x306b,
    district: 11,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1059,
    battery_floor_mv: 11503,
    max_gate_mm: 2519,
    radio_class: 3,
};

pub const PROFILE_0108: StationProfile = StationProfile {
    station_id: 0x306c,
    district: 12,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1096,
    battery_floor_mv: 11532,
    max_gate_mm: 2536,
    radio_class: 4,
};

pub const PROFILE_0109: StationProfile = StationProfile {
    station_id: 0x306d,
    district: 13,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1133,
    battery_floor_mv: 11561,
    max_gate_mm: 2553,
    radio_class: 5,
};

pub const PROFILE_0110: StationProfile = StationProfile {
    station_id: 0x306e,
    district: 14,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1170,
    battery_floor_mv: 11590,
    max_gate_mm: 2570,
    radio_class: 6,
};

pub const PROFILE_0111: StationProfile = StationProfile {
    station_id: 0x306f,
    district: 15,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1207,
    battery_floor_mv: 11619,
    max_gate_mm: 2587,
    radio_class: 7,
};

pub const PROFILE_0112: StationProfile = StationProfile {
    station_id: 0x3070,
    district: 16,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1244,
    battery_floor_mv: 11648,
    max_gate_mm: 2604,
    radio_class: 0,
};

pub const PROFILE_0113: StationProfile = StationProfile {
    station_id: 0x3071,
    district: 17,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1281,
    battery_floor_mv: 11677,
    max_gate_mm: 2621,
    radio_class: 1,
};

pub const PROFILE_0114: StationProfile = StationProfile {
    station_id: 0x3072,
    district: 18,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1318,
    battery_floor_mv: 11706,
    max_gate_mm: 2638,
    radio_class: 2,
};

pub const PROFILE_0115: StationProfile = StationProfile {
    station_id: 0x3073,
    district: 19,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1355,
    battery_floor_mv: 11735,
    max_gate_mm: 2655,
    radio_class: 3,
};

pub const PROFILE_0116: StationProfile = StationProfile {
    station_id: 0x3074,
    district: 20,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1392,
    battery_floor_mv: 11764,
    max_gate_mm: 2672,
    radio_class: 4,
};

pub const PROFILE_0117: StationProfile = StationProfile {
    station_id: 0x3075,
    district: 21,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1429,
    battery_floor_mv: 11793,
    max_gate_mm: 2689,
    radio_class: 5,
};

pub const PROFILE_0118: StationProfile = StationProfile {
    station_id: 0x3076,
    district: 22,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1466,
    battery_floor_mv: 11822,
    max_gate_mm: 2706,
    radio_class: 6,
};

pub const PROFILE_0119: StationProfile = StationProfile {
    station_id: 0x3077,
    district: 23,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1503,
    battery_floor_mv: 11851,
    max_gate_mm: 2723,
    radio_class: 7,
};

pub const PROFILE_0120: StationProfile = StationProfile {
    station_id: 0x3078,
    district: 24,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1540,
    battery_floor_mv: 11880,
    max_gate_mm: 2740,
    radio_class: 0,
};

pub const PROFILE_0121: StationProfile = StationProfile {
    station_id: 0x3079,
    district: 25,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1577,
    battery_floor_mv: 11909,
    max_gate_mm: 2757,
    radio_class: 1,
};

pub const PROFILE_0122: StationProfile = StationProfile {
    station_id: 0x307a,
    district: 26,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1614,
    battery_floor_mv: 11938,
    max_gate_mm: 2774,
    radio_class: 2,
};

pub const PROFILE_0123: StationProfile = StationProfile {
    station_id: 0x307b,
    district: 27,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1651,
    battery_floor_mv: 11967,
    max_gate_mm: 2791,
    radio_class: 3,
};

pub const PROFILE_0124: StationProfile = StationProfile {
    station_id: 0x307c,
    district: 28,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1688,
    battery_floor_mv: 11996,
    max_gate_mm: 2808,
    radio_class: 4,
};

pub const PROFILE_0125: StationProfile = StationProfile {
    station_id: 0x307d,
    district: 29,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1725,
    battery_floor_mv: 12025,
    max_gate_mm: 2825,
    radio_class: 5,
};

pub const PROFILE_0126: StationProfile = StationProfile {
    station_id: 0x307e,
    district: 30,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1762,
    battery_floor_mv: 12054,
    max_gate_mm: 2842,
    radio_class: 6,
};

pub const PROFILE_0127: StationProfile = StationProfile {
    station_id: 0x307f,
    district: 31,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1799,
    battery_floor_mv: 12083,
    max_gate_mm: 2859,
    radio_class: 7,
};

pub const PROFILE_0128: StationProfile = StationProfile {
    station_id: 0x3080,
    district: 0,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1836,
    battery_floor_mv: 12112,
    max_gate_mm: 2876,
    radio_class: 0,
};

pub const PROFILE_0129: StationProfile = StationProfile {
    station_id: 0x3081,
    district: 1,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1873,
    battery_floor_mv: 12141,
    max_gate_mm: 2893,
    radio_class: 1,
};

pub const PROFILE_0130: StationProfile = StationProfile {
    station_id: 0x3082,
    district: 2,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1910,
    battery_floor_mv: 12170,
    max_gate_mm: 2910,
    radio_class: 2,
};

pub const PROFILE_0131: StationProfile = StationProfile {
    station_id: 0x3083,
    district: 3,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1947,
    battery_floor_mv: 12199,
    max_gate_mm: 2927,
    radio_class: 3,
};

pub const PROFILE_0132: StationProfile = StationProfile {
    station_id: 0x3084,
    district: 4,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1984,
    battery_floor_mv: 12228,
    max_gate_mm: 2944,
    radio_class: 4,
};

pub const PROFILE_0133: StationProfile = StationProfile {
    station_id: 0x3085,
    district: 5,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2021,
    battery_floor_mv: 12257,
    max_gate_mm: 2961,
    radio_class: 5,
};

pub const PROFILE_0134: StationProfile = StationProfile {
    station_id: 0x3086,
    district: 6,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2058,
    battery_floor_mv: 12286,
    max_gate_mm: 2978,
    radio_class: 6,
};

pub const PROFILE_0135: StationProfile = StationProfile {
    station_id: 0x3087,
    district: 7,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2095,
    battery_floor_mv: 12315,
    max_gate_mm: 2995,
    radio_class: 7,
};

pub const PROFILE_0136: StationProfile = StationProfile {
    station_id: 0x3088,
    district: 8,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2132,
    battery_floor_mv: 12344,
    max_gate_mm: 712,
    radio_class: 0,
};

pub const PROFILE_0137: StationProfile = StationProfile {
    station_id: 0x3089,
    district: 9,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2169,
    battery_floor_mv: 12373,
    max_gate_mm: 729,
    radio_class: 1,
};

pub const PROFILE_0138: StationProfile = StationProfile {
    station_id: 0x308a,
    district: 10,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2206,
    battery_floor_mv: 12402,
    max_gate_mm: 746,
    radio_class: 2,
};

pub const PROFILE_0139: StationProfile = StationProfile {
    station_id: 0x308b,
    district: 11,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2243,
    battery_floor_mv: 12431,
    max_gate_mm: 763,
    radio_class: 3,
};

pub const PROFILE_0140: StationProfile = StationProfile {
    station_id: 0x308c,
    district: 12,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2280,
    battery_floor_mv: 12460,
    max_gate_mm: 780,
    radio_class: 4,
};

pub const PROFILE_0141: StationProfile = StationProfile {
    station_id: 0x308d,
    district: 13,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2317,
    battery_floor_mv: 12489,
    max_gate_mm: 797,
    radio_class: 5,
};

pub const PROFILE_0142: StationProfile = StationProfile {
    station_id: 0x308e,
    district: 14,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2354,
    battery_floor_mv: 12518,
    max_gate_mm: 814,
    radio_class: 6,
};

pub const PROFILE_0143: StationProfile = StationProfile {
    station_id: 0x308f,
    district: 15,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2391,
    battery_floor_mv: 12547,
    max_gate_mm: 831,
    radio_class: 7,
};

pub const PROFILE_0144: StationProfile = StationProfile {
    station_id: 0x3090,
    district: 16,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2428,
    battery_floor_mv: 12576,
    max_gate_mm: 848,
    radio_class: 0,
};

pub const PROFILE_0145: StationProfile = StationProfile {
    station_id: 0x3091,
    district: 17,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2465,
    battery_floor_mv: 11205,
    max_gate_mm: 865,
    radio_class: 1,
};

pub const PROFILE_0146: StationProfile = StationProfile {
    station_id: 0x3092,
    district: 18,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2502,
    battery_floor_mv: 11234,
    max_gate_mm: 882,
    radio_class: 2,
};

pub const PROFILE_0147: StationProfile = StationProfile {
    station_id: 0x3093,
    district: 19,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2539,
    battery_floor_mv: 11263,
    max_gate_mm: 899,
    radio_class: 3,
};

pub const PROFILE_0148: StationProfile = StationProfile {
    station_id: 0x3094,
    district: 20,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2576,
    battery_floor_mv: 11292,
    max_gate_mm: 916,
    radio_class: 4,
};

pub const PROFILE_0149: StationProfile = StationProfile {
    station_id: 0x3095,
    district: 21,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2613,
    battery_floor_mv: 11321,
    max_gate_mm: 933,
    radio_class: 5,
};

pub const PROFILE_0150: StationProfile = StationProfile {
    station_id: 0x3096,
    district: 22,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2650,
    battery_floor_mv: 11350,
    max_gate_mm: 950,
    radio_class: 6,
};

pub const PROFILE_0151: StationProfile = StationProfile {
    station_id: 0x3097,
    district: 23,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2687,
    battery_floor_mv: 11379,
    max_gate_mm: 967,
    radio_class: 7,
};

pub const PROFILE_0152: StationProfile = StationProfile {
    station_id: 0x3098,
    district: 24,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2724,
    battery_floor_mv: 11408,
    max_gate_mm: 984,
    radio_class: 0,
};

pub const PROFILE_0153: StationProfile = StationProfile {
    station_id: 0x3099,
    district: 25,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2761,
    battery_floor_mv: 11437,
    max_gate_mm: 1001,
    radio_class: 1,
};

pub const PROFILE_0154: StationProfile = StationProfile {
    station_id: 0x309a,
    district: 26,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2798,
    battery_floor_mv: 11466,
    max_gate_mm: 1018,
    radio_class: 2,
};

pub const PROFILE_0155: StationProfile = StationProfile {
    station_id: 0x309b,
    district: 27,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 935,
    battery_floor_mv: 11495,
    max_gate_mm: 1035,
    radio_class: 3,
};

pub const PROFILE_0156: StationProfile = StationProfile {
    station_id: 0x309c,
    district: 28,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 972,
    battery_floor_mv: 11524,
    max_gate_mm: 1052,
    radio_class: 4,
};

pub const PROFILE_0157: StationProfile = StationProfile {
    station_id: 0x309d,
    district: 29,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1009,
    battery_floor_mv: 11553,
    max_gate_mm: 1069,
    radio_class: 5,
};

pub const PROFILE_0158: StationProfile = StationProfile {
    station_id: 0x309e,
    district: 30,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1046,
    battery_floor_mv: 11582,
    max_gate_mm: 1086,
    radio_class: 6,
};

pub const PROFILE_0159: StationProfile = StationProfile {
    station_id: 0x309f,
    district: 31,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1083,
    battery_floor_mv: 11611,
    max_gate_mm: 1103,
    radio_class: 7,
};

pub const PROFILE_0160: StationProfile = StationProfile {
    station_id: 0x30a0,
    district: 0,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1120,
    battery_floor_mv: 11640,
    max_gate_mm: 1120,
    radio_class: 0,
};

pub const PROFILE_0161: StationProfile = StationProfile {
    station_id: 0x30a1,
    district: 1,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1157,
    battery_floor_mv: 11669,
    max_gate_mm: 1137,
    radio_class: 1,
};

pub const PROFILE_0162: StationProfile = StationProfile {
    station_id: 0x30a2,
    district: 2,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1194,
    battery_floor_mv: 11698,
    max_gate_mm: 1154,
    radio_class: 2,
};

pub const PROFILE_0163: StationProfile = StationProfile {
    station_id: 0x30a3,
    district: 3,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1231,
    battery_floor_mv: 11727,
    max_gate_mm: 1171,
    radio_class: 3,
};

pub const PROFILE_0164: StationProfile = StationProfile {
    station_id: 0x30a4,
    district: 4,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1268,
    battery_floor_mv: 11756,
    max_gate_mm: 1188,
    radio_class: 4,
};

pub const PROFILE_0165: StationProfile = StationProfile {
    station_id: 0x30a5,
    district: 5,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1305,
    battery_floor_mv: 11785,
    max_gate_mm: 1205,
    radio_class: 5,
};

pub const PROFILE_0166: StationProfile = StationProfile {
    station_id: 0x30a6,
    district: 6,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1342,
    battery_floor_mv: 11814,
    max_gate_mm: 1222,
    radio_class: 6,
};

pub const PROFILE_0167: StationProfile = StationProfile {
    station_id: 0x30a7,
    district: 7,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1379,
    battery_floor_mv: 11843,
    max_gate_mm: 1239,
    radio_class: 7,
};

pub const PROFILE_0168: StationProfile = StationProfile {
    station_id: 0x30a8,
    district: 8,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1416,
    battery_floor_mv: 11872,
    max_gate_mm: 1256,
    radio_class: 0,
};

pub const PROFILE_0169: StationProfile = StationProfile {
    station_id: 0x30a9,
    district: 9,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1453,
    battery_floor_mv: 11901,
    max_gate_mm: 1273,
    radio_class: 1,
};

pub const PROFILE_0170: StationProfile = StationProfile {
    station_id: 0x30aa,
    district: 10,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1490,
    battery_floor_mv: 11930,
    max_gate_mm: 1290,
    radio_class: 2,
};

pub const PROFILE_0171: StationProfile = StationProfile {
    station_id: 0x30ab,
    district: 11,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1527,
    battery_floor_mv: 11959,
    max_gate_mm: 1307,
    radio_class: 3,
};

pub const PROFILE_0172: StationProfile = StationProfile {
    station_id: 0x30ac,
    district: 12,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1564,
    battery_floor_mv: 11988,
    max_gate_mm: 1324,
    radio_class: 4,
};

pub const PROFILE_0173: StationProfile = StationProfile {
    station_id: 0x30ad,
    district: 13,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1601,
    battery_floor_mv: 12017,
    max_gate_mm: 1341,
    radio_class: 5,
};

pub const PROFILE_0174: StationProfile = StationProfile {
    station_id: 0x30ae,
    district: 14,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1638,
    battery_floor_mv: 12046,
    max_gate_mm: 1358,
    radio_class: 6,
};

pub const PROFILE_0175: StationProfile = StationProfile {
    station_id: 0x30af,
    district: 15,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1675,
    battery_floor_mv: 12075,
    max_gate_mm: 1375,
    radio_class: 7,
};

pub const PROFILE_0176: StationProfile = StationProfile {
    station_id: 0x30b0,
    district: 16,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1712,
    battery_floor_mv: 12104,
    max_gate_mm: 1392,
    radio_class: 0,
};

pub const PROFILE_0177: StationProfile = StationProfile {
    station_id: 0x30b1,
    district: 17,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1749,
    battery_floor_mv: 12133,
    max_gate_mm: 1409,
    radio_class: 1,
};

pub const PROFILE_0178: StationProfile = StationProfile {
    station_id: 0x30b2,
    district: 18,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1786,
    battery_floor_mv: 12162,
    max_gate_mm: 1426,
    radio_class: 2,
};

pub const PROFILE_0179: StationProfile = StationProfile {
    station_id: 0x30b3,
    district: 19,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1823,
    battery_floor_mv: 12191,
    max_gate_mm: 1443,
    radio_class: 3,
};

pub const PROFILE_0180: StationProfile = StationProfile {
    station_id: 0x30b4,
    district: 20,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1860,
    battery_floor_mv: 12220,
    max_gate_mm: 1460,
    radio_class: 4,
};

pub const PROFILE_0181: StationProfile = StationProfile {
    station_id: 0x30b5,
    district: 21,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1897,
    battery_floor_mv: 12249,
    max_gate_mm: 1477,
    radio_class: 5,
};

pub const PROFILE_0182: StationProfile = StationProfile {
    station_id: 0x30b6,
    district: 22,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1934,
    battery_floor_mv: 12278,
    max_gate_mm: 1494,
    radio_class: 6,
};

pub const PROFILE_0183: StationProfile = StationProfile {
    station_id: 0x30b7,
    district: 23,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1971,
    battery_floor_mv: 12307,
    max_gate_mm: 1511,
    radio_class: 7,
};

pub const PROFILE_0184: StationProfile = StationProfile {
    station_id: 0x30b8,
    district: 24,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2008,
    battery_floor_mv: 12336,
    max_gate_mm: 1528,
    radio_class: 0,
};

pub const PROFILE_0185: StationProfile = StationProfile {
    station_id: 0x30b9,
    district: 25,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2045,
    battery_floor_mv: 12365,
    max_gate_mm: 1545,
    radio_class: 1,
};

pub const PROFILE_0186: StationProfile = StationProfile {
    station_id: 0x30ba,
    district: 26,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2082,
    battery_floor_mv: 12394,
    max_gate_mm: 1562,
    radio_class: 2,
};

pub const PROFILE_0187: StationProfile = StationProfile {
    station_id: 0x30bb,
    district: 27,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2119,
    battery_floor_mv: 12423,
    max_gate_mm: 1579,
    radio_class: 3,
};

pub const PROFILE_0188: StationProfile = StationProfile {
    station_id: 0x30bc,
    district: 28,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2156,
    battery_floor_mv: 12452,
    max_gate_mm: 1596,
    radio_class: 4,
};

pub const PROFILE_0189: StationProfile = StationProfile {
    station_id: 0x30bd,
    district: 29,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2193,
    battery_floor_mv: 12481,
    max_gate_mm: 1613,
    radio_class: 5,
};

pub const PROFILE_0190: StationProfile = StationProfile {
    station_id: 0x30be,
    district: 30,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2230,
    battery_floor_mv: 12510,
    max_gate_mm: 1630,
    radio_class: 6,
};

pub const PROFILE_0191: StationProfile = StationProfile {
    station_id: 0x30bf,
    district: 31,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2267,
    battery_floor_mv: 12539,
    max_gate_mm: 1647,
    radio_class: 7,
};

pub const PROFILE_0192: StationProfile = StationProfile {
    station_id: 0x30c0,
    district: 0,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2304,
    battery_floor_mv: 12568,
    max_gate_mm: 1664,
    radio_class: 0,
};

pub const PROFILE_0193: StationProfile = StationProfile {
    station_id: 0x30c1,
    district: 1,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2341,
    battery_floor_mv: 12597,
    max_gate_mm: 1681,
    radio_class: 1,
};

pub const PROFILE_0194: StationProfile = StationProfile {
    station_id: 0x30c2,
    district: 2,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2378,
    battery_floor_mv: 11226,
    max_gate_mm: 1698,
    radio_class: 2,
};

pub const PROFILE_0195: StationProfile = StationProfile {
    station_id: 0x30c3,
    district: 3,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2415,
    battery_floor_mv: 11255,
    max_gate_mm: 1715,
    radio_class: 3,
};

pub const PROFILE_0196: StationProfile = StationProfile {
    station_id: 0x30c4,
    district: 4,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2452,
    battery_floor_mv: 11284,
    max_gate_mm: 1732,
    radio_class: 4,
};

pub const PROFILE_0197: StationProfile = StationProfile {
    station_id: 0x30c5,
    district: 5,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2489,
    battery_floor_mv: 11313,
    max_gate_mm: 1749,
    radio_class: 5,
};

pub const PROFILE_0198: StationProfile = StationProfile {
    station_id: 0x30c6,
    district: 6,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2526,
    battery_floor_mv: 11342,
    max_gate_mm: 1766,
    radio_class: 6,
};

pub const PROFILE_0199: StationProfile = StationProfile {
    station_id: 0x30c7,
    district: 7,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2563,
    battery_floor_mv: 11371,
    max_gate_mm: 1783,
    radio_class: 7,
};

pub const PROFILE_0200: StationProfile = StationProfile {
    station_id: 0x30c8,
    district: 8,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2600,
    battery_floor_mv: 11400,
    max_gate_mm: 1800,
    radio_class: 0,
};

pub const PROFILE_0201: StationProfile = StationProfile {
    station_id: 0x30c9,
    district: 9,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2637,
    battery_floor_mv: 11429,
    max_gate_mm: 1817,
    radio_class: 1,
};

pub const PROFILE_0202: StationProfile = StationProfile {
    station_id: 0x30ca,
    district: 10,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2674,
    battery_floor_mv: 11458,
    max_gate_mm: 1834,
    radio_class: 2,
};

pub const PROFILE_0203: StationProfile = StationProfile {
    station_id: 0x30cb,
    district: 11,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2711,
    battery_floor_mv: 11487,
    max_gate_mm: 1851,
    radio_class: 3,
};

pub const PROFILE_0204: StationProfile = StationProfile {
    station_id: 0x30cc,
    district: 12,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2748,
    battery_floor_mv: 11516,
    max_gate_mm: 1868,
    radio_class: 4,
};

pub const PROFILE_0205: StationProfile = StationProfile {
    station_id: 0x30cd,
    district: 13,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2785,
    battery_floor_mv: 11545,
    max_gate_mm: 1885,
    radio_class: 5,
};

pub const PROFILE_0206: StationProfile = StationProfile {
    station_id: 0x30ce,
    district: 14,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 922,
    battery_floor_mv: 11574,
    max_gate_mm: 1902,
    radio_class: 6,
};

pub const PROFILE_0207: StationProfile = StationProfile {
    station_id: 0x30cf,
    district: 15,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 959,
    battery_floor_mv: 11603,
    max_gate_mm: 1919,
    radio_class: 7,
};

pub const PROFILE_0208: StationProfile = StationProfile {
    station_id: 0x30d0,
    district: 16,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 996,
    battery_floor_mv: 11632,
    max_gate_mm: 1936,
    radio_class: 0,
};

pub const PROFILE_0209: StationProfile = StationProfile {
    station_id: 0x30d1,
    district: 17,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1033,
    battery_floor_mv: 11661,
    max_gate_mm: 1953,
    radio_class: 1,
};

pub const PROFILE_0210: StationProfile = StationProfile {
    station_id: 0x30d2,
    district: 18,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1070,
    battery_floor_mv: 11690,
    max_gate_mm: 1970,
    radio_class: 2,
};

pub const PROFILE_0211: StationProfile = StationProfile {
    station_id: 0x30d3,
    district: 19,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1107,
    battery_floor_mv: 11719,
    max_gate_mm: 1987,
    radio_class: 3,
};

pub const PROFILE_0212: StationProfile = StationProfile {
    station_id: 0x30d4,
    district: 20,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1144,
    battery_floor_mv: 11748,
    max_gate_mm: 2004,
    radio_class: 4,
};

pub const PROFILE_0213: StationProfile = StationProfile {
    station_id: 0x30d5,
    district: 21,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1181,
    battery_floor_mv: 11777,
    max_gate_mm: 2021,
    radio_class: 5,
};

pub const PROFILE_0214: StationProfile = StationProfile {
    station_id: 0x30d6,
    district: 22,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1218,
    battery_floor_mv: 11806,
    max_gate_mm: 2038,
    radio_class: 6,
};

pub const PROFILE_0215: StationProfile = StationProfile {
    station_id: 0x30d7,
    district: 23,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1255,
    battery_floor_mv: 11835,
    max_gate_mm: 2055,
    radio_class: 7,
};

pub const PROFILE_0216: StationProfile = StationProfile {
    station_id: 0x30d8,
    district: 24,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1292,
    battery_floor_mv: 11864,
    max_gate_mm: 2072,
    radio_class: 0,
};

pub const PROFILE_0217: StationProfile = StationProfile {
    station_id: 0x30d9,
    district: 25,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1329,
    battery_floor_mv: 11893,
    max_gate_mm: 2089,
    radio_class: 1,
};

pub const PROFILE_0218: StationProfile = StationProfile {
    station_id: 0x30da,
    district: 26,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1366,
    battery_floor_mv: 11922,
    max_gate_mm: 2106,
    radio_class: 2,
};

pub const PROFILE_0219: StationProfile = StationProfile {
    station_id: 0x30db,
    district: 27,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1403,
    battery_floor_mv: 11951,
    max_gate_mm: 2123,
    radio_class: 3,
};

pub const PROFILE_0220: StationProfile = StationProfile {
    station_id: 0x30dc,
    district: 28,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1440,
    battery_floor_mv: 11980,
    max_gate_mm: 2140,
    radio_class: 4,
};

pub const PROFILE_0221: StationProfile = StationProfile {
    station_id: 0x30dd,
    district: 29,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1477,
    battery_floor_mv: 12009,
    max_gate_mm: 2157,
    radio_class: 5,
};

pub const PROFILE_0222: StationProfile = StationProfile {
    station_id: 0x30de,
    district: 30,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1514,
    battery_floor_mv: 12038,
    max_gate_mm: 2174,
    radio_class: 6,
};

pub const PROFILE_0223: StationProfile = StationProfile {
    station_id: 0x30df,
    district: 31,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1551,
    battery_floor_mv: 12067,
    max_gate_mm: 2191,
    radio_class: 7,
};

pub const PROFILE_0224: StationProfile = StationProfile {
    station_id: 0x30e0,
    district: 0,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1588,
    battery_floor_mv: 12096,
    max_gate_mm: 2208,
    radio_class: 0,
};

pub const PROFILE_0225: StationProfile = StationProfile {
    station_id: 0x30e1,
    district: 1,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1625,
    battery_floor_mv: 12125,
    max_gate_mm: 2225,
    radio_class: 1,
};

pub const PROFILE_0226: StationProfile = StationProfile {
    station_id: 0x30e2,
    district: 2,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1662,
    battery_floor_mv: 12154,
    max_gate_mm: 2242,
    radio_class: 2,
};

pub const PROFILE_0227: StationProfile = StationProfile {
    station_id: 0x30e3,
    district: 3,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1699,
    battery_floor_mv: 12183,
    max_gate_mm: 2259,
    radio_class: 3,
};

pub const PROFILE_0228: StationProfile = StationProfile {
    station_id: 0x30e4,
    district: 4,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1736,
    battery_floor_mv: 12212,
    max_gate_mm: 2276,
    radio_class: 4,
};

pub const PROFILE_0229: StationProfile = StationProfile {
    station_id: 0x30e5,
    district: 5,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1773,
    battery_floor_mv: 12241,
    max_gate_mm: 2293,
    radio_class: 5,
};

pub const PROFILE_0230: StationProfile = StationProfile {
    station_id: 0x30e6,
    district: 6,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1810,
    battery_floor_mv: 12270,
    max_gate_mm: 2310,
    radio_class: 6,
};

pub const PROFILE_0231: StationProfile = StationProfile {
    station_id: 0x30e7,
    district: 7,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1847,
    battery_floor_mv: 12299,
    max_gate_mm: 2327,
    radio_class: 7,
};

pub const PROFILE_0232: StationProfile = StationProfile {
    station_id: 0x30e8,
    district: 8,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1884,
    battery_floor_mv: 12328,
    max_gate_mm: 2344,
    radio_class: 0,
};

pub const PROFILE_0233: StationProfile = StationProfile {
    station_id: 0x30e9,
    district: 9,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1921,
    battery_floor_mv: 12357,
    max_gate_mm: 2361,
    radio_class: 1,
};

pub const PROFILE_0234: StationProfile = StationProfile {
    station_id: 0x30ea,
    district: 10,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1958,
    battery_floor_mv: 12386,
    max_gate_mm: 2378,
    radio_class: 2,
};

pub const PROFILE_0235: StationProfile = StationProfile {
    station_id: 0x30eb,
    district: 11,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1995,
    battery_floor_mv: 12415,
    max_gate_mm: 2395,
    radio_class: 3,
};

pub const PROFILE_0236: StationProfile = StationProfile {
    station_id: 0x30ec,
    district: 12,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2032,
    battery_floor_mv: 12444,
    max_gate_mm: 2412,
    radio_class: 4,
};

pub const PROFILE_0237: StationProfile = StationProfile {
    station_id: 0x30ed,
    district: 13,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2069,
    battery_floor_mv: 12473,
    max_gate_mm: 2429,
    radio_class: 5,
};

pub const PROFILE_0238: StationProfile = StationProfile {
    station_id: 0x30ee,
    district: 14,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2106,
    battery_floor_mv: 12502,
    max_gate_mm: 2446,
    radio_class: 6,
};

pub const PROFILE_0239: StationProfile = StationProfile {
    station_id: 0x30ef,
    district: 15,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2143,
    battery_floor_mv: 12531,
    max_gate_mm: 2463,
    radio_class: 7,
};

pub const PROFILE_0240: StationProfile = StationProfile {
    station_id: 0x30f0,
    district: 16,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2180,
    battery_floor_mv: 12560,
    max_gate_mm: 2480,
    radio_class: 0,
};

pub const PROFILE_0241: StationProfile = StationProfile {
    station_id: 0x30f1,
    district: 17,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2217,
    battery_floor_mv: 12589,
    max_gate_mm: 2497,
    radio_class: 1,
};

pub const PROFILE_0242: StationProfile = StationProfile {
    station_id: 0x30f2,
    district: 18,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2254,
    battery_floor_mv: 11218,
    max_gate_mm: 2514,
    radio_class: 2,
};

pub const PROFILE_0243: StationProfile = StationProfile {
    station_id: 0x30f3,
    district: 19,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2291,
    battery_floor_mv: 11247,
    max_gate_mm: 2531,
    radio_class: 3,
};

pub const PROFILE_0244: StationProfile = StationProfile {
    station_id: 0x30f4,
    district: 20,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2328,
    battery_floor_mv: 11276,
    max_gate_mm: 2548,
    radio_class: 4,
};

pub const PROFILE_0245: StationProfile = StationProfile {
    station_id: 0x30f5,
    district: 21,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2365,
    battery_floor_mv: 11305,
    max_gate_mm: 2565,
    radio_class: 5,
};

pub const PROFILE_0246: StationProfile = StationProfile {
    station_id: 0x30f6,
    district: 22,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2402,
    battery_floor_mv: 11334,
    max_gate_mm: 2582,
    radio_class: 6,
};

pub const PROFILE_0247: StationProfile = StationProfile {
    station_id: 0x30f7,
    district: 23,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2439,
    battery_floor_mv: 11363,
    max_gate_mm: 2599,
    radio_class: 7,
};

pub const PROFILE_0248: StationProfile = StationProfile {
    station_id: 0x30f8,
    district: 24,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2476,
    battery_floor_mv: 11392,
    max_gate_mm: 2616,
    radio_class: 0,
};

pub const PROFILE_0249: StationProfile = StationProfile {
    station_id: 0x30f9,
    district: 25,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2513,
    battery_floor_mv: 11421,
    max_gate_mm: 2633,
    radio_class: 1,
};

pub const PROFILE_0250: StationProfile = StationProfile {
    station_id: 0x30fa,
    district: 26,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2550,
    battery_floor_mv: 11450,
    max_gate_mm: 2650,
    radio_class: 2,
};

pub const PROFILE_0251: StationProfile = StationProfile {
    station_id: 0x30fb,
    district: 27,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2587,
    battery_floor_mv: 11479,
    max_gate_mm: 2667,
    radio_class: 3,
};

pub const PROFILE_0252: StationProfile = StationProfile {
    station_id: 0x30fc,
    district: 28,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2624,
    battery_floor_mv: 11508,
    max_gate_mm: 2684,
    radio_class: 4,
};

pub const PROFILE_0253: StationProfile = StationProfile {
    station_id: 0x30fd,
    district: 29,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2661,
    battery_floor_mv: 11537,
    max_gate_mm: 2701,
    radio_class: 5,
};

pub const PROFILE_0254: StationProfile = StationProfile {
    station_id: 0x30fe,
    district: 30,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2698,
    battery_floor_mv: 11566,
    max_gate_mm: 2718,
    radio_class: 6,
};

pub const PROFILE_0255: StationProfile = StationProfile {
    station_id: 0x30ff,
    district: 31,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2735,
    battery_floor_mv: 11595,
    max_gate_mm: 2735,
    radio_class: 7,
};

pub const PROFILE_0256: StationProfile = StationProfile {
    station_id: 0x3100,
    district: 0,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2772,
    battery_floor_mv: 11624,
    max_gate_mm: 2752,
    radio_class: 0,
};

pub const PROFILE_0257: StationProfile = StationProfile {
    station_id: 0x3101,
    district: 1,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 909,
    battery_floor_mv: 11653,
    max_gate_mm: 2769,
    radio_class: 1,
};

pub const PROFILE_0258: StationProfile = StationProfile {
    station_id: 0x3102,
    district: 2,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 946,
    battery_floor_mv: 11682,
    max_gate_mm: 2786,
    radio_class: 2,
};

pub const PROFILE_0259: StationProfile = StationProfile {
    station_id: 0x3103,
    district: 3,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 983,
    battery_floor_mv: 11711,
    max_gate_mm: 2803,
    radio_class: 3,
};

pub const PROFILE_0260: StationProfile = StationProfile {
    station_id: 0x3104,
    district: 4,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1020,
    battery_floor_mv: 11740,
    max_gate_mm: 2820,
    radio_class: 4,
};

pub const PROFILE_0261: StationProfile = StationProfile {
    station_id: 0x3105,
    district: 5,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1057,
    battery_floor_mv: 11769,
    max_gate_mm: 2837,
    radio_class: 5,
};

pub const PROFILE_0262: StationProfile = StationProfile {
    station_id: 0x3106,
    district: 6,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1094,
    battery_floor_mv: 11798,
    max_gate_mm: 2854,
    radio_class: 6,
};

pub const PROFILE_0263: StationProfile = StationProfile {
    station_id: 0x3107,
    district: 7,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1131,
    battery_floor_mv: 11827,
    max_gate_mm: 2871,
    radio_class: 7,
};

pub const PROFILE_0264: StationProfile = StationProfile {
    station_id: 0x3108,
    district: 8,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1168,
    battery_floor_mv: 11856,
    max_gate_mm: 2888,
    radio_class: 0,
};

pub const PROFILE_0265: StationProfile = StationProfile {
    station_id: 0x3109,
    district: 9,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1205,
    battery_floor_mv: 11885,
    max_gate_mm: 2905,
    radio_class: 1,
};

pub const PROFILE_0266: StationProfile = StationProfile {
    station_id: 0x310a,
    district: 10,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1242,
    battery_floor_mv: 11914,
    max_gate_mm: 2922,
    radio_class: 2,
};

pub const PROFILE_0267: StationProfile = StationProfile {
    station_id: 0x310b,
    district: 11,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1279,
    battery_floor_mv: 11943,
    max_gate_mm: 2939,
    radio_class: 3,
};

pub const PROFILE_0268: StationProfile = StationProfile {
    station_id: 0x310c,
    district: 12,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1316,
    battery_floor_mv: 11972,
    max_gate_mm: 2956,
    radio_class: 4,
};

pub const PROFILE_0269: StationProfile = StationProfile {
    station_id: 0x310d,
    district: 13,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1353,
    battery_floor_mv: 12001,
    max_gate_mm: 2973,
    radio_class: 5,
};

pub const PROFILE_0270: StationProfile = StationProfile {
    station_id: 0x310e,
    district: 14,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1390,
    battery_floor_mv: 12030,
    max_gate_mm: 2990,
    radio_class: 6,
};

pub const PROFILE_0271: StationProfile = StationProfile {
    station_id: 0x310f,
    district: 15,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1427,
    battery_floor_mv: 12059,
    max_gate_mm: 707,
    radio_class: 7,
};

pub const PROFILE_0272: StationProfile = StationProfile {
    station_id: 0x3110,
    district: 16,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1464,
    battery_floor_mv: 12088,
    max_gate_mm: 724,
    radio_class: 0,
};

pub const PROFILE_0273: StationProfile = StationProfile {
    station_id: 0x3111,
    district: 17,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1501,
    battery_floor_mv: 12117,
    max_gate_mm: 741,
    radio_class: 1,
};

pub const PROFILE_0274: StationProfile = StationProfile {
    station_id: 0x3112,
    district: 18,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1538,
    battery_floor_mv: 12146,
    max_gate_mm: 758,
    radio_class: 2,
};

pub const PROFILE_0275: StationProfile = StationProfile {
    station_id: 0x3113,
    district: 19,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1575,
    battery_floor_mv: 12175,
    max_gate_mm: 775,
    radio_class: 3,
};

pub const PROFILE_0276: StationProfile = StationProfile {
    station_id: 0x3114,
    district: 20,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1612,
    battery_floor_mv: 12204,
    max_gate_mm: 792,
    radio_class: 4,
};

pub const PROFILE_0277: StationProfile = StationProfile {
    station_id: 0x3115,
    district: 21,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1649,
    battery_floor_mv: 12233,
    max_gate_mm: 809,
    radio_class: 5,
};

pub const PROFILE_0278: StationProfile = StationProfile {
    station_id: 0x3116,
    district: 22,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1686,
    battery_floor_mv: 12262,
    max_gate_mm: 826,
    radio_class: 6,
};

pub const PROFILE_0279: StationProfile = StationProfile {
    station_id: 0x3117,
    district: 23,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1723,
    battery_floor_mv: 12291,
    max_gate_mm: 843,
    radio_class: 7,
};

pub const PROFILE_0280: StationProfile = StationProfile {
    station_id: 0x3118,
    district: 24,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1760,
    battery_floor_mv: 12320,
    max_gate_mm: 860,
    radio_class: 0,
};

pub const PROFILE_0281: StationProfile = StationProfile {
    station_id: 0x3119,
    district: 25,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1797,
    battery_floor_mv: 12349,
    max_gate_mm: 877,
    radio_class: 1,
};

pub const PROFILE_0282: StationProfile = StationProfile {
    station_id: 0x311a,
    district: 26,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1834,
    battery_floor_mv: 12378,
    max_gate_mm: 894,
    radio_class: 2,
};

pub const PROFILE_0283: StationProfile = StationProfile {
    station_id: 0x311b,
    district: 27,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1871,
    battery_floor_mv: 12407,
    max_gate_mm: 911,
    radio_class: 3,
};

pub const PROFILE_0284: StationProfile = StationProfile {
    station_id: 0x311c,
    district: 28,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1908,
    battery_floor_mv: 12436,
    max_gate_mm: 928,
    radio_class: 4,
};

pub const PROFILE_0285: StationProfile = StationProfile {
    station_id: 0x311d,
    district: 29,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1945,
    battery_floor_mv: 12465,
    max_gate_mm: 945,
    radio_class: 5,
};

pub const PROFILE_0286: StationProfile = StationProfile {
    station_id: 0x311e,
    district: 30,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1982,
    battery_floor_mv: 12494,
    max_gate_mm: 962,
    radio_class: 6,
};

pub const PROFILE_0287: StationProfile = StationProfile {
    station_id: 0x311f,
    district: 31,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2019,
    battery_floor_mv: 12523,
    max_gate_mm: 979,
    radio_class: 7,
};

pub const PROFILE_0288: StationProfile = StationProfile {
    station_id: 0x3120,
    district: 0,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2056,
    battery_floor_mv: 12552,
    max_gate_mm: 996,
    radio_class: 0,
};

pub const PROFILE_0289: StationProfile = StationProfile {
    station_id: 0x3121,
    district: 1,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2093,
    battery_floor_mv: 12581,
    max_gate_mm: 1013,
    radio_class: 1,
};

pub const PROFILE_0290: StationProfile = StationProfile {
    station_id: 0x3122,
    district: 2,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2130,
    battery_floor_mv: 11210,
    max_gate_mm: 1030,
    radio_class: 2,
};

pub const PROFILE_0291: StationProfile = StationProfile {
    station_id: 0x3123,
    district: 3,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2167,
    battery_floor_mv: 11239,
    max_gate_mm: 1047,
    radio_class: 3,
};

pub const PROFILE_0292: StationProfile = StationProfile {
    station_id: 0x3124,
    district: 4,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2204,
    battery_floor_mv: 11268,
    max_gate_mm: 1064,
    radio_class: 4,
};

pub const PROFILE_0293: StationProfile = StationProfile {
    station_id: 0x3125,
    district: 5,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2241,
    battery_floor_mv: 11297,
    max_gate_mm: 1081,
    radio_class: 5,
};

pub const PROFILE_0294: StationProfile = StationProfile {
    station_id: 0x3126,
    district: 6,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2278,
    battery_floor_mv: 11326,
    max_gate_mm: 1098,
    radio_class: 6,
};

pub const PROFILE_0295: StationProfile = StationProfile {
    station_id: 0x3127,
    district: 7,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2315,
    battery_floor_mv: 11355,
    max_gate_mm: 1115,
    radio_class: 7,
};

pub const PROFILE_0296: StationProfile = StationProfile {
    station_id: 0x3128,
    district: 8,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2352,
    battery_floor_mv: 11384,
    max_gate_mm: 1132,
    radio_class: 0,
};

pub const PROFILE_0297: StationProfile = StationProfile {
    station_id: 0x3129,
    district: 9,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2389,
    battery_floor_mv: 11413,
    max_gate_mm: 1149,
    radio_class: 1,
};

pub const PROFILE_0298: StationProfile = StationProfile {
    station_id: 0x312a,
    district: 10,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2426,
    battery_floor_mv: 11442,
    max_gate_mm: 1166,
    radio_class: 2,
};

pub const PROFILE_0299: StationProfile = StationProfile {
    station_id: 0x312b,
    district: 11,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2463,
    battery_floor_mv: 11471,
    max_gate_mm: 1183,
    radio_class: 3,
};

pub const PROFILE_0300: StationProfile = StationProfile {
    station_id: 0x312c,
    district: 12,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2500,
    battery_floor_mv: 11500,
    max_gate_mm: 1200,
    radio_class: 4,
};

pub const PROFILE_0301: StationProfile = StationProfile {
    station_id: 0x312d,
    district: 13,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2537,
    battery_floor_mv: 11529,
    max_gate_mm: 1217,
    radio_class: 5,
};

pub const PROFILE_0302: StationProfile = StationProfile {
    station_id: 0x312e,
    district: 14,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2574,
    battery_floor_mv: 11558,
    max_gate_mm: 1234,
    radio_class: 6,
};

pub const PROFILE_0303: StationProfile = StationProfile {
    station_id: 0x312f,
    district: 15,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2611,
    battery_floor_mv: 11587,
    max_gate_mm: 1251,
    radio_class: 7,
};

pub const PROFILE_0304: StationProfile = StationProfile {
    station_id: 0x3130,
    district: 16,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2648,
    battery_floor_mv: 11616,
    max_gate_mm: 1268,
    radio_class: 0,
};

pub const PROFILE_0305: StationProfile = StationProfile {
    station_id: 0x3131,
    district: 17,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2685,
    battery_floor_mv: 11645,
    max_gate_mm: 1285,
    radio_class: 1,
};

pub const PROFILE_0306: StationProfile = StationProfile {
    station_id: 0x3132,
    district: 18,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2722,
    battery_floor_mv: 11674,
    max_gate_mm: 1302,
    radio_class: 2,
};

pub const PROFILE_0307: StationProfile = StationProfile {
    station_id: 0x3133,
    district: 19,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2759,
    battery_floor_mv: 11703,
    max_gate_mm: 1319,
    radio_class: 3,
};

pub const PROFILE_0308: StationProfile = StationProfile {
    station_id: 0x3134,
    district: 20,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2796,
    battery_floor_mv: 11732,
    max_gate_mm: 1336,
    radio_class: 4,
};

pub const PROFILE_0309: StationProfile = StationProfile {
    station_id: 0x3135,
    district: 21,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 933,
    battery_floor_mv: 11761,
    max_gate_mm: 1353,
    radio_class: 5,
};

pub const PROFILE_0310: StationProfile = StationProfile {
    station_id: 0x3136,
    district: 22,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 970,
    battery_floor_mv: 11790,
    max_gate_mm: 1370,
    radio_class: 6,
};

pub const PROFILE_0311: StationProfile = StationProfile {
    station_id: 0x3137,
    district: 23,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1007,
    battery_floor_mv: 11819,
    max_gate_mm: 1387,
    radio_class: 7,
};

pub const PROFILE_0312: StationProfile = StationProfile {
    station_id: 0x3138,
    district: 24,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1044,
    battery_floor_mv: 11848,
    max_gate_mm: 1404,
    radio_class: 0,
};

pub const PROFILE_0313: StationProfile = StationProfile {
    station_id: 0x3139,
    district: 25,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1081,
    battery_floor_mv: 11877,
    max_gate_mm: 1421,
    radio_class: 1,
};

pub const PROFILE_0314: StationProfile = StationProfile {
    station_id: 0x313a,
    district: 26,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1118,
    battery_floor_mv: 11906,
    max_gate_mm: 1438,
    radio_class: 2,
};

pub const PROFILE_0315: StationProfile = StationProfile {
    station_id: 0x313b,
    district: 27,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1155,
    battery_floor_mv: 11935,
    max_gate_mm: 1455,
    radio_class: 3,
};

pub const PROFILE_0316: StationProfile = StationProfile {
    station_id: 0x313c,
    district: 28,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1192,
    battery_floor_mv: 11964,
    max_gate_mm: 1472,
    radio_class: 4,
};

pub const PROFILE_0317: StationProfile = StationProfile {
    station_id: 0x313d,
    district: 29,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1229,
    battery_floor_mv: 11993,
    max_gate_mm: 1489,
    radio_class: 5,
};

pub const PROFILE_0318: StationProfile = StationProfile {
    station_id: 0x313e,
    district: 30,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1266,
    battery_floor_mv: 12022,
    max_gate_mm: 1506,
    radio_class: 6,
};

pub const PROFILE_0319: StationProfile = StationProfile {
    station_id: 0x313f,
    district: 31,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1303,
    battery_floor_mv: 12051,
    max_gate_mm: 1523,
    radio_class: 7,
};

pub const PROFILE_0320: StationProfile = StationProfile {
    station_id: 0x3140,
    district: 0,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1340,
    battery_floor_mv: 12080,
    max_gate_mm: 1540,
    radio_class: 0,
};

pub const PROFILE_0321: StationProfile = StationProfile {
    station_id: 0x3141,
    district: 1,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1377,
    battery_floor_mv: 12109,
    max_gate_mm: 1557,
    radio_class: 1,
};

pub const PROFILE_0322: StationProfile = StationProfile {
    station_id: 0x3142,
    district: 2,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1414,
    battery_floor_mv: 12138,
    max_gate_mm: 1574,
    radio_class: 2,
};

pub const PROFILE_0323: StationProfile = StationProfile {
    station_id: 0x3143,
    district: 3,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1451,
    battery_floor_mv: 12167,
    max_gate_mm: 1591,
    radio_class: 3,
};

pub const PROFILE_0324: StationProfile = StationProfile {
    station_id: 0x3144,
    district: 4,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1488,
    battery_floor_mv: 12196,
    max_gate_mm: 1608,
    radio_class: 4,
};

pub const PROFILE_0325: StationProfile = StationProfile {
    station_id: 0x3145,
    district: 5,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1525,
    battery_floor_mv: 12225,
    max_gate_mm: 1625,
    radio_class: 5,
};

pub const PROFILE_0326: StationProfile = StationProfile {
    station_id: 0x3146,
    district: 6,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1562,
    battery_floor_mv: 12254,
    max_gate_mm: 1642,
    radio_class: 6,
};

pub const PROFILE_0327: StationProfile = StationProfile {
    station_id: 0x3147,
    district: 7,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1599,
    battery_floor_mv: 12283,
    max_gate_mm: 1659,
    radio_class: 7,
};

pub const PROFILE_0328: StationProfile = StationProfile {
    station_id: 0x3148,
    district: 8,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1636,
    battery_floor_mv: 12312,
    max_gate_mm: 1676,
    radio_class: 0,
};

pub const PROFILE_0329: StationProfile = StationProfile {
    station_id: 0x3149,
    district: 9,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1673,
    battery_floor_mv: 12341,
    max_gate_mm: 1693,
    radio_class: 1,
};

pub const PROFILE_0330: StationProfile = StationProfile {
    station_id: 0x314a,
    district: 10,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1710,
    battery_floor_mv: 12370,
    max_gate_mm: 1710,
    radio_class: 2,
};

pub const PROFILE_0331: StationProfile = StationProfile {
    station_id: 0x314b,
    district: 11,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1747,
    battery_floor_mv: 12399,
    max_gate_mm: 1727,
    radio_class: 3,
};

pub const PROFILE_0332: StationProfile = StationProfile {
    station_id: 0x314c,
    district: 12,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1784,
    battery_floor_mv: 12428,
    max_gate_mm: 1744,
    radio_class: 4,
};

pub const PROFILE_0333: StationProfile = StationProfile {
    station_id: 0x314d,
    district: 13,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1821,
    battery_floor_mv: 12457,
    max_gate_mm: 1761,
    radio_class: 5,
};

pub const PROFILE_0334: StationProfile = StationProfile {
    station_id: 0x314e,
    district: 14,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1858,
    battery_floor_mv: 12486,
    max_gate_mm: 1778,
    radio_class: 6,
};

pub const PROFILE_0335: StationProfile = StationProfile {
    station_id: 0x314f,
    district: 15,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1895,
    battery_floor_mv: 12515,
    max_gate_mm: 1795,
    radio_class: 7,
};

pub const PROFILE_0336: StationProfile = StationProfile {
    station_id: 0x3150,
    district: 16,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1932,
    battery_floor_mv: 12544,
    max_gate_mm: 1812,
    radio_class: 0,
};

pub const PROFILE_0337: StationProfile = StationProfile {
    station_id: 0x3151,
    district: 17,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1969,
    battery_floor_mv: 12573,
    max_gate_mm: 1829,
    radio_class: 1,
};

pub const PROFILE_0338: StationProfile = StationProfile {
    station_id: 0x3152,
    district: 18,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2006,
    battery_floor_mv: 11202,
    max_gate_mm: 1846,
    radio_class: 2,
};

pub const PROFILE_0339: StationProfile = StationProfile {
    station_id: 0x3153,
    district: 19,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2043,
    battery_floor_mv: 11231,
    max_gate_mm: 1863,
    radio_class: 3,
};

pub const PROFILE_0340: StationProfile = StationProfile {
    station_id: 0x3154,
    district: 20,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2080,
    battery_floor_mv: 11260,
    max_gate_mm: 1880,
    radio_class: 4,
};

pub const PROFILE_0341: StationProfile = StationProfile {
    station_id: 0x3155,
    district: 21,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2117,
    battery_floor_mv: 11289,
    max_gate_mm: 1897,
    radio_class: 5,
};

pub const PROFILE_0342: StationProfile = StationProfile {
    station_id: 0x3156,
    district: 22,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2154,
    battery_floor_mv: 11318,
    max_gate_mm: 1914,
    radio_class: 6,
};

pub const PROFILE_0343: StationProfile = StationProfile {
    station_id: 0x3157,
    district: 23,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2191,
    battery_floor_mv: 11347,
    max_gate_mm: 1931,
    radio_class: 7,
};

pub const PROFILE_0344: StationProfile = StationProfile {
    station_id: 0x3158,
    district: 24,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2228,
    battery_floor_mv: 11376,
    max_gate_mm: 1948,
    radio_class: 0,
};

pub const PROFILE_0345: StationProfile = StationProfile {
    station_id: 0x3159,
    district: 25,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2265,
    battery_floor_mv: 11405,
    max_gate_mm: 1965,
    radio_class: 1,
};

pub const PROFILE_0346: StationProfile = StationProfile {
    station_id: 0x315a,
    district: 26,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2302,
    battery_floor_mv: 11434,
    max_gate_mm: 1982,
    radio_class: 2,
};

pub const PROFILE_0347: StationProfile = StationProfile {
    station_id: 0x315b,
    district: 27,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2339,
    battery_floor_mv: 11463,
    max_gate_mm: 1999,
    radio_class: 3,
};

pub const PROFILE_0348: StationProfile = StationProfile {
    station_id: 0x315c,
    district: 28,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2376,
    battery_floor_mv: 11492,
    max_gate_mm: 2016,
    radio_class: 4,
};

pub const PROFILE_0349: StationProfile = StationProfile {
    station_id: 0x315d,
    district: 29,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2413,
    battery_floor_mv: 11521,
    max_gate_mm: 2033,
    radio_class: 5,
};

pub const PROFILE_0350: StationProfile = StationProfile {
    station_id: 0x315e,
    district: 30,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2450,
    battery_floor_mv: 11550,
    max_gate_mm: 2050,
    radio_class: 6,
};

pub const PROFILE_0351: StationProfile = StationProfile {
    station_id: 0x315f,
    district: 31,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2487,
    battery_floor_mv: 11579,
    max_gate_mm: 2067,
    radio_class: 7,
};

pub const PROFILE_0352: StationProfile = StationProfile {
    station_id: 0x3160,
    district: 0,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2524,
    battery_floor_mv: 11608,
    max_gate_mm: 2084,
    radio_class: 0,
};

pub const PROFILE_0353: StationProfile = StationProfile {
    station_id: 0x3161,
    district: 1,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2561,
    battery_floor_mv: 11637,
    max_gate_mm: 2101,
    radio_class: 1,
};

pub const PROFILE_0354: StationProfile = StationProfile {
    station_id: 0x3162,
    district: 2,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2598,
    battery_floor_mv: 11666,
    max_gate_mm: 2118,
    radio_class: 2,
};

pub const PROFILE_0355: StationProfile = StationProfile {
    station_id: 0x3163,
    district: 3,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2635,
    battery_floor_mv: 11695,
    max_gate_mm: 2135,
    radio_class: 3,
};

pub const PROFILE_0356: StationProfile = StationProfile {
    station_id: 0x3164,
    district: 4,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2672,
    battery_floor_mv: 11724,
    max_gate_mm: 2152,
    radio_class: 4,
};

pub const PROFILE_0357: StationProfile = StationProfile {
    station_id: 0x3165,
    district: 5,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2709,
    battery_floor_mv: 11753,
    max_gate_mm: 2169,
    radio_class: 5,
};

pub const PROFILE_0358: StationProfile = StationProfile {
    station_id: 0x3166,
    district: 6,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2746,
    battery_floor_mv: 11782,
    max_gate_mm: 2186,
    radio_class: 6,
};

pub const PROFILE_0359: StationProfile = StationProfile {
    station_id: 0x3167,
    district: 7,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2783,
    battery_floor_mv: 11811,
    max_gate_mm: 2203,
    radio_class: 7,
};

pub const PROFILE_0360: StationProfile = StationProfile {
    station_id: 0x3168,
    district: 8,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 920,
    battery_floor_mv: 11840,
    max_gate_mm: 2220,
    radio_class: 0,
};

pub const PROFILE_0361: StationProfile = StationProfile {
    station_id: 0x3169,
    district: 9,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 957,
    battery_floor_mv: 11869,
    max_gate_mm: 2237,
    radio_class: 1,
};

pub const PROFILE_0362: StationProfile = StationProfile {
    station_id: 0x316a,
    district: 10,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 994,
    battery_floor_mv: 11898,
    max_gate_mm: 2254,
    radio_class: 2,
};

pub const PROFILE_0363: StationProfile = StationProfile {
    station_id: 0x316b,
    district: 11,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1031,
    battery_floor_mv: 11927,
    max_gate_mm: 2271,
    radio_class: 3,
};

pub const PROFILE_0364: StationProfile = StationProfile {
    station_id: 0x316c,
    district: 12,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1068,
    battery_floor_mv: 11956,
    max_gate_mm: 2288,
    radio_class: 4,
};

pub const PROFILE_0365: StationProfile = StationProfile {
    station_id: 0x316d,
    district: 13,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1105,
    battery_floor_mv: 11985,
    max_gate_mm: 2305,
    radio_class: 5,
};

pub const PROFILE_0366: StationProfile = StationProfile {
    station_id: 0x316e,
    district: 14,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1142,
    battery_floor_mv: 12014,
    max_gate_mm: 2322,
    radio_class: 6,
};

pub const PROFILE_0367: StationProfile = StationProfile {
    station_id: 0x316f,
    district: 15,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1179,
    battery_floor_mv: 12043,
    max_gate_mm: 2339,
    radio_class: 7,
};

pub const PROFILE_0368: StationProfile = StationProfile {
    station_id: 0x3170,
    district: 16,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1216,
    battery_floor_mv: 12072,
    max_gate_mm: 2356,
    radio_class: 0,
};

pub const PROFILE_0369: StationProfile = StationProfile {
    station_id: 0x3171,
    district: 17,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1253,
    battery_floor_mv: 12101,
    max_gate_mm: 2373,
    radio_class: 1,
};

pub const PROFILE_0370: StationProfile = StationProfile {
    station_id: 0x3172,
    district: 18,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1290,
    battery_floor_mv: 12130,
    max_gate_mm: 2390,
    radio_class: 2,
};

pub const PROFILE_0371: StationProfile = StationProfile {
    station_id: 0x3173,
    district: 19,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1327,
    battery_floor_mv: 12159,
    max_gate_mm: 2407,
    radio_class: 3,
};

pub const PROFILE_0372: StationProfile = StationProfile {
    station_id: 0x3174,
    district: 20,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1364,
    battery_floor_mv: 12188,
    max_gate_mm: 2424,
    radio_class: 4,
};

pub const PROFILE_0373: StationProfile = StationProfile {
    station_id: 0x3175,
    district: 21,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1401,
    battery_floor_mv: 12217,
    max_gate_mm: 2441,
    radio_class: 5,
};

pub const PROFILE_0374: StationProfile = StationProfile {
    station_id: 0x3176,
    district: 22,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1438,
    battery_floor_mv: 12246,
    max_gate_mm: 2458,
    radio_class: 6,
};

pub const PROFILE_0375: StationProfile = StationProfile {
    station_id: 0x3177,
    district: 23,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1475,
    battery_floor_mv: 12275,
    max_gate_mm: 2475,
    radio_class: 7,
};

pub const PROFILE_0376: StationProfile = StationProfile {
    station_id: 0x3178,
    district: 24,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1512,
    battery_floor_mv: 12304,
    max_gate_mm: 2492,
    radio_class: 0,
};

pub const PROFILE_0377: StationProfile = StationProfile {
    station_id: 0x3179,
    district: 25,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1549,
    battery_floor_mv: 12333,
    max_gate_mm: 2509,
    radio_class: 1,
};

pub const PROFILE_0378: StationProfile = StationProfile {
    station_id: 0x317a,
    district: 26,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1586,
    battery_floor_mv: 12362,
    max_gate_mm: 2526,
    radio_class: 2,
};

pub const PROFILE_0379: StationProfile = StationProfile {
    station_id: 0x317b,
    district: 27,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1623,
    battery_floor_mv: 12391,
    max_gate_mm: 2543,
    radio_class: 3,
};

pub const PROFILE_0380: StationProfile = StationProfile {
    station_id: 0x317c,
    district: 28,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1660,
    battery_floor_mv: 12420,
    max_gate_mm: 2560,
    radio_class: 4,
};

pub const PROFILE_0381: StationProfile = StationProfile {
    station_id: 0x317d,
    district: 29,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1697,
    battery_floor_mv: 12449,
    max_gate_mm: 2577,
    radio_class: 5,
};

pub const PROFILE_0382: StationProfile = StationProfile {
    station_id: 0x317e,
    district: 30,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1734,
    battery_floor_mv: 12478,
    max_gate_mm: 2594,
    radio_class: 6,
};

pub const PROFILE_0383: StationProfile = StationProfile {
    station_id: 0x317f,
    district: 31,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1771,
    battery_floor_mv: 12507,
    max_gate_mm: 2611,
    radio_class: 7,
};

pub const PROFILE_0384: StationProfile = StationProfile {
    station_id: 0x3180,
    district: 0,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1808,
    battery_floor_mv: 12536,
    max_gate_mm: 2628,
    radio_class: 0,
};

pub const PROFILE_0385: StationProfile = StationProfile {
    station_id: 0x3181,
    district: 1,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1845,
    battery_floor_mv: 12565,
    max_gate_mm: 2645,
    radio_class: 1,
};

pub const PROFILE_0386: StationProfile = StationProfile {
    station_id: 0x3182,
    district: 2,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1882,
    battery_floor_mv: 12594,
    max_gate_mm: 2662,
    radio_class: 2,
};

pub const PROFILE_0387: StationProfile = StationProfile {
    station_id: 0x3183,
    district: 3,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1919,
    battery_floor_mv: 11223,
    max_gate_mm: 2679,
    radio_class: 3,
};

pub const PROFILE_0388: StationProfile = StationProfile {
    station_id: 0x3184,
    district: 4,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1956,
    battery_floor_mv: 11252,
    max_gate_mm: 2696,
    radio_class: 4,
};

pub const PROFILE_0389: StationProfile = StationProfile {
    station_id: 0x3185,
    district: 5,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1993,
    battery_floor_mv: 11281,
    max_gate_mm: 2713,
    radio_class: 5,
};

pub const PROFILE_0390: StationProfile = StationProfile {
    station_id: 0x3186,
    district: 6,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2030,
    battery_floor_mv: 11310,
    max_gate_mm: 2730,
    radio_class: 6,
};

pub const PROFILE_0391: StationProfile = StationProfile {
    station_id: 0x3187,
    district: 7,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2067,
    battery_floor_mv: 11339,
    max_gate_mm: 2747,
    radio_class: 7,
};

pub const PROFILE_0392: StationProfile = StationProfile {
    station_id: 0x3188,
    district: 8,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2104,
    battery_floor_mv: 11368,
    max_gate_mm: 2764,
    radio_class: 0,
};

pub const PROFILE_0393: StationProfile = StationProfile {
    station_id: 0x3189,
    district: 9,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2141,
    battery_floor_mv: 11397,
    max_gate_mm: 2781,
    radio_class: 1,
};

pub const PROFILE_0394: StationProfile = StationProfile {
    station_id: 0x318a,
    district: 10,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2178,
    battery_floor_mv: 11426,
    max_gate_mm: 2798,
    radio_class: 2,
};

pub const PROFILE_0395: StationProfile = StationProfile {
    station_id: 0x318b,
    district: 11,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2215,
    battery_floor_mv: 11455,
    max_gate_mm: 2815,
    radio_class: 3,
};

pub const PROFILE_0396: StationProfile = StationProfile {
    station_id: 0x318c,
    district: 12,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2252,
    battery_floor_mv: 11484,
    max_gate_mm: 2832,
    radio_class: 4,
};

pub const PROFILE_0397: StationProfile = StationProfile {
    station_id: 0x318d,
    district: 13,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2289,
    battery_floor_mv: 11513,
    max_gate_mm: 2849,
    radio_class: 5,
};

pub const PROFILE_0398: StationProfile = StationProfile {
    station_id: 0x318e,
    district: 14,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2326,
    battery_floor_mv: 11542,
    max_gate_mm: 2866,
    radio_class: 6,
};

pub const PROFILE_0399: StationProfile = StationProfile {
    station_id: 0x318f,
    district: 15,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2363,
    battery_floor_mv: 11571,
    max_gate_mm: 2883,
    radio_class: 7,
};

pub const PROFILE_0400: StationProfile = StationProfile {
    station_id: 0x3190,
    district: 16,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2400,
    battery_floor_mv: 11600,
    max_gate_mm: 2900,
    radio_class: 0,
};

pub const PROFILE_0401: StationProfile = StationProfile {
    station_id: 0x3191,
    district: 17,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2437,
    battery_floor_mv: 11629,
    max_gate_mm: 2917,
    radio_class: 1,
};

pub const PROFILE_0402: StationProfile = StationProfile {
    station_id: 0x3192,
    district: 18,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2474,
    battery_floor_mv: 11658,
    max_gate_mm: 2934,
    radio_class: 2,
};

pub const PROFILE_0403: StationProfile = StationProfile {
    station_id: 0x3193,
    district: 19,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2511,
    battery_floor_mv: 11687,
    max_gate_mm: 2951,
    radio_class: 3,
};

pub const PROFILE_0404: StationProfile = StationProfile {
    station_id: 0x3194,
    district: 20,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2548,
    battery_floor_mv: 11716,
    max_gate_mm: 2968,
    radio_class: 4,
};

pub const PROFILE_0405: StationProfile = StationProfile {
    station_id: 0x3195,
    district: 21,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2585,
    battery_floor_mv: 11745,
    max_gate_mm: 2985,
    radio_class: 5,
};

pub const PROFILE_0406: StationProfile = StationProfile {
    station_id: 0x3196,
    district: 22,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2622,
    battery_floor_mv: 11774,
    max_gate_mm: 702,
    radio_class: 6,
};

pub const PROFILE_0407: StationProfile = StationProfile {
    station_id: 0x3197,
    district: 23,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2659,
    battery_floor_mv: 11803,
    max_gate_mm: 719,
    radio_class: 7,
};

pub const PROFILE_0408: StationProfile = StationProfile {
    station_id: 0x3198,
    district: 24,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2696,
    battery_floor_mv: 11832,
    max_gate_mm: 736,
    radio_class: 0,
};

pub const PROFILE_0409: StationProfile = StationProfile {
    station_id: 0x3199,
    district: 25,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2733,
    battery_floor_mv: 11861,
    max_gate_mm: 753,
    radio_class: 1,
};

pub const PROFILE_0410: StationProfile = StationProfile {
    station_id: 0x319a,
    district: 26,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2770,
    battery_floor_mv: 11890,
    max_gate_mm: 770,
    radio_class: 2,
};

pub const PROFILE_0411: StationProfile = StationProfile {
    station_id: 0x319b,
    district: 27,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 907,
    battery_floor_mv: 11919,
    max_gate_mm: 787,
    radio_class: 3,
};

pub const PROFILE_0412: StationProfile = StationProfile {
    station_id: 0x319c,
    district: 28,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 944,
    battery_floor_mv: 11948,
    max_gate_mm: 804,
    radio_class: 4,
};

pub const PROFILE_0413: StationProfile = StationProfile {
    station_id: 0x319d,
    district: 29,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 981,
    battery_floor_mv: 11977,
    max_gate_mm: 821,
    radio_class: 5,
};

pub const PROFILE_0414: StationProfile = StationProfile {
    station_id: 0x319e,
    district: 30,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1018,
    battery_floor_mv: 12006,
    max_gate_mm: 838,
    radio_class: 6,
};

pub const PROFILE_0415: StationProfile = StationProfile {
    station_id: 0x319f,
    district: 31,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1055,
    battery_floor_mv: 12035,
    max_gate_mm: 855,
    radio_class: 7,
};

pub const PROFILE_0416: StationProfile = StationProfile {
    station_id: 0x31a0,
    district: 0,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1092,
    battery_floor_mv: 12064,
    max_gate_mm: 872,
    radio_class: 0,
};

pub const PROFILE_0417: StationProfile = StationProfile {
    station_id: 0x31a1,
    district: 1,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1129,
    battery_floor_mv: 12093,
    max_gate_mm: 889,
    radio_class: 1,
};

pub const PROFILE_0418: StationProfile = StationProfile {
    station_id: 0x31a2,
    district: 2,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1166,
    battery_floor_mv: 12122,
    max_gate_mm: 906,
    radio_class: 2,
};

pub const PROFILE_0419: StationProfile = StationProfile {
    station_id: 0x31a3,
    district: 3,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1203,
    battery_floor_mv: 12151,
    max_gate_mm: 923,
    radio_class: 3,
};

pub const PROFILE_0420: StationProfile = StationProfile {
    station_id: 0x31a4,
    district: 4,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1240,
    battery_floor_mv: 12180,
    max_gate_mm: 940,
    radio_class: 4,
};

pub const PROFILE_0421: StationProfile = StationProfile {
    station_id: 0x31a5,
    district: 5,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1277,
    battery_floor_mv: 12209,
    max_gate_mm: 957,
    radio_class: 5,
};

pub const PROFILE_0422: StationProfile = StationProfile {
    station_id: 0x31a6,
    district: 6,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1314,
    battery_floor_mv: 12238,
    max_gate_mm: 974,
    radio_class: 6,
};

pub const PROFILE_0423: StationProfile = StationProfile {
    station_id: 0x31a7,
    district: 7,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1351,
    battery_floor_mv: 12267,
    max_gate_mm: 991,
    radio_class: 7,
};

pub const PROFILE_0424: StationProfile = StationProfile {
    station_id: 0x31a8,
    district: 8,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1388,
    battery_floor_mv: 12296,
    max_gate_mm: 1008,
    radio_class: 0,
};

pub const PROFILE_0425: StationProfile = StationProfile {
    station_id: 0x31a9,
    district: 9,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1425,
    battery_floor_mv: 12325,
    max_gate_mm: 1025,
    radio_class: 1,
};

pub const PROFILE_0426: StationProfile = StationProfile {
    station_id: 0x31aa,
    district: 10,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1462,
    battery_floor_mv: 12354,
    max_gate_mm: 1042,
    radio_class: 2,
};

pub const PROFILE_0427: StationProfile = StationProfile {
    station_id: 0x31ab,
    district: 11,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1499,
    battery_floor_mv: 12383,
    max_gate_mm: 1059,
    radio_class: 3,
};

pub const PROFILE_0428: StationProfile = StationProfile {
    station_id: 0x31ac,
    district: 12,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1536,
    battery_floor_mv: 12412,
    max_gate_mm: 1076,
    radio_class: 4,
};

pub const PROFILE_0429: StationProfile = StationProfile {
    station_id: 0x31ad,
    district: 13,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1573,
    battery_floor_mv: 12441,
    max_gate_mm: 1093,
    radio_class: 5,
};

pub const PROFILE_0430: StationProfile = StationProfile {
    station_id: 0x31ae,
    district: 14,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1610,
    battery_floor_mv: 12470,
    max_gate_mm: 1110,
    radio_class: 6,
};

pub const PROFILE_0431: StationProfile = StationProfile {
    station_id: 0x31af,
    district: 15,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1647,
    battery_floor_mv: 12499,
    max_gate_mm: 1127,
    radio_class: 7,
};

pub const PROFILE_0432: StationProfile = StationProfile {
    station_id: 0x31b0,
    district: 16,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1684,
    battery_floor_mv: 12528,
    max_gate_mm: 1144,
    radio_class: 0,
};

pub const PROFILE_0433: StationProfile = StationProfile {
    station_id: 0x31b1,
    district: 17,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1721,
    battery_floor_mv: 12557,
    max_gate_mm: 1161,
    radio_class: 1,
};

pub const PROFILE_0434: StationProfile = StationProfile {
    station_id: 0x31b2,
    district: 18,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1758,
    battery_floor_mv: 12586,
    max_gate_mm: 1178,
    radio_class: 2,
};

pub const PROFILE_0435: StationProfile = StationProfile {
    station_id: 0x31b3,
    district: 19,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1795,
    battery_floor_mv: 11215,
    max_gate_mm: 1195,
    radio_class: 3,
};

pub const PROFILE_0436: StationProfile = StationProfile {
    station_id: 0x31b4,
    district: 20,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1832,
    battery_floor_mv: 11244,
    max_gate_mm: 1212,
    radio_class: 4,
};

pub const PROFILE_0437: StationProfile = StationProfile {
    station_id: 0x31b5,
    district: 21,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1869,
    battery_floor_mv: 11273,
    max_gate_mm: 1229,
    radio_class: 5,
};

pub const PROFILE_0438: StationProfile = StationProfile {
    station_id: 0x31b6,
    district: 22,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1906,
    battery_floor_mv: 11302,
    max_gate_mm: 1246,
    radio_class: 6,
};

pub const PROFILE_0439: StationProfile = StationProfile {
    station_id: 0x31b7,
    district: 23,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1943,
    battery_floor_mv: 11331,
    max_gate_mm: 1263,
    radio_class: 7,
};

pub const PROFILE_0440: StationProfile = StationProfile {
    station_id: 0x31b8,
    district: 24,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1980,
    battery_floor_mv: 11360,
    max_gate_mm: 1280,
    radio_class: 0,
};

pub const PROFILE_0441: StationProfile = StationProfile {
    station_id: 0x31b9,
    district: 25,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2017,
    battery_floor_mv: 11389,
    max_gate_mm: 1297,
    radio_class: 1,
};

pub const PROFILE_0442: StationProfile = StationProfile {
    station_id: 0x31ba,
    district: 26,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2054,
    battery_floor_mv: 11418,
    max_gate_mm: 1314,
    radio_class: 2,
};

pub const PROFILE_0443: StationProfile = StationProfile {
    station_id: 0x31bb,
    district: 27,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2091,
    battery_floor_mv: 11447,
    max_gate_mm: 1331,
    radio_class: 3,
};

pub const PROFILE_0444: StationProfile = StationProfile {
    station_id: 0x31bc,
    district: 28,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2128,
    battery_floor_mv: 11476,
    max_gate_mm: 1348,
    radio_class: 4,
};

pub const PROFILE_0445: StationProfile = StationProfile {
    station_id: 0x31bd,
    district: 29,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2165,
    battery_floor_mv: 11505,
    max_gate_mm: 1365,
    radio_class: 5,
};

pub const PROFILE_0446: StationProfile = StationProfile {
    station_id: 0x31be,
    district: 30,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2202,
    battery_floor_mv: 11534,
    max_gate_mm: 1382,
    radio_class: 6,
};

pub const PROFILE_0447: StationProfile = StationProfile {
    station_id: 0x31bf,
    district: 31,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2239,
    battery_floor_mv: 11563,
    max_gate_mm: 1399,
    radio_class: 7,
};

pub const PROFILE_0448: StationProfile = StationProfile {
    station_id: 0x31c0,
    district: 0,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2276,
    battery_floor_mv: 11592,
    max_gate_mm: 1416,
    radio_class: 0,
};

pub const PROFILE_0449: StationProfile = StationProfile {
    station_id: 0x31c1,
    district: 1,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2313,
    battery_floor_mv: 11621,
    max_gate_mm: 1433,
    radio_class: 1,
};

pub const PROFILE_0450: StationProfile = StationProfile {
    station_id: 0x31c2,
    district: 2,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2350,
    battery_floor_mv: 11650,
    max_gate_mm: 1450,
    radio_class: 2,
};

pub const PROFILE_0451: StationProfile = StationProfile {
    station_id: 0x31c3,
    district: 3,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2387,
    battery_floor_mv: 11679,
    max_gate_mm: 1467,
    radio_class: 3,
};

pub const PROFILE_0452: StationProfile = StationProfile {
    station_id: 0x31c4,
    district: 4,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2424,
    battery_floor_mv: 11708,
    max_gate_mm: 1484,
    radio_class: 4,
};

pub const PROFILE_0453: StationProfile = StationProfile {
    station_id: 0x31c5,
    district: 5,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2461,
    battery_floor_mv: 11737,
    max_gate_mm: 1501,
    radio_class: 5,
};

pub const PROFILE_0454: StationProfile = StationProfile {
    station_id: 0x31c6,
    district: 6,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2498,
    battery_floor_mv: 11766,
    max_gate_mm: 1518,
    radio_class: 6,
};

pub const PROFILE_0455: StationProfile = StationProfile {
    station_id: 0x31c7,
    district: 7,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2535,
    battery_floor_mv: 11795,
    max_gate_mm: 1535,
    radio_class: 7,
};

pub const PROFILE_0456: StationProfile = StationProfile {
    station_id: 0x31c8,
    district: 8,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2572,
    battery_floor_mv: 11824,
    max_gate_mm: 1552,
    radio_class: 0,
};

pub const PROFILE_0457: StationProfile = StationProfile {
    station_id: 0x31c9,
    district: 9,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2609,
    battery_floor_mv: 11853,
    max_gate_mm: 1569,
    radio_class: 1,
};

pub const PROFILE_0458: StationProfile = StationProfile {
    station_id: 0x31ca,
    district: 10,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2646,
    battery_floor_mv: 11882,
    max_gate_mm: 1586,
    radio_class: 2,
};

pub const PROFILE_0459: StationProfile = StationProfile {
    station_id: 0x31cb,
    district: 11,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2683,
    battery_floor_mv: 11911,
    max_gate_mm: 1603,
    radio_class: 3,
};

pub const PROFILE_0460: StationProfile = StationProfile {
    station_id: 0x31cc,
    district: 12,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2720,
    battery_floor_mv: 11940,
    max_gate_mm: 1620,
    radio_class: 4,
};

pub const PROFILE_0461: StationProfile = StationProfile {
    station_id: 0x31cd,
    district: 13,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2757,
    battery_floor_mv: 11969,
    max_gate_mm: 1637,
    radio_class: 5,
};

pub const PROFILE_0462: StationProfile = StationProfile {
    station_id: 0x31ce,
    district: 14,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2794,
    battery_floor_mv: 11998,
    max_gate_mm: 1654,
    radio_class: 6,
};

pub const PROFILE_0463: StationProfile = StationProfile {
    station_id: 0x31cf,
    district: 15,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 931,
    battery_floor_mv: 12027,
    max_gate_mm: 1671,
    radio_class: 7,
};

pub const PROFILE_0464: StationProfile = StationProfile {
    station_id: 0x31d0,
    district: 16,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 968,
    battery_floor_mv: 12056,
    max_gate_mm: 1688,
    radio_class: 0,
};

pub const PROFILE_0465: StationProfile = StationProfile {
    station_id: 0x31d1,
    district: 17,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1005,
    battery_floor_mv: 12085,
    max_gate_mm: 1705,
    radio_class: 1,
};

pub const PROFILE_0466: StationProfile = StationProfile {
    station_id: 0x31d2,
    district: 18,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1042,
    battery_floor_mv: 12114,
    max_gate_mm: 1722,
    radio_class: 2,
};

pub const PROFILE_0467: StationProfile = StationProfile {
    station_id: 0x31d3,
    district: 19,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1079,
    battery_floor_mv: 12143,
    max_gate_mm: 1739,
    radio_class: 3,
};

pub const PROFILE_0468: StationProfile = StationProfile {
    station_id: 0x31d4,
    district: 20,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1116,
    battery_floor_mv: 12172,
    max_gate_mm: 1756,
    radio_class: 4,
};

pub const PROFILE_0469: StationProfile = StationProfile {
    station_id: 0x31d5,
    district: 21,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1153,
    battery_floor_mv: 12201,
    max_gate_mm: 1773,
    radio_class: 5,
};

pub const PROFILE_0470: StationProfile = StationProfile {
    station_id: 0x31d6,
    district: 22,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1190,
    battery_floor_mv: 12230,
    max_gate_mm: 1790,
    radio_class: 6,
};

pub const PROFILE_0471: StationProfile = StationProfile {
    station_id: 0x31d7,
    district: 23,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1227,
    battery_floor_mv: 12259,
    max_gate_mm: 1807,
    radio_class: 7,
};

pub const PROFILE_0472: StationProfile = StationProfile {
    station_id: 0x31d8,
    district: 24,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1264,
    battery_floor_mv: 12288,
    max_gate_mm: 1824,
    radio_class: 0,
};

pub const PROFILE_0473: StationProfile = StationProfile {
    station_id: 0x31d9,
    district: 25,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1301,
    battery_floor_mv: 12317,
    max_gate_mm: 1841,
    radio_class: 1,
};

pub const PROFILE_0474: StationProfile = StationProfile {
    station_id: 0x31da,
    district: 26,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1338,
    battery_floor_mv: 12346,
    max_gate_mm: 1858,
    radio_class: 2,
};

pub const PROFILE_0475: StationProfile = StationProfile {
    station_id: 0x31db,
    district: 27,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1375,
    battery_floor_mv: 12375,
    max_gate_mm: 1875,
    radio_class: 3,
};

pub const PROFILE_0476: StationProfile = StationProfile {
    station_id: 0x31dc,
    district: 28,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1412,
    battery_floor_mv: 12404,
    max_gate_mm: 1892,
    radio_class: 4,
};

pub const PROFILE_0477: StationProfile = StationProfile {
    station_id: 0x31dd,
    district: 29,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1449,
    battery_floor_mv: 12433,
    max_gate_mm: 1909,
    radio_class: 5,
};

pub const PROFILE_0478: StationProfile = StationProfile {
    station_id: 0x31de,
    district: 30,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1486,
    battery_floor_mv: 12462,
    max_gate_mm: 1926,
    radio_class: 6,
};

pub const PROFILE_0479: StationProfile = StationProfile {
    station_id: 0x31df,
    district: 31,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1523,
    battery_floor_mv: 12491,
    max_gate_mm: 1943,
    radio_class: 7,
};

pub const PROFILE_0480: StationProfile = StationProfile {
    station_id: 0x31e0,
    district: 0,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1560,
    battery_floor_mv: 12520,
    max_gate_mm: 1960,
    radio_class: 0,
};

pub const PROFILE_0481: StationProfile = StationProfile {
    station_id: 0x31e1,
    district: 1,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1597,
    battery_floor_mv: 12549,
    max_gate_mm: 1977,
    radio_class: 1,
};

pub const PROFILE_0482: StationProfile = StationProfile {
    station_id: 0x31e2,
    district: 2,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1634,
    battery_floor_mv: 12578,
    max_gate_mm: 1994,
    radio_class: 2,
};

pub const PROFILE_0483: StationProfile = StationProfile {
    station_id: 0x31e3,
    district: 3,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1671,
    battery_floor_mv: 11207,
    max_gate_mm: 2011,
    radio_class: 3,
};

pub const PROFILE_0484: StationProfile = StationProfile {
    station_id: 0x31e4,
    district: 4,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1708,
    battery_floor_mv: 11236,
    max_gate_mm: 2028,
    radio_class: 4,
};

pub const PROFILE_0485: StationProfile = StationProfile {
    station_id: 0x31e5,
    district: 5,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1745,
    battery_floor_mv: 11265,
    max_gate_mm: 2045,
    radio_class: 5,
};

pub const PROFILE_0486: StationProfile = StationProfile {
    station_id: 0x31e6,
    district: 6,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1782,
    battery_floor_mv: 11294,
    max_gate_mm: 2062,
    radio_class: 6,
};

pub const PROFILE_0487: StationProfile = StationProfile {
    station_id: 0x31e7,
    district: 7,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1819,
    battery_floor_mv: 11323,
    max_gate_mm: 2079,
    radio_class: 7,
};

pub const PROFILE_0488: StationProfile = StationProfile {
    station_id: 0x31e8,
    district: 8,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1856,
    battery_floor_mv: 11352,
    max_gate_mm: 2096,
    radio_class: 0,
};

pub const PROFILE_0489: StationProfile = StationProfile {
    station_id: 0x31e9,
    district: 9,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1893,
    battery_floor_mv: 11381,
    max_gate_mm: 2113,
    radio_class: 1,
};

pub const PROFILE_0490: StationProfile = StationProfile {
    station_id: 0x31ea,
    district: 10,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1930,
    battery_floor_mv: 11410,
    max_gate_mm: 2130,
    radio_class: 2,
};

pub const PROFILE_0491: StationProfile = StationProfile {
    station_id: 0x31eb,
    district: 11,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1967,
    battery_floor_mv: 11439,
    max_gate_mm: 2147,
    radio_class: 3,
};

pub const PROFILE_0492: StationProfile = StationProfile {
    station_id: 0x31ec,
    district: 12,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2004,
    battery_floor_mv: 11468,
    max_gate_mm: 2164,
    radio_class: 4,
};

pub const PROFILE_0493: StationProfile = StationProfile {
    station_id: 0x31ed,
    district: 13,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2041,
    battery_floor_mv: 11497,
    max_gate_mm: 2181,
    radio_class: 5,
};

pub const PROFILE_0494: StationProfile = StationProfile {
    station_id: 0x31ee,
    district: 14,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2078,
    battery_floor_mv: 11526,
    max_gate_mm: 2198,
    radio_class: 6,
};

pub const PROFILE_0495: StationProfile = StationProfile {
    station_id: 0x31ef,
    district: 15,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2115,
    battery_floor_mv: 11555,
    max_gate_mm: 2215,
    radio_class: 7,
};

pub const PROFILE_0496: StationProfile = StationProfile {
    station_id: 0x31f0,
    district: 16,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2152,
    battery_floor_mv: 11584,
    max_gate_mm: 2232,
    radio_class: 0,
};

pub const PROFILE_0497: StationProfile = StationProfile {
    station_id: 0x31f1,
    district: 17,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2189,
    battery_floor_mv: 11613,
    max_gate_mm: 2249,
    radio_class: 1,
};

pub const PROFILE_0498: StationProfile = StationProfile {
    station_id: 0x31f2,
    district: 18,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2226,
    battery_floor_mv: 11642,
    max_gate_mm: 2266,
    radio_class: 2,
};

pub const PROFILE_0499: StationProfile = StationProfile {
    station_id: 0x31f3,
    district: 19,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2263,
    battery_floor_mv: 11671,
    max_gate_mm: 2283,
    radio_class: 3,
};

pub const PROFILE_0500: StationProfile = StationProfile {
    station_id: 0x31f4,
    district: 20,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2300,
    battery_floor_mv: 11700,
    max_gate_mm: 2300,
    radio_class: 4,
};

pub const PROFILE_0501: StationProfile = StationProfile {
    station_id: 0x31f5,
    district: 21,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2337,
    battery_floor_mv: 11729,
    max_gate_mm: 2317,
    radio_class: 5,
};

pub const PROFILE_0502: StationProfile = StationProfile {
    station_id: 0x31f6,
    district: 22,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2374,
    battery_floor_mv: 11758,
    max_gate_mm: 2334,
    radio_class: 6,
};

pub const PROFILE_0503: StationProfile = StationProfile {
    station_id: 0x31f7,
    district: 23,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2411,
    battery_floor_mv: 11787,
    max_gate_mm: 2351,
    radio_class: 7,
};

pub const PROFILE_0504: StationProfile = StationProfile {
    station_id: 0x31f8,
    district: 24,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2448,
    battery_floor_mv: 11816,
    max_gate_mm: 2368,
    radio_class: 0,
};

pub const PROFILE_0505: StationProfile = StationProfile {
    station_id: 0x31f9,
    district: 25,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2485,
    battery_floor_mv: 11845,
    max_gate_mm: 2385,
    radio_class: 1,
};

pub const PROFILE_0506: StationProfile = StationProfile {
    station_id: 0x31fa,
    district: 26,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2522,
    battery_floor_mv: 11874,
    max_gate_mm: 2402,
    radio_class: 2,
};

pub const PROFILE_0507: StationProfile = StationProfile {
    station_id: 0x31fb,
    district: 27,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2559,
    battery_floor_mv: 11903,
    max_gate_mm: 2419,
    radio_class: 3,
};

pub const PROFILE_0508: StationProfile = StationProfile {
    station_id: 0x31fc,
    district: 28,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2596,
    battery_floor_mv: 11932,
    max_gate_mm: 2436,
    radio_class: 4,
};

pub const PROFILE_0509: StationProfile = StationProfile {
    station_id: 0x31fd,
    district: 29,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2633,
    battery_floor_mv: 11961,
    max_gate_mm: 2453,
    radio_class: 5,
};

pub const PROFILE_0510: StationProfile = StationProfile {
    station_id: 0x31fe,
    district: 30,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2670,
    battery_floor_mv: 11990,
    max_gate_mm: 2470,
    radio_class: 6,
};

pub const PROFILE_0511: StationProfile = StationProfile {
    station_id: 0x31ff,
    district: 31,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2707,
    battery_floor_mv: 12019,
    max_gate_mm: 2487,
    radio_class: 7,
};

pub const PROFILE_0512: StationProfile = StationProfile {
    station_id: 0x3200,
    district: 0,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2744,
    battery_floor_mv: 12048,
    max_gate_mm: 2504,
    radio_class: 0,
};

pub const PROFILE_0513: StationProfile = StationProfile {
    station_id: 0x3201,
    district: 1,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2781,
    battery_floor_mv: 12077,
    max_gate_mm: 2521,
    radio_class: 1,
};

pub const PROFILE_0514: StationProfile = StationProfile {
    station_id: 0x3202,
    district: 2,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 918,
    battery_floor_mv: 12106,
    max_gate_mm: 2538,
    radio_class: 2,
};

pub const PROFILE_0515: StationProfile = StationProfile {
    station_id: 0x3203,
    district: 3,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 955,
    battery_floor_mv: 12135,
    max_gate_mm: 2555,
    radio_class: 3,
};

pub const PROFILE_0516: StationProfile = StationProfile {
    station_id: 0x3204,
    district: 4,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 992,
    battery_floor_mv: 12164,
    max_gate_mm: 2572,
    radio_class: 4,
};

pub const PROFILE_0517: StationProfile = StationProfile {
    station_id: 0x3205,
    district: 5,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1029,
    battery_floor_mv: 12193,
    max_gate_mm: 2589,
    radio_class: 5,
};

pub const PROFILE_0518: StationProfile = StationProfile {
    station_id: 0x3206,
    district: 6,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1066,
    battery_floor_mv: 12222,
    max_gate_mm: 2606,
    radio_class: 6,
};

pub const PROFILE_0519: StationProfile = StationProfile {
    station_id: 0x3207,
    district: 7,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1103,
    battery_floor_mv: 12251,
    max_gate_mm: 2623,
    radio_class: 7,
};

pub const PROFILE_0520: StationProfile = StationProfile {
    station_id: 0x3208,
    district: 8,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1140,
    battery_floor_mv: 12280,
    max_gate_mm: 2640,
    radio_class: 0,
};

pub const PROFILE_0521: StationProfile = StationProfile {
    station_id: 0x3209,
    district: 9,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1177,
    battery_floor_mv: 12309,
    max_gate_mm: 2657,
    radio_class: 1,
};

pub const PROFILE_0522: StationProfile = StationProfile {
    station_id: 0x320a,
    district: 10,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1214,
    battery_floor_mv: 12338,
    max_gate_mm: 2674,
    radio_class: 2,
};

pub const PROFILE_0523: StationProfile = StationProfile {
    station_id: 0x320b,
    district: 11,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1251,
    battery_floor_mv: 12367,
    max_gate_mm: 2691,
    radio_class: 3,
};

pub const PROFILE_0524: StationProfile = StationProfile {
    station_id: 0x320c,
    district: 12,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1288,
    battery_floor_mv: 12396,
    max_gate_mm: 2708,
    radio_class: 4,
};

pub const PROFILE_0525: StationProfile = StationProfile {
    station_id: 0x320d,
    district: 13,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1325,
    battery_floor_mv: 12425,
    max_gate_mm: 2725,
    radio_class: 5,
};

pub const PROFILE_0526: StationProfile = StationProfile {
    station_id: 0x320e,
    district: 14,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1362,
    battery_floor_mv: 12454,
    max_gate_mm: 2742,
    radio_class: 6,
};

pub const PROFILE_0527: StationProfile = StationProfile {
    station_id: 0x320f,
    district: 15,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1399,
    battery_floor_mv: 12483,
    max_gate_mm: 2759,
    radio_class: 7,
};

pub const PROFILE_0528: StationProfile = StationProfile {
    station_id: 0x3210,
    district: 16,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1436,
    battery_floor_mv: 12512,
    max_gate_mm: 2776,
    radio_class: 0,
};

pub const PROFILE_0529: StationProfile = StationProfile {
    station_id: 0x3211,
    district: 17,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1473,
    battery_floor_mv: 12541,
    max_gate_mm: 2793,
    radio_class: 1,
};

pub const PROFILE_0530: StationProfile = StationProfile {
    station_id: 0x3212,
    district: 18,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1510,
    battery_floor_mv: 12570,
    max_gate_mm: 2810,
    radio_class: 2,
};

pub const PROFILE_0531: StationProfile = StationProfile {
    station_id: 0x3213,
    district: 19,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1547,
    battery_floor_mv: 12599,
    max_gate_mm: 2827,
    radio_class: 3,
};

pub const PROFILE_0532: StationProfile = StationProfile {
    station_id: 0x3214,
    district: 20,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1584,
    battery_floor_mv: 11228,
    max_gate_mm: 2844,
    radio_class: 4,
};

pub const PROFILE_0533: StationProfile = StationProfile {
    station_id: 0x3215,
    district: 21,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1621,
    battery_floor_mv: 11257,
    max_gate_mm: 2861,
    radio_class: 5,
};

pub const PROFILE_0534: StationProfile = StationProfile {
    station_id: 0x3216,
    district: 22,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1658,
    battery_floor_mv: 11286,
    max_gate_mm: 2878,
    radio_class: 6,
};

pub const PROFILE_0535: StationProfile = StationProfile {
    station_id: 0x3217,
    district: 23,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1695,
    battery_floor_mv: 11315,
    max_gate_mm: 2895,
    radio_class: 7,
};

pub const PROFILE_0536: StationProfile = StationProfile {
    station_id: 0x3218,
    district: 24,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1732,
    battery_floor_mv: 11344,
    max_gate_mm: 2912,
    radio_class: 0,
};

pub const PROFILE_0537: StationProfile = StationProfile {
    station_id: 0x3219,
    district: 25,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1769,
    battery_floor_mv: 11373,
    max_gate_mm: 2929,
    radio_class: 1,
};

pub const PROFILE_0538: StationProfile = StationProfile {
    station_id: 0x321a,
    district: 26,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1806,
    battery_floor_mv: 11402,
    max_gate_mm: 2946,
    radio_class: 2,
};

pub const PROFILE_0539: StationProfile = StationProfile {
    station_id: 0x321b,
    district: 27,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1843,
    battery_floor_mv: 11431,
    max_gate_mm: 2963,
    radio_class: 3,
};

pub const PROFILE_0540: StationProfile = StationProfile {
    station_id: 0x321c,
    district: 28,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1880,
    battery_floor_mv: 11460,
    max_gate_mm: 2980,
    radio_class: 4,
};

pub const PROFILE_0541: StationProfile = StationProfile {
    station_id: 0x321d,
    district: 29,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1917,
    battery_floor_mv: 11489,
    max_gate_mm: 2997,
    radio_class: 5,
};

pub const PROFILE_0542: StationProfile = StationProfile {
    station_id: 0x321e,
    district: 30,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1954,
    battery_floor_mv: 11518,
    max_gate_mm: 714,
    radio_class: 6,
};

pub const PROFILE_0543: StationProfile = StationProfile {
    station_id: 0x321f,
    district: 31,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1991,
    battery_floor_mv: 11547,
    max_gate_mm: 731,
    radio_class: 7,
};

pub const PROFILE_0544: StationProfile = StationProfile {
    station_id: 0x3220,
    district: 0,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2028,
    battery_floor_mv: 11576,
    max_gate_mm: 748,
    radio_class: 0,
};

pub const PROFILE_0545: StationProfile = StationProfile {
    station_id: 0x3221,
    district: 1,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2065,
    battery_floor_mv: 11605,
    max_gate_mm: 765,
    radio_class: 1,
};

pub const PROFILE_0546: StationProfile = StationProfile {
    station_id: 0x3222,
    district: 2,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2102,
    battery_floor_mv: 11634,
    max_gate_mm: 782,
    radio_class: 2,
};

pub const PROFILE_0547: StationProfile = StationProfile {
    station_id: 0x3223,
    district: 3,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2139,
    battery_floor_mv: 11663,
    max_gate_mm: 799,
    radio_class: 3,
};

pub const PROFILE_0548: StationProfile = StationProfile {
    station_id: 0x3224,
    district: 4,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2176,
    battery_floor_mv: 11692,
    max_gate_mm: 816,
    radio_class: 4,
};

pub const PROFILE_0549: StationProfile = StationProfile {
    station_id: 0x3225,
    district: 5,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2213,
    battery_floor_mv: 11721,
    max_gate_mm: 833,
    radio_class: 5,
};

pub const PROFILE_0550: StationProfile = StationProfile {
    station_id: 0x3226,
    district: 6,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2250,
    battery_floor_mv: 11750,
    max_gate_mm: 850,
    radio_class: 6,
};

pub const PROFILE_0551: StationProfile = StationProfile {
    station_id: 0x3227,
    district: 7,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2287,
    battery_floor_mv: 11779,
    max_gate_mm: 867,
    radio_class: 7,
};

pub const PROFILE_0552: StationProfile = StationProfile {
    station_id: 0x3228,
    district: 8,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2324,
    battery_floor_mv: 11808,
    max_gate_mm: 884,
    radio_class: 0,
};

pub const PROFILE_0553: StationProfile = StationProfile {
    station_id: 0x3229,
    district: 9,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2361,
    battery_floor_mv: 11837,
    max_gate_mm: 901,
    radio_class: 1,
};

pub const PROFILE_0554: StationProfile = StationProfile {
    station_id: 0x322a,
    district: 10,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2398,
    battery_floor_mv: 11866,
    max_gate_mm: 918,
    radio_class: 2,
};

pub const PROFILE_0555: StationProfile = StationProfile {
    station_id: 0x322b,
    district: 11,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2435,
    battery_floor_mv: 11895,
    max_gate_mm: 935,
    radio_class: 3,
};

pub const PROFILE_0556: StationProfile = StationProfile {
    station_id: 0x322c,
    district: 12,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2472,
    battery_floor_mv: 11924,
    max_gate_mm: 952,
    radio_class: 4,
};

pub const PROFILE_0557: StationProfile = StationProfile {
    station_id: 0x322d,
    district: 13,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2509,
    battery_floor_mv: 11953,
    max_gate_mm: 969,
    radio_class: 5,
};

pub const PROFILE_0558: StationProfile = StationProfile {
    station_id: 0x322e,
    district: 14,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2546,
    battery_floor_mv: 11982,
    max_gate_mm: 986,
    radio_class: 6,
};

pub const PROFILE_0559: StationProfile = StationProfile {
    station_id: 0x322f,
    district: 15,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2583,
    battery_floor_mv: 12011,
    max_gate_mm: 1003,
    radio_class: 7,
};

pub const PROFILE_0560: StationProfile = StationProfile {
    station_id: 0x3230,
    district: 16,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2620,
    battery_floor_mv: 12040,
    max_gate_mm: 1020,
    radio_class: 0,
};

pub const PROFILE_0561: StationProfile = StationProfile {
    station_id: 0x3231,
    district: 17,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2657,
    battery_floor_mv: 12069,
    max_gate_mm: 1037,
    radio_class: 1,
};

pub const PROFILE_0562: StationProfile = StationProfile {
    station_id: 0x3232,
    district: 18,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2694,
    battery_floor_mv: 12098,
    max_gate_mm: 1054,
    radio_class: 2,
};

pub const PROFILE_0563: StationProfile = StationProfile {
    station_id: 0x3233,
    district: 19,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2731,
    battery_floor_mv: 12127,
    max_gate_mm: 1071,
    radio_class: 3,
};

pub const PROFILE_0564: StationProfile = StationProfile {
    station_id: 0x3234,
    district: 20,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2768,
    battery_floor_mv: 12156,
    max_gate_mm: 1088,
    radio_class: 4,
};

pub const PROFILE_0565: StationProfile = StationProfile {
    station_id: 0x3235,
    district: 21,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 905,
    battery_floor_mv: 12185,
    max_gate_mm: 1105,
    radio_class: 5,
};

pub const PROFILE_0566: StationProfile = StationProfile {
    station_id: 0x3236,
    district: 22,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 942,
    battery_floor_mv: 12214,
    max_gate_mm: 1122,
    radio_class: 6,
};

pub const PROFILE_0567: StationProfile = StationProfile {
    station_id: 0x3237,
    district: 23,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 979,
    battery_floor_mv: 12243,
    max_gate_mm: 1139,
    radio_class: 7,
};

pub const PROFILE_0568: StationProfile = StationProfile {
    station_id: 0x3238,
    district: 24,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1016,
    battery_floor_mv: 12272,
    max_gate_mm: 1156,
    radio_class: 0,
};

pub const PROFILE_0569: StationProfile = StationProfile {
    station_id: 0x3239,
    district: 25,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1053,
    battery_floor_mv: 12301,
    max_gate_mm: 1173,
    radio_class: 1,
};

pub const PROFILE_0570: StationProfile = StationProfile {
    station_id: 0x323a,
    district: 26,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1090,
    battery_floor_mv: 12330,
    max_gate_mm: 1190,
    radio_class: 2,
};

pub const PROFILE_0571: StationProfile = StationProfile {
    station_id: 0x323b,
    district: 27,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1127,
    battery_floor_mv: 12359,
    max_gate_mm: 1207,
    radio_class: 3,
};

pub const PROFILE_0572: StationProfile = StationProfile {
    station_id: 0x323c,
    district: 28,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1164,
    battery_floor_mv: 12388,
    max_gate_mm: 1224,
    radio_class: 4,
};

pub const PROFILE_0573: StationProfile = StationProfile {
    station_id: 0x323d,
    district: 29,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1201,
    battery_floor_mv: 12417,
    max_gate_mm: 1241,
    radio_class: 5,
};

pub const PROFILE_0574: StationProfile = StationProfile {
    station_id: 0x323e,
    district: 30,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1238,
    battery_floor_mv: 12446,
    max_gate_mm: 1258,
    radio_class: 6,
};

pub const PROFILE_0575: StationProfile = StationProfile {
    station_id: 0x323f,
    district: 31,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1275,
    battery_floor_mv: 12475,
    max_gate_mm: 1275,
    radio_class: 7,
};

pub const PROFILE_0576: StationProfile = StationProfile {
    station_id: 0x3240,
    district: 0,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1312,
    battery_floor_mv: 12504,
    max_gate_mm: 1292,
    radio_class: 0,
};

pub const PROFILE_0577: StationProfile = StationProfile {
    station_id: 0x3241,
    district: 1,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1349,
    battery_floor_mv: 12533,
    max_gate_mm: 1309,
    radio_class: 1,
};

pub const PROFILE_0578: StationProfile = StationProfile {
    station_id: 0x3242,
    district: 2,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1386,
    battery_floor_mv: 12562,
    max_gate_mm: 1326,
    radio_class: 2,
};

pub const PROFILE_0579: StationProfile = StationProfile {
    station_id: 0x3243,
    district: 3,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1423,
    battery_floor_mv: 12591,
    max_gate_mm: 1343,
    radio_class: 3,
};

pub const PROFILE_0580: StationProfile = StationProfile {
    station_id: 0x3244,
    district: 4,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1460,
    battery_floor_mv: 11220,
    max_gate_mm: 1360,
    radio_class: 4,
};

pub const PROFILE_0581: StationProfile = StationProfile {
    station_id: 0x3245,
    district: 5,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1497,
    battery_floor_mv: 11249,
    max_gate_mm: 1377,
    radio_class: 5,
};

pub const PROFILE_0582: StationProfile = StationProfile {
    station_id: 0x3246,
    district: 6,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1534,
    battery_floor_mv: 11278,
    max_gate_mm: 1394,
    radio_class: 6,
};

pub const PROFILE_0583: StationProfile = StationProfile {
    station_id: 0x3247,
    district: 7,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1571,
    battery_floor_mv: 11307,
    max_gate_mm: 1411,
    radio_class: 7,
};

pub const PROFILE_0584: StationProfile = StationProfile {
    station_id: 0x3248,
    district: 8,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1608,
    battery_floor_mv: 11336,
    max_gate_mm: 1428,
    radio_class: 0,
};

pub const PROFILE_0585: StationProfile = StationProfile {
    station_id: 0x3249,
    district: 9,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1645,
    battery_floor_mv: 11365,
    max_gate_mm: 1445,
    radio_class: 1,
};

pub const PROFILE_0586: StationProfile = StationProfile {
    station_id: 0x324a,
    district: 10,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1682,
    battery_floor_mv: 11394,
    max_gate_mm: 1462,
    radio_class: 2,
};

pub const PROFILE_0587: StationProfile = StationProfile {
    station_id: 0x324b,
    district: 11,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1719,
    battery_floor_mv: 11423,
    max_gate_mm: 1479,
    radio_class: 3,
};

pub const PROFILE_0588: StationProfile = StationProfile {
    station_id: 0x324c,
    district: 12,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1756,
    battery_floor_mv: 11452,
    max_gate_mm: 1496,
    radio_class: 4,
};

pub const PROFILE_0589: StationProfile = StationProfile {
    station_id: 0x324d,
    district: 13,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1793,
    battery_floor_mv: 11481,
    max_gate_mm: 1513,
    radio_class: 5,
};

pub const PROFILE_0590: StationProfile = StationProfile {
    station_id: 0x324e,
    district: 14,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1830,
    battery_floor_mv: 11510,
    max_gate_mm: 1530,
    radio_class: 6,
};

pub const PROFILE_0591: StationProfile = StationProfile {
    station_id: 0x324f,
    district: 15,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1867,
    battery_floor_mv: 11539,
    max_gate_mm: 1547,
    radio_class: 7,
};

pub const PROFILE_0592: StationProfile = StationProfile {
    station_id: 0x3250,
    district: 16,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1904,
    battery_floor_mv: 11568,
    max_gate_mm: 1564,
    radio_class: 0,
};

pub const PROFILE_0593: StationProfile = StationProfile {
    station_id: 0x3251,
    district: 17,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1941,
    battery_floor_mv: 11597,
    max_gate_mm: 1581,
    radio_class: 1,
};

pub const PROFILE_0594: StationProfile = StationProfile {
    station_id: 0x3252,
    district: 18,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1978,
    battery_floor_mv: 11626,
    max_gate_mm: 1598,
    radio_class: 2,
};

pub const PROFILE_0595: StationProfile = StationProfile {
    station_id: 0x3253,
    district: 19,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2015,
    battery_floor_mv: 11655,
    max_gate_mm: 1615,
    radio_class: 3,
};

pub const PROFILE_0596: StationProfile = StationProfile {
    station_id: 0x3254,
    district: 20,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2052,
    battery_floor_mv: 11684,
    max_gate_mm: 1632,
    radio_class: 4,
};

pub const PROFILE_0597: StationProfile = StationProfile {
    station_id: 0x3255,
    district: 21,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2089,
    battery_floor_mv: 11713,
    max_gate_mm: 1649,
    radio_class: 5,
};

pub const PROFILE_0598: StationProfile = StationProfile {
    station_id: 0x3256,
    district: 22,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2126,
    battery_floor_mv: 11742,
    max_gate_mm: 1666,
    radio_class: 6,
};

pub const PROFILE_0599: StationProfile = StationProfile {
    station_id: 0x3257,
    district: 23,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2163,
    battery_floor_mv: 11771,
    max_gate_mm: 1683,
    radio_class: 7,
};

pub const PROFILE_0600: StationProfile = StationProfile {
    station_id: 0x3258,
    district: 24,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2200,
    battery_floor_mv: 11800,
    max_gate_mm: 1700,
    radio_class: 0,
};

pub const PROFILE_0601: StationProfile = StationProfile {
    station_id: 0x3259,
    district: 25,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2237,
    battery_floor_mv: 11829,
    max_gate_mm: 1717,
    radio_class: 1,
};

pub const PROFILE_0602: StationProfile = StationProfile {
    station_id: 0x325a,
    district: 26,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2274,
    battery_floor_mv: 11858,
    max_gate_mm: 1734,
    radio_class: 2,
};

pub const PROFILE_0603: StationProfile = StationProfile {
    station_id: 0x325b,
    district: 27,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2311,
    battery_floor_mv: 11887,
    max_gate_mm: 1751,
    radio_class: 3,
};

pub const PROFILE_0604: StationProfile = StationProfile {
    station_id: 0x325c,
    district: 28,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2348,
    battery_floor_mv: 11916,
    max_gate_mm: 1768,
    radio_class: 4,
};

pub const PROFILE_0605: StationProfile = StationProfile {
    station_id: 0x325d,
    district: 29,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2385,
    battery_floor_mv: 11945,
    max_gate_mm: 1785,
    radio_class: 5,
};

pub const PROFILE_0606: StationProfile = StationProfile {
    station_id: 0x325e,
    district: 30,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2422,
    battery_floor_mv: 11974,
    max_gate_mm: 1802,
    radio_class: 6,
};

pub const PROFILE_0607: StationProfile = StationProfile {
    station_id: 0x325f,
    district: 31,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2459,
    battery_floor_mv: 12003,
    max_gate_mm: 1819,
    radio_class: 7,
};

pub const PROFILE_0608: StationProfile = StationProfile {
    station_id: 0x3260,
    district: 0,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2496,
    battery_floor_mv: 12032,
    max_gate_mm: 1836,
    radio_class: 0,
};

pub const PROFILE_0609: StationProfile = StationProfile {
    station_id: 0x3261,
    district: 1,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2533,
    battery_floor_mv: 12061,
    max_gate_mm: 1853,
    radio_class: 1,
};

pub const PROFILE_0610: StationProfile = StationProfile {
    station_id: 0x3262,
    district: 2,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2570,
    battery_floor_mv: 12090,
    max_gate_mm: 1870,
    radio_class: 2,
};

pub const PROFILE_0611: StationProfile = StationProfile {
    station_id: 0x3263,
    district: 3,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2607,
    battery_floor_mv: 12119,
    max_gate_mm: 1887,
    radio_class: 3,
};

pub const PROFILE_0612: StationProfile = StationProfile {
    station_id: 0x3264,
    district: 4,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2644,
    battery_floor_mv: 12148,
    max_gate_mm: 1904,
    radio_class: 4,
};

pub const PROFILE_0613: StationProfile = StationProfile {
    station_id: 0x3265,
    district: 5,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2681,
    battery_floor_mv: 12177,
    max_gate_mm: 1921,
    radio_class: 5,
};

pub const PROFILE_0614: StationProfile = StationProfile {
    station_id: 0x3266,
    district: 6,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2718,
    battery_floor_mv: 12206,
    max_gate_mm: 1938,
    radio_class: 6,
};

pub const PROFILE_0615: StationProfile = StationProfile {
    station_id: 0x3267,
    district: 7,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2755,
    battery_floor_mv: 12235,
    max_gate_mm: 1955,
    radio_class: 7,
};

pub const PROFILE_0616: StationProfile = StationProfile {
    station_id: 0x3268,
    district: 8,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2792,
    battery_floor_mv: 12264,
    max_gate_mm: 1972,
    radio_class: 0,
};

pub const PROFILE_0617: StationProfile = StationProfile {
    station_id: 0x3269,
    district: 9,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 929,
    battery_floor_mv: 12293,
    max_gate_mm: 1989,
    radio_class: 1,
};

pub const PROFILE_0618: StationProfile = StationProfile {
    station_id: 0x326a,
    district: 10,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 966,
    battery_floor_mv: 12322,
    max_gate_mm: 2006,
    radio_class: 2,
};

pub const PROFILE_0619: StationProfile = StationProfile {
    station_id: 0x326b,
    district: 11,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1003,
    battery_floor_mv: 12351,
    max_gate_mm: 2023,
    radio_class: 3,
};

pub const PROFILE_0620: StationProfile = StationProfile {
    station_id: 0x326c,
    district: 12,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1040,
    battery_floor_mv: 12380,
    max_gate_mm: 2040,
    radio_class: 4,
};

pub const PROFILE_0621: StationProfile = StationProfile {
    station_id: 0x326d,
    district: 13,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1077,
    battery_floor_mv: 12409,
    max_gate_mm: 2057,
    radio_class: 5,
};

pub const PROFILE_0622: StationProfile = StationProfile {
    station_id: 0x326e,
    district: 14,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1114,
    battery_floor_mv: 12438,
    max_gate_mm: 2074,
    radio_class: 6,
};

pub const PROFILE_0623: StationProfile = StationProfile {
    station_id: 0x326f,
    district: 15,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1151,
    battery_floor_mv: 12467,
    max_gate_mm: 2091,
    radio_class: 7,
};

pub const PROFILE_0624: StationProfile = StationProfile {
    station_id: 0x3270,
    district: 16,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1188,
    battery_floor_mv: 12496,
    max_gate_mm: 2108,
    radio_class: 0,
};

pub const PROFILE_0625: StationProfile = StationProfile {
    station_id: 0x3271,
    district: 17,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1225,
    battery_floor_mv: 12525,
    max_gate_mm: 2125,
    radio_class: 1,
};

pub const PROFILE_0626: StationProfile = StationProfile {
    station_id: 0x3272,
    district: 18,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1262,
    battery_floor_mv: 12554,
    max_gate_mm: 2142,
    radio_class: 2,
};

pub const PROFILE_0627: StationProfile = StationProfile {
    station_id: 0x3273,
    district: 19,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1299,
    battery_floor_mv: 12583,
    max_gate_mm: 2159,
    radio_class: 3,
};

pub const PROFILE_0628: StationProfile = StationProfile {
    station_id: 0x3274,
    district: 20,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1336,
    battery_floor_mv: 11212,
    max_gate_mm: 2176,
    radio_class: 4,
};

pub const PROFILE_0629: StationProfile = StationProfile {
    station_id: 0x3275,
    district: 21,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1373,
    battery_floor_mv: 11241,
    max_gate_mm: 2193,
    radio_class: 5,
};

pub const PROFILE_0630: StationProfile = StationProfile {
    station_id: 0x3276,
    district: 22,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1410,
    battery_floor_mv: 11270,
    max_gate_mm: 2210,
    radio_class: 6,
};

pub const PROFILE_0631: StationProfile = StationProfile {
    station_id: 0x3277,
    district: 23,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1447,
    battery_floor_mv: 11299,
    max_gate_mm: 2227,
    radio_class: 7,
};

pub const PROFILE_0632: StationProfile = StationProfile {
    station_id: 0x3278,
    district: 24,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1484,
    battery_floor_mv: 11328,
    max_gate_mm: 2244,
    radio_class: 0,
};

pub const PROFILE_0633: StationProfile = StationProfile {
    station_id: 0x3279,
    district: 25,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1521,
    battery_floor_mv: 11357,
    max_gate_mm: 2261,
    radio_class: 1,
};

pub const PROFILE_0634: StationProfile = StationProfile {
    station_id: 0x327a,
    district: 26,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1558,
    battery_floor_mv: 11386,
    max_gate_mm: 2278,
    radio_class: 2,
};

pub const PROFILE_0635: StationProfile = StationProfile {
    station_id: 0x327b,
    district: 27,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1595,
    battery_floor_mv: 11415,
    max_gate_mm: 2295,
    radio_class: 3,
};

pub const PROFILE_0636: StationProfile = StationProfile {
    station_id: 0x327c,
    district: 28,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1632,
    battery_floor_mv: 11444,
    max_gate_mm: 2312,
    radio_class: 4,
};

pub const PROFILE_0637: StationProfile = StationProfile {
    station_id: 0x327d,
    district: 29,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1669,
    battery_floor_mv: 11473,
    max_gate_mm: 2329,
    radio_class: 5,
};

pub const PROFILE_0638: StationProfile = StationProfile {
    station_id: 0x327e,
    district: 30,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1706,
    battery_floor_mv: 11502,
    max_gate_mm: 2346,
    radio_class: 6,
};

pub const PROFILE_0639: StationProfile = StationProfile {
    station_id: 0x327f,
    district: 31,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1743,
    battery_floor_mv: 11531,
    max_gate_mm: 2363,
    radio_class: 7,
};

pub const PROFILE_0640: StationProfile = StationProfile {
    station_id: 0x3280,
    district: 0,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1780,
    battery_floor_mv: 11560,
    max_gate_mm: 2380,
    radio_class: 0,
};

pub const PROFILE_0641: StationProfile = StationProfile {
    station_id: 0x3281,
    district: 1,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1817,
    battery_floor_mv: 11589,
    max_gate_mm: 2397,
    radio_class: 1,
};

pub const PROFILE_0642: StationProfile = StationProfile {
    station_id: 0x3282,
    district: 2,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1854,
    battery_floor_mv: 11618,
    max_gate_mm: 2414,
    radio_class: 2,
};

pub const PROFILE_0643: StationProfile = StationProfile {
    station_id: 0x3283,
    district: 3,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1891,
    battery_floor_mv: 11647,
    max_gate_mm: 2431,
    radio_class: 3,
};

pub const PROFILE_0644: StationProfile = StationProfile {
    station_id: 0x3284,
    district: 4,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1928,
    battery_floor_mv: 11676,
    max_gate_mm: 2448,
    radio_class: 4,
};

pub const PROFILE_0645: StationProfile = StationProfile {
    station_id: 0x3285,
    district: 5,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1965,
    battery_floor_mv: 11705,
    max_gate_mm: 2465,
    radio_class: 5,
};

pub const PROFILE_0646: StationProfile = StationProfile {
    station_id: 0x3286,
    district: 6,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2002,
    battery_floor_mv: 11734,
    max_gate_mm: 2482,
    radio_class: 6,
};

pub const PROFILE_0647: StationProfile = StationProfile {
    station_id: 0x3287,
    district: 7,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2039,
    battery_floor_mv: 11763,
    max_gate_mm: 2499,
    radio_class: 7,
};

pub const PROFILE_0648: StationProfile = StationProfile {
    station_id: 0x3288,
    district: 8,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2076,
    battery_floor_mv: 11792,
    max_gate_mm: 2516,
    radio_class: 0,
};

pub const PROFILE_0649: StationProfile = StationProfile {
    station_id: 0x3289,
    district: 9,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2113,
    battery_floor_mv: 11821,
    max_gate_mm: 2533,
    radio_class: 1,
};

pub const PROFILE_0650: StationProfile = StationProfile {
    station_id: 0x328a,
    district: 10,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2150,
    battery_floor_mv: 11850,
    max_gate_mm: 2550,
    radio_class: 2,
};

pub const PROFILE_0651: StationProfile = StationProfile {
    station_id: 0x328b,
    district: 11,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2187,
    battery_floor_mv: 11879,
    max_gate_mm: 2567,
    radio_class: 3,
};

pub const PROFILE_0652: StationProfile = StationProfile {
    station_id: 0x328c,
    district: 12,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2224,
    battery_floor_mv: 11908,
    max_gate_mm: 2584,
    radio_class: 4,
};

pub const PROFILE_0653: StationProfile = StationProfile {
    station_id: 0x328d,
    district: 13,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2261,
    battery_floor_mv: 11937,
    max_gate_mm: 2601,
    radio_class: 5,
};

pub const PROFILE_0654: StationProfile = StationProfile {
    station_id: 0x328e,
    district: 14,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2298,
    battery_floor_mv: 11966,
    max_gate_mm: 2618,
    radio_class: 6,
};

pub const PROFILE_0655: StationProfile = StationProfile {
    station_id: 0x328f,
    district: 15,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2335,
    battery_floor_mv: 11995,
    max_gate_mm: 2635,
    radio_class: 7,
};

pub const PROFILE_0656: StationProfile = StationProfile {
    station_id: 0x3290,
    district: 16,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2372,
    battery_floor_mv: 12024,
    max_gate_mm: 2652,
    radio_class: 0,
};

pub const PROFILE_0657: StationProfile = StationProfile {
    station_id: 0x3291,
    district: 17,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2409,
    battery_floor_mv: 12053,
    max_gate_mm: 2669,
    radio_class: 1,
};

pub const PROFILE_0658: StationProfile = StationProfile {
    station_id: 0x3292,
    district: 18,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2446,
    battery_floor_mv: 12082,
    max_gate_mm: 2686,
    radio_class: 2,
};

pub const PROFILE_0659: StationProfile = StationProfile {
    station_id: 0x3293,
    district: 19,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2483,
    battery_floor_mv: 12111,
    max_gate_mm: 2703,
    radio_class: 3,
};

pub const PROFILE_0660: StationProfile = StationProfile {
    station_id: 0x3294,
    district: 20,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2520,
    battery_floor_mv: 12140,
    max_gate_mm: 2720,
    radio_class: 4,
};

pub const PROFILE_0661: StationProfile = StationProfile {
    station_id: 0x3295,
    district: 21,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2557,
    battery_floor_mv: 12169,
    max_gate_mm: 2737,
    radio_class: 5,
};

pub const PROFILE_0662: StationProfile = StationProfile {
    station_id: 0x3296,
    district: 22,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2594,
    battery_floor_mv: 12198,
    max_gate_mm: 2754,
    radio_class: 6,
};

pub const PROFILE_0663: StationProfile = StationProfile {
    station_id: 0x3297,
    district: 23,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2631,
    battery_floor_mv: 12227,
    max_gate_mm: 2771,
    radio_class: 7,
};

pub const PROFILE_0664: StationProfile = StationProfile {
    station_id: 0x3298,
    district: 24,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2668,
    battery_floor_mv: 12256,
    max_gate_mm: 2788,
    radio_class: 0,
};

pub const PROFILE_0665: StationProfile = StationProfile {
    station_id: 0x3299,
    district: 25,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2705,
    battery_floor_mv: 12285,
    max_gate_mm: 2805,
    radio_class: 1,
};

pub const PROFILE_0666: StationProfile = StationProfile {
    station_id: 0x329a,
    district: 26,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2742,
    battery_floor_mv: 12314,
    max_gate_mm: 2822,
    radio_class: 2,
};

pub const PROFILE_0667: StationProfile = StationProfile {
    station_id: 0x329b,
    district: 27,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2779,
    battery_floor_mv: 12343,
    max_gate_mm: 2839,
    radio_class: 3,
};

pub const PROFILE_0668: StationProfile = StationProfile {
    station_id: 0x329c,
    district: 28,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 916,
    battery_floor_mv: 12372,
    max_gate_mm: 2856,
    radio_class: 4,
};

pub const PROFILE_0669: StationProfile = StationProfile {
    station_id: 0x329d,
    district: 29,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 953,
    battery_floor_mv: 12401,
    max_gate_mm: 2873,
    radio_class: 5,
};

pub const PROFILE_0670: StationProfile = StationProfile {
    station_id: 0x329e,
    district: 30,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 990,
    battery_floor_mv: 12430,
    max_gate_mm: 2890,
    radio_class: 6,
};

pub const PROFILE_0671: StationProfile = StationProfile {
    station_id: 0x329f,
    district: 31,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1027,
    battery_floor_mv: 12459,
    max_gate_mm: 2907,
    radio_class: 7,
};

pub const PROFILE_0672: StationProfile = StationProfile {
    station_id: 0x32a0,
    district: 0,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1064,
    battery_floor_mv: 12488,
    max_gate_mm: 2924,
    radio_class: 0,
};

pub const PROFILE_0673: StationProfile = StationProfile {
    station_id: 0x32a1,
    district: 1,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1101,
    battery_floor_mv: 12517,
    max_gate_mm: 2941,
    radio_class: 1,
};

pub const PROFILE_0674: StationProfile = StationProfile {
    station_id: 0x32a2,
    district: 2,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1138,
    battery_floor_mv: 12546,
    max_gate_mm: 2958,
    radio_class: 2,
};

pub const PROFILE_0675: StationProfile = StationProfile {
    station_id: 0x32a3,
    district: 3,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1175,
    battery_floor_mv: 12575,
    max_gate_mm: 2975,
    radio_class: 3,
};

pub const PROFILE_0676: StationProfile = StationProfile {
    station_id: 0x32a4,
    district: 4,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1212,
    battery_floor_mv: 11204,
    max_gate_mm: 2992,
    radio_class: 4,
};

pub const PROFILE_0677: StationProfile = StationProfile {
    station_id: 0x32a5,
    district: 5,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1249,
    battery_floor_mv: 11233,
    max_gate_mm: 709,
    radio_class: 5,
};

pub const PROFILE_0678: StationProfile = StationProfile {
    station_id: 0x32a6,
    district: 6,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1286,
    battery_floor_mv: 11262,
    max_gate_mm: 726,
    radio_class: 6,
};

pub const PROFILE_0679: StationProfile = StationProfile {
    station_id: 0x32a7,
    district: 7,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1323,
    battery_floor_mv: 11291,
    max_gate_mm: 743,
    radio_class: 7,
};

pub const PROFILE_0680: StationProfile = StationProfile {
    station_id: 0x32a8,
    district: 8,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1360,
    battery_floor_mv: 11320,
    max_gate_mm: 760,
    radio_class: 0,
};

pub const PROFILE_0681: StationProfile = StationProfile {
    station_id: 0x32a9,
    district: 9,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1397,
    battery_floor_mv: 11349,
    max_gate_mm: 777,
    radio_class: 1,
};

pub const PROFILE_0682: StationProfile = StationProfile {
    station_id: 0x32aa,
    district: 10,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1434,
    battery_floor_mv: 11378,
    max_gate_mm: 794,
    radio_class: 2,
};

pub const PROFILE_0683: StationProfile = StationProfile {
    station_id: 0x32ab,
    district: 11,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1471,
    battery_floor_mv: 11407,
    max_gate_mm: 811,
    radio_class: 3,
};

pub const PROFILE_0684: StationProfile = StationProfile {
    station_id: 0x32ac,
    district: 12,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1508,
    battery_floor_mv: 11436,
    max_gate_mm: 828,
    radio_class: 4,
};

pub const PROFILE_0685: StationProfile = StationProfile {
    station_id: 0x32ad,
    district: 13,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1545,
    battery_floor_mv: 11465,
    max_gate_mm: 845,
    radio_class: 5,
};

pub const PROFILE_0686: StationProfile = StationProfile {
    station_id: 0x32ae,
    district: 14,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1582,
    battery_floor_mv: 11494,
    max_gate_mm: 862,
    radio_class: 6,
};

pub const PROFILE_0687: StationProfile = StationProfile {
    station_id: 0x32af,
    district: 15,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1619,
    battery_floor_mv: 11523,
    max_gate_mm: 879,
    radio_class: 7,
};

pub const PROFILE_0688: StationProfile = StationProfile {
    station_id: 0x32b0,
    district: 16,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1656,
    battery_floor_mv: 11552,
    max_gate_mm: 896,
    radio_class: 0,
};

pub const PROFILE_0689: StationProfile = StationProfile {
    station_id: 0x32b1,
    district: 17,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1693,
    battery_floor_mv: 11581,
    max_gate_mm: 913,
    radio_class: 1,
};

pub const PROFILE_0690: StationProfile = StationProfile {
    station_id: 0x32b2,
    district: 18,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1730,
    battery_floor_mv: 11610,
    max_gate_mm: 930,
    radio_class: 2,
};

pub const PROFILE_0691: StationProfile = StationProfile {
    station_id: 0x32b3,
    district: 19,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1767,
    battery_floor_mv: 11639,
    max_gate_mm: 947,
    radio_class: 3,
};

pub const PROFILE_0692: StationProfile = StationProfile {
    station_id: 0x32b4,
    district: 20,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1804,
    battery_floor_mv: 11668,
    max_gate_mm: 964,
    radio_class: 4,
};

pub const PROFILE_0693: StationProfile = StationProfile {
    station_id: 0x32b5,
    district: 21,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1841,
    battery_floor_mv: 11697,
    max_gate_mm: 981,
    radio_class: 5,
};

pub const PROFILE_0694: StationProfile = StationProfile {
    station_id: 0x32b6,
    district: 22,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1878,
    battery_floor_mv: 11726,
    max_gate_mm: 998,
    radio_class: 6,
};

pub const PROFILE_0695: StationProfile = StationProfile {
    station_id: 0x32b7,
    district: 23,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1915,
    battery_floor_mv: 11755,
    max_gate_mm: 1015,
    radio_class: 7,
};

pub const PROFILE_0696: StationProfile = StationProfile {
    station_id: 0x32b8,
    district: 24,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1952,
    battery_floor_mv: 11784,
    max_gate_mm: 1032,
    radio_class: 0,
};

pub const PROFILE_0697: StationProfile = StationProfile {
    station_id: 0x32b9,
    district: 25,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1989,
    battery_floor_mv: 11813,
    max_gate_mm: 1049,
    radio_class: 1,
};

pub const PROFILE_0698: StationProfile = StationProfile {
    station_id: 0x32ba,
    district: 26,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2026,
    battery_floor_mv: 11842,
    max_gate_mm: 1066,
    radio_class: 2,
};

pub const PROFILE_0699: StationProfile = StationProfile {
    station_id: 0x32bb,
    district: 27,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2063,
    battery_floor_mv: 11871,
    max_gate_mm: 1083,
    radio_class: 3,
};

pub const PROFILE_0700: StationProfile = StationProfile {
    station_id: 0x32bc,
    district: 28,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2100,
    battery_floor_mv: 11900,
    max_gate_mm: 1100,
    radio_class: 4,
};

pub const PROFILE_0701: StationProfile = StationProfile {
    station_id: 0x32bd,
    district: 29,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2137,
    battery_floor_mv: 11929,
    max_gate_mm: 1117,
    radio_class: 5,
};

pub const PROFILE_0702: StationProfile = StationProfile {
    station_id: 0x32be,
    district: 30,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2174,
    battery_floor_mv: 11958,
    max_gate_mm: 1134,
    radio_class: 6,
};

pub const PROFILE_0703: StationProfile = StationProfile {
    station_id: 0x32bf,
    district: 31,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2211,
    battery_floor_mv: 11987,
    max_gate_mm: 1151,
    radio_class: 7,
};

pub const PROFILE_0704: StationProfile = StationProfile {
    station_id: 0x32c0,
    district: 0,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2248,
    battery_floor_mv: 12016,
    max_gate_mm: 1168,
    radio_class: 0,
};

pub const PROFILE_0705: StationProfile = StationProfile {
    station_id: 0x32c1,
    district: 1,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2285,
    battery_floor_mv: 12045,
    max_gate_mm: 1185,
    radio_class: 1,
};

pub const PROFILE_0706: StationProfile = StationProfile {
    station_id: 0x32c2,
    district: 2,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2322,
    battery_floor_mv: 12074,
    max_gate_mm: 1202,
    radio_class: 2,
};

pub const PROFILE_0707: StationProfile = StationProfile {
    station_id: 0x32c3,
    district: 3,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2359,
    battery_floor_mv: 12103,
    max_gate_mm: 1219,
    radio_class: 3,
};

pub const PROFILE_0708: StationProfile = StationProfile {
    station_id: 0x32c4,
    district: 4,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2396,
    battery_floor_mv: 12132,
    max_gate_mm: 1236,
    radio_class: 4,
};

pub const PROFILE_0709: StationProfile = StationProfile {
    station_id: 0x32c5,
    district: 5,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2433,
    battery_floor_mv: 12161,
    max_gate_mm: 1253,
    radio_class: 5,
};

pub const PROFILE_0710: StationProfile = StationProfile {
    station_id: 0x32c6,
    district: 6,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2470,
    battery_floor_mv: 12190,
    max_gate_mm: 1270,
    radio_class: 6,
};

pub const PROFILE_0711: StationProfile = StationProfile {
    station_id: 0x32c7,
    district: 7,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2507,
    battery_floor_mv: 12219,
    max_gate_mm: 1287,
    radio_class: 7,
};

pub const PROFILE_0712: StationProfile = StationProfile {
    station_id: 0x32c8,
    district: 8,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2544,
    battery_floor_mv: 12248,
    max_gate_mm: 1304,
    radio_class: 0,
};

pub const PROFILE_0713: StationProfile = StationProfile {
    station_id: 0x32c9,
    district: 9,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2581,
    battery_floor_mv: 12277,
    max_gate_mm: 1321,
    radio_class: 1,
};

pub const PROFILE_0714: StationProfile = StationProfile {
    station_id: 0x32ca,
    district: 10,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2618,
    battery_floor_mv: 12306,
    max_gate_mm: 1338,
    radio_class: 2,
};

pub const PROFILE_0715: StationProfile = StationProfile {
    station_id: 0x32cb,
    district: 11,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2655,
    battery_floor_mv: 12335,
    max_gate_mm: 1355,
    radio_class: 3,
};

pub const PROFILE_0716: StationProfile = StationProfile {
    station_id: 0x32cc,
    district: 12,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2692,
    battery_floor_mv: 12364,
    max_gate_mm: 1372,
    radio_class: 4,
};

pub const PROFILE_0717: StationProfile = StationProfile {
    station_id: 0x32cd,
    district: 13,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2729,
    battery_floor_mv: 12393,
    max_gate_mm: 1389,
    radio_class: 5,
};

pub const PROFILE_0718: StationProfile = StationProfile {
    station_id: 0x32ce,
    district: 14,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2766,
    battery_floor_mv: 12422,
    max_gate_mm: 1406,
    radio_class: 6,
};

pub const PROFILE_0719: StationProfile = StationProfile {
    station_id: 0x32cf,
    district: 15,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 903,
    battery_floor_mv: 12451,
    max_gate_mm: 1423,
    radio_class: 7,
};

pub const PROFILE_0720: StationProfile = StationProfile {
    station_id: 0x32d0,
    district: 16,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 940,
    battery_floor_mv: 12480,
    max_gate_mm: 1440,
    radio_class: 0,
};

pub const PROFILE_0721: StationProfile = StationProfile {
    station_id: 0x32d1,
    district: 17,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 977,
    battery_floor_mv: 12509,
    max_gate_mm: 1457,
    radio_class: 1,
};

pub const PROFILE_0722: StationProfile = StationProfile {
    station_id: 0x32d2,
    district: 18,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1014,
    battery_floor_mv: 12538,
    max_gate_mm: 1474,
    radio_class: 2,
};

pub const PROFILE_0723: StationProfile = StationProfile {
    station_id: 0x32d3,
    district: 19,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1051,
    battery_floor_mv: 12567,
    max_gate_mm: 1491,
    radio_class: 3,
};

pub const PROFILE_0724: StationProfile = StationProfile {
    station_id: 0x32d4,
    district: 20,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1088,
    battery_floor_mv: 12596,
    max_gate_mm: 1508,
    radio_class: 4,
};

pub const PROFILE_0725: StationProfile = StationProfile {
    station_id: 0x32d5,
    district: 21,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1125,
    battery_floor_mv: 11225,
    max_gate_mm: 1525,
    radio_class: 5,
};

pub const PROFILE_0726: StationProfile = StationProfile {
    station_id: 0x32d6,
    district: 22,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1162,
    battery_floor_mv: 11254,
    max_gate_mm: 1542,
    radio_class: 6,
};

pub const PROFILE_0727: StationProfile = StationProfile {
    station_id: 0x32d7,
    district: 23,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1199,
    battery_floor_mv: 11283,
    max_gate_mm: 1559,
    radio_class: 7,
};

pub const PROFILE_0728: StationProfile = StationProfile {
    station_id: 0x32d8,
    district: 24,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1236,
    battery_floor_mv: 11312,
    max_gate_mm: 1576,
    radio_class: 0,
};

pub const PROFILE_0729: StationProfile = StationProfile {
    station_id: 0x32d9,
    district: 25,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1273,
    battery_floor_mv: 11341,
    max_gate_mm: 1593,
    radio_class: 1,
};

pub const PROFILE_0730: StationProfile = StationProfile {
    station_id: 0x32da,
    district: 26,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1310,
    battery_floor_mv: 11370,
    max_gate_mm: 1610,
    radio_class: 2,
};

pub const PROFILE_0731: StationProfile = StationProfile {
    station_id: 0x32db,
    district: 27,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1347,
    battery_floor_mv: 11399,
    max_gate_mm: 1627,
    radio_class: 3,
};

pub const PROFILE_0732: StationProfile = StationProfile {
    station_id: 0x32dc,
    district: 28,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1384,
    battery_floor_mv: 11428,
    max_gate_mm: 1644,
    radio_class: 4,
};

pub const PROFILE_0733: StationProfile = StationProfile {
    station_id: 0x32dd,
    district: 29,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1421,
    battery_floor_mv: 11457,
    max_gate_mm: 1661,
    radio_class: 5,
};

pub const PROFILE_0734: StationProfile = StationProfile {
    station_id: 0x32de,
    district: 30,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1458,
    battery_floor_mv: 11486,
    max_gate_mm: 1678,
    radio_class: 6,
};

pub const PROFILE_0735: StationProfile = StationProfile {
    station_id: 0x32df,
    district: 31,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1495,
    battery_floor_mv: 11515,
    max_gate_mm: 1695,
    radio_class: 7,
};

pub const PROFILE_0736: StationProfile = StationProfile {
    station_id: 0x32e0,
    district: 0,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1532,
    battery_floor_mv: 11544,
    max_gate_mm: 1712,
    radio_class: 0,
};

pub const PROFILE_0737: StationProfile = StationProfile {
    station_id: 0x32e1,
    district: 1,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1569,
    battery_floor_mv: 11573,
    max_gate_mm: 1729,
    radio_class: 1,
};

pub const PROFILE_0738: StationProfile = StationProfile {
    station_id: 0x32e2,
    district: 2,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1606,
    battery_floor_mv: 11602,
    max_gate_mm: 1746,
    radio_class: 2,
};

pub const PROFILE_0739: StationProfile = StationProfile {
    station_id: 0x32e3,
    district: 3,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1643,
    battery_floor_mv: 11631,
    max_gate_mm: 1763,
    radio_class: 3,
};

pub const PROFILE_0740: StationProfile = StationProfile {
    station_id: 0x32e4,
    district: 4,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1680,
    battery_floor_mv: 11660,
    max_gate_mm: 1780,
    radio_class: 4,
};

pub const PROFILE_0741: StationProfile = StationProfile {
    station_id: 0x32e5,
    district: 5,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1717,
    battery_floor_mv: 11689,
    max_gate_mm: 1797,
    radio_class: 5,
};

pub const PROFILE_0742: StationProfile = StationProfile {
    station_id: 0x32e6,
    district: 6,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1754,
    battery_floor_mv: 11718,
    max_gate_mm: 1814,
    radio_class: 6,
};

pub const PROFILE_0743: StationProfile = StationProfile {
    station_id: 0x32e7,
    district: 7,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1791,
    battery_floor_mv: 11747,
    max_gate_mm: 1831,
    radio_class: 7,
};

pub const PROFILE_0744: StationProfile = StationProfile {
    station_id: 0x32e8,
    district: 8,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1828,
    battery_floor_mv: 11776,
    max_gate_mm: 1848,
    radio_class: 0,
};

pub const PROFILE_0745: StationProfile = StationProfile {
    station_id: 0x32e9,
    district: 9,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1865,
    battery_floor_mv: 11805,
    max_gate_mm: 1865,
    radio_class: 1,
};

pub const PROFILE_0746: StationProfile = StationProfile {
    station_id: 0x32ea,
    district: 10,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1902,
    battery_floor_mv: 11834,
    max_gate_mm: 1882,
    radio_class: 2,
};

pub const PROFILE_0747: StationProfile = StationProfile {
    station_id: 0x32eb,
    district: 11,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1939,
    battery_floor_mv: 11863,
    max_gate_mm: 1899,
    radio_class: 3,
};

pub const PROFILE_0748: StationProfile = StationProfile {
    station_id: 0x32ec,
    district: 12,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1976,
    battery_floor_mv: 11892,
    max_gate_mm: 1916,
    radio_class: 4,
};

pub const PROFILE_0749: StationProfile = StationProfile {
    station_id: 0x32ed,
    district: 13,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2013,
    battery_floor_mv: 11921,
    max_gate_mm: 1933,
    radio_class: 5,
};

pub const PROFILE_0750: StationProfile = StationProfile {
    station_id: 0x32ee,
    district: 14,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2050,
    battery_floor_mv: 11950,
    max_gate_mm: 1950,
    radio_class: 6,
};

pub const PROFILE_0751: StationProfile = StationProfile {
    station_id: 0x32ef,
    district: 15,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2087,
    battery_floor_mv: 11979,
    max_gate_mm: 1967,
    radio_class: 7,
};

pub const PROFILE_0752: StationProfile = StationProfile {
    station_id: 0x32f0,
    district: 16,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2124,
    battery_floor_mv: 12008,
    max_gate_mm: 1984,
    radio_class: 0,
};

pub const PROFILE_0753: StationProfile = StationProfile {
    station_id: 0x32f1,
    district: 17,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2161,
    battery_floor_mv: 12037,
    max_gate_mm: 2001,
    radio_class: 1,
};

pub const PROFILE_0754: StationProfile = StationProfile {
    station_id: 0x32f2,
    district: 18,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2198,
    battery_floor_mv: 12066,
    max_gate_mm: 2018,
    radio_class: 2,
};

pub const PROFILE_0755: StationProfile = StationProfile {
    station_id: 0x32f3,
    district: 19,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2235,
    battery_floor_mv: 12095,
    max_gate_mm: 2035,
    radio_class: 3,
};

pub const PROFILE_0756: StationProfile = StationProfile {
    station_id: 0x32f4,
    district: 20,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2272,
    battery_floor_mv: 12124,
    max_gate_mm: 2052,
    radio_class: 4,
};

pub const PROFILE_0757: StationProfile = StationProfile {
    station_id: 0x32f5,
    district: 21,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2309,
    battery_floor_mv: 12153,
    max_gate_mm: 2069,
    radio_class: 5,
};

pub const PROFILE_0758: StationProfile = StationProfile {
    station_id: 0x32f6,
    district: 22,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2346,
    battery_floor_mv: 12182,
    max_gate_mm: 2086,
    radio_class: 6,
};

pub const PROFILE_0759: StationProfile = StationProfile {
    station_id: 0x32f7,
    district: 23,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2383,
    battery_floor_mv: 12211,
    max_gate_mm: 2103,
    radio_class: 7,
};

pub const PROFILE_0760: StationProfile = StationProfile {
    station_id: 0x32f8,
    district: 24,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2420,
    battery_floor_mv: 12240,
    max_gate_mm: 2120,
    radio_class: 0,
};

pub const PROFILE_0761: StationProfile = StationProfile {
    station_id: 0x32f9,
    district: 25,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2457,
    battery_floor_mv: 12269,
    max_gate_mm: 2137,
    radio_class: 1,
};

pub const PROFILE_0762: StationProfile = StationProfile {
    station_id: 0x32fa,
    district: 26,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2494,
    battery_floor_mv: 12298,
    max_gate_mm: 2154,
    radio_class: 2,
};

pub const PROFILE_0763: StationProfile = StationProfile {
    station_id: 0x32fb,
    district: 27,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2531,
    battery_floor_mv: 12327,
    max_gate_mm: 2171,
    radio_class: 3,
};

pub const PROFILE_0764: StationProfile = StationProfile {
    station_id: 0x32fc,
    district: 28,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2568,
    battery_floor_mv: 12356,
    max_gate_mm: 2188,
    radio_class: 4,
};

pub const PROFILE_0765: StationProfile = StationProfile {
    station_id: 0x32fd,
    district: 29,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2605,
    battery_floor_mv: 12385,
    max_gate_mm: 2205,
    radio_class: 5,
};

pub const PROFILE_0766: StationProfile = StationProfile {
    station_id: 0x32fe,
    district: 30,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2642,
    battery_floor_mv: 12414,
    max_gate_mm: 2222,
    radio_class: 6,
};

pub const PROFILE_0767: StationProfile = StationProfile {
    station_id: 0x32ff,
    district: 31,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2679,
    battery_floor_mv: 12443,
    max_gate_mm: 2239,
    radio_class: 7,
};

pub const PROFILE_0768: StationProfile = StationProfile {
    station_id: 0x3300,
    district: 0,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2716,
    battery_floor_mv: 12472,
    max_gate_mm: 2256,
    radio_class: 0,
};

pub const PROFILE_0769: StationProfile = StationProfile {
    station_id: 0x3301,
    district: 1,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2753,
    battery_floor_mv: 12501,
    max_gate_mm: 2273,
    radio_class: 1,
};

pub const PROFILE_0770: StationProfile = StationProfile {
    station_id: 0x3302,
    district: 2,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2790,
    battery_floor_mv: 12530,
    max_gate_mm: 2290,
    radio_class: 2,
};

pub const PROFILE_0771: StationProfile = StationProfile {
    station_id: 0x3303,
    district: 3,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 927,
    battery_floor_mv: 12559,
    max_gate_mm: 2307,
    radio_class: 3,
};

pub const PROFILE_0772: StationProfile = StationProfile {
    station_id: 0x3304,
    district: 4,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 964,
    battery_floor_mv: 12588,
    max_gate_mm: 2324,
    radio_class: 4,
};

pub const PROFILE_0773: StationProfile = StationProfile {
    station_id: 0x3305,
    district: 5,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1001,
    battery_floor_mv: 11217,
    max_gate_mm: 2341,
    radio_class: 5,
};

pub const PROFILE_0774: StationProfile = StationProfile {
    station_id: 0x3306,
    district: 6,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1038,
    battery_floor_mv: 11246,
    max_gate_mm: 2358,
    radio_class: 6,
};

pub const PROFILE_0775: StationProfile = StationProfile {
    station_id: 0x3307,
    district: 7,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1075,
    battery_floor_mv: 11275,
    max_gate_mm: 2375,
    radio_class: 7,
};

pub const PROFILE_0776: StationProfile = StationProfile {
    station_id: 0x3308,
    district: 8,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1112,
    battery_floor_mv: 11304,
    max_gate_mm: 2392,
    radio_class: 0,
};

pub const PROFILE_0777: StationProfile = StationProfile {
    station_id: 0x3309,
    district: 9,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1149,
    battery_floor_mv: 11333,
    max_gate_mm: 2409,
    radio_class: 1,
};

pub const PROFILE_0778: StationProfile = StationProfile {
    station_id: 0x330a,
    district: 10,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1186,
    battery_floor_mv: 11362,
    max_gate_mm: 2426,
    radio_class: 2,
};

pub const PROFILE_0779: StationProfile = StationProfile {
    station_id: 0x330b,
    district: 11,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1223,
    battery_floor_mv: 11391,
    max_gate_mm: 2443,
    radio_class: 3,
};

pub const PROFILE_0780: StationProfile = StationProfile {
    station_id: 0x330c,
    district: 12,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1260,
    battery_floor_mv: 11420,
    max_gate_mm: 2460,
    radio_class: 4,
};

pub const PROFILE_0781: StationProfile = StationProfile {
    station_id: 0x330d,
    district: 13,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1297,
    battery_floor_mv: 11449,
    max_gate_mm: 2477,
    radio_class: 5,
};

pub const PROFILE_0782: StationProfile = StationProfile {
    station_id: 0x330e,
    district: 14,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1334,
    battery_floor_mv: 11478,
    max_gate_mm: 2494,
    radio_class: 6,
};

pub const PROFILE_0783: StationProfile = StationProfile {
    station_id: 0x330f,
    district: 15,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1371,
    battery_floor_mv: 11507,
    max_gate_mm: 2511,
    radio_class: 7,
};

pub const PROFILE_0784: StationProfile = StationProfile {
    station_id: 0x3310,
    district: 16,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1408,
    battery_floor_mv: 11536,
    max_gate_mm: 2528,
    radio_class: 0,
};

pub const PROFILE_0785: StationProfile = StationProfile {
    station_id: 0x3311,
    district: 17,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1445,
    battery_floor_mv: 11565,
    max_gate_mm: 2545,
    radio_class: 1,
};

pub const PROFILE_0786: StationProfile = StationProfile {
    station_id: 0x3312,
    district: 18,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1482,
    battery_floor_mv: 11594,
    max_gate_mm: 2562,
    radio_class: 2,
};

pub const PROFILE_0787: StationProfile = StationProfile {
    station_id: 0x3313,
    district: 19,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1519,
    battery_floor_mv: 11623,
    max_gate_mm: 2579,
    radio_class: 3,
};

pub const PROFILE_0788: StationProfile = StationProfile {
    station_id: 0x3314,
    district: 20,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1556,
    battery_floor_mv: 11652,
    max_gate_mm: 2596,
    radio_class: 4,
};

pub const PROFILE_0789: StationProfile = StationProfile {
    station_id: 0x3315,
    district: 21,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1593,
    battery_floor_mv: 11681,
    max_gate_mm: 2613,
    radio_class: 5,
};

pub const PROFILE_0790: StationProfile = StationProfile {
    station_id: 0x3316,
    district: 22,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1630,
    battery_floor_mv: 11710,
    max_gate_mm: 2630,
    radio_class: 6,
};

pub const PROFILE_0791: StationProfile = StationProfile {
    station_id: 0x3317,
    district: 23,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1667,
    battery_floor_mv: 11739,
    max_gate_mm: 2647,
    radio_class: 7,
};

pub const PROFILE_0792: StationProfile = StationProfile {
    station_id: 0x3318,
    district: 24,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1704,
    battery_floor_mv: 11768,
    max_gate_mm: 2664,
    radio_class: 0,
};

pub const PROFILE_0793: StationProfile = StationProfile {
    station_id: 0x3319,
    district: 25,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1741,
    battery_floor_mv: 11797,
    max_gate_mm: 2681,
    radio_class: 1,
};

pub const PROFILE_0794: StationProfile = StationProfile {
    station_id: 0x331a,
    district: 26,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1778,
    battery_floor_mv: 11826,
    max_gate_mm: 2698,
    radio_class: 2,
};

pub const PROFILE_0795: StationProfile = StationProfile {
    station_id: 0x331b,
    district: 27,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1815,
    battery_floor_mv: 11855,
    max_gate_mm: 2715,
    radio_class: 3,
};

pub const PROFILE_0796: StationProfile = StationProfile {
    station_id: 0x331c,
    district: 28,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1852,
    battery_floor_mv: 11884,
    max_gate_mm: 2732,
    radio_class: 4,
};

pub const PROFILE_0797: StationProfile = StationProfile {
    station_id: 0x331d,
    district: 29,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1889,
    battery_floor_mv: 11913,
    max_gate_mm: 2749,
    radio_class: 5,
};

pub const PROFILE_0798: StationProfile = StationProfile {
    station_id: 0x331e,
    district: 30,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1926,
    battery_floor_mv: 11942,
    max_gate_mm: 2766,
    radio_class: 6,
};

pub const PROFILE_0799: StationProfile = StationProfile {
    station_id: 0x331f,
    district: 31,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1963,
    battery_floor_mv: 11971,
    max_gate_mm: 2783,
    radio_class: 7,
};

pub const PROFILE_0800: StationProfile = StationProfile {
    station_id: 0x3320,
    district: 0,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2000,
    battery_floor_mv: 12000,
    max_gate_mm: 2800,
    radio_class: 0,
};

pub const PROFILE_0801: StationProfile = StationProfile {
    station_id: 0x3321,
    district: 1,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2037,
    battery_floor_mv: 12029,
    max_gate_mm: 2817,
    radio_class: 1,
};

pub const PROFILE_0802: StationProfile = StationProfile {
    station_id: 0x3322,
    district: 2,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2074,
    battery_floor_mv: 12058,
    max_gate_mm: 2834,
    radio_class: 2,
};

pub const PROFILE_0803: StationProfile = StationProfile {
    station_id: 0x3323,
    district: 3,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2111,
    battery_floor_mv: 12087,
    max_gate_mm: 2851,
    radio_class: 3,
};

pub const PROFILE_0804: StationProfile = StationProfile {
    station_id: 0x3324,
    district: 4,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2148,
    battery_floor_mv: 12116,
    max_gate_mm: 2868,
    radio_class: 4,
};

pub const PROFILE_0805: StationProfile = StationProfile {
    station_id: 0x3325,
    district: 5,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2185,
    battery_floor_mv: 12145,
    max_gate_mm: 2885,
    radio_class: 5,
};

pub const PROFILE_0806: StationProfile = StationProfile {
    station_id: 0x3326,
    district: 6,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2222,
    battery_floor_mv: 12174,
    max_gate_mm: 2902,
    radio_class: 6,
};

pub const PROFILE_0807: StationProfile = StationProfile {
    station_id: 0x3327,
    district: 7,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2259,
    battery_floor_mv: 12203,
    max_gate_mm: 2919,
    radio_class: 7,
};

pub const PROFILE_0808: StationProfile = StationProfile {
    station_id: 0x3328,
    district: 8,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2296,
    battery_floor_mv: 12232,
    max_gate_mm: 2936,
    radio_class: 0,
};

pub const PROFILE_0809: StationProfile = StationProfile {
    station_id: 0x3329,
    district: 9,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2333,
    battery_floor_mv: 12261,
    max_gate_mm: 2953,
    radio_class: 1,
};

pub const PROFILE_0810: StationProfile = StationProfile {
    station_id: 0x332a,
    district: 10,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2370,
    battery_floor_mv: 12290,
    max_gate_mm: 2970,
    radio_class: 2,
};

pub const PROFILE_0811: StationProfile = StationProfile {
    station_id: 0x332b,
    district: 11,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2407,
    battery_floor_mv: 12319,
    max_gate_mm: 2987,
    radio_class: 3,
};

pub const PROFILE_0812: StationProfile = StationProfile {
    station_id: 0x332c,
    district: 12,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2444,
    battery_floor_mv: 12348,
    max_gate_mm: 704,
    radio_class: 4,
};

pub const PROFILE_0813: StationProfile = StationProfile {
    station_id: 0x332d,
    district: 13,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2481,
    battery_floor_mv: 12377,
    max_gate_mm: 721,
    radio_class: 5,
};

pub const PROFILE_0814: StationProfile = StationProfile {
    station_id: 0x332e,
    district: 14,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2518,
    battery_floor_mv: 12406,
    max_gate_mm: 738,
    radio_class: 6,
};

pub const PROFILE_0815: StationProfile = StationProfile {
    station_id: 0x332f,
    district: 15,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2555,
    battery_floor_mv: 12435,
    max_gate_mm: 755,
    radio_class: 7,
};

pub const PROFILE_0816: StationProfile = StationProfile {
    station_id: 0x3330,
    district: 16,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2592,
    battery_floor_mv: 12464,
    max_gate_mm: 772,
    radio_class: 0,
};

pub const PROFILE_0817: StationProfile = StationProfile {
    station_id: 0x3331,
    district: 17,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2629,
    battery_floor_mv: 12493,
    max_gate_mm: 789,
    radio_class: 1,
};

pub const PROFILE_0818: StationProfile = StationProfile {
    station_id: 0x3332,
    district: 18,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2666,
    battery_floor_mv: 12522,
    max_gate_mm: 806,
    radio_class: 2,
};

pub const PROFILE_0819: StationProfile = StationProfile {
    station_id: 0x3333,
    district: 19,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2703,
    battery_floor_mv: 12551,
    max_gate_mm: 823,
    radio_class: 3,
};

pub const PROFILE_0820: StationProfile = StationProfile {
    station_id: 0x3334,
    district: 20,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2740,
    battery_floor_mv: 12580,
    max_gate_mm: 840,
    radio_class: 4,
};

pub const PROFILE_0821: StationProfile = StationProfile {
    station_id: 0x3335,
    district: 21,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2777,
    battery_floor_mv: 11209,
    max_gate_mm: 857,
    radio_class: 5,
};

pub const PROFILE_0822: StationProfile = StationProfile {
    station_id: 0x3336,
    district: 22,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 914,
    battery_floor_mv: 11238,
    max_gate_mm: 874,
    radio_class: 6,
};

pub const PROFILE_0823: StationProfile = StationProfile {
    station_id: 0x3337,
    district: 23,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 951,
    battery_floor_mv: 11267,
    max_gate_mm: 891,
    radio_class: 7,
};

pub const PROFILE_0824: StationProfile = StationProfile {
    station_id: 0x3338,
    district: 24,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 988,
    battery_floor_mv: 11296,
    max_gate_mm: 908,
    radio_class: 0,
};

pub const PROFILE_0825: StationProfile = StationProfile {
    station_id: 0x3339,
    district: 25,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1025,
    battery_floor_mv: 11325,
    max_gate_mm: 925,
    radio_class: 1,
};

pub const PROFILE_0826: StationProfile = StationProfile {
    station_id: 0x333a,
    district: 26,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1062,
    battery_floor_mv: 11354,
    max_gate_mm: 942,
    radio_class: 2,
};

pub const PROFILE_0827: StationProfile = StationProfile {
    station_id: 0x333b,
    district: 27,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1099,
    battery_floor_mv: 11383,
    max_gate_mm: 959,
    radio_class: 3,
};

pub const PROFILE_0828: StationProfile = StationProfile {
    station_id: 0x333c,
    district: 28,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1136,
    battery_floor_mv: 11412,
    max_gate_mm: 976,
    radio_class: 4,
};

pub const PROFILE_0829: StationProfile = StationProfile {
    station_id: 0x333d,
    district: 29,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1173,
    battery_floor_mv: 11441,
    max_gate_mm: 993,
    radio_class: 5,
};

pub const PROFILE_0830: StationProfile = StationProfile {
    station_id: 0x333e,
    district: 30,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1210,
    battery_floor_mv: 11470,
    max_gate_mm: 1010,
    radio_class: 6,
};

pub const PROFILE_0831: StationProfile = StationProfile {
    station_id: 0x333f,
    district: 31,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1247,
    battery_floor_mv: 11499,
    max_gate_mm: 1027,
    radio_class: 7,
};

pub const PROFILE_0832: StationProfile = StationProfile {
    station_id: 0x3340,
    district: 0,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1284,
    battery_floor_mv: 11528,
    max_gate_mm: 1044,
    radio_class: 0,
};

pub const PROFILE_0833: StationProfile = StationProfile {
    station_id: 0x3341,
    district: 1,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1321,
    battery_floor_mv: 11557,
    max_gate_mm: 1061,
    radio_class: 1,
};

pub const PROFILE_0834: StationProfile = StationProfile {
    station_id: 0x3342,
    district: 2,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1358,
    battery_floor_mv: 11586,
    max_gate_mm: 1078,
    radio_class: 2,
};

pub const PROFILE_0835: StationProfile = StationProfile {
    station_id: 0x3343,
    district: 3,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1395,
    battery_floor_mv: 11615,
    max_gate_mm: 1095,
    radio_class: 3,
};

pub const PROFILE_0836: StationProfile = StationProfile {
    station_id: 0x3344,
    district: 4,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1432,
    battery_floor_mv: 11644,
    max_gate_mm: 1112,
    radio_class: 4,
};

pub const PROFILE_0837: StationProfile = StationProfile {
    station_id: 0x3345,
    district: 5,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1469,
    battery_floor_mv: 11673,
    max_gate_mm: 1129,
    radio_class: 5,
};

pub const PROFILE_0838: StationProfile = StationProfile {
    station_id: 0x3346,
    district: 6,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1506,
    battery_floor_mv: 11702,
    max_gate_mm: 1146,
    radio_class: 6,
};

pub const PROFILE_0839: StationProfile = StationProfile {
    station_id: 0x3347,
    district: 7,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1543,
    battery_floor_mv: 11731,
    max_gate_mm: 1163,
    radio_class: 7,
};

pub const PROFILE_0840: StationProfile = StationProfile {
    station_id: 0x3348,
    district: 8,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1580,
    battery_floor_mv: 11760,
    max_gate_mm: 1180,
    radio_class: 0,
};

pub const PROFILE_0841: StationProfile = StationProfile {
    station_id: 0x3349,
    district: 9,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1617,
    battery_floor_mv: 11789,
    max_gate_mm: 1197,
    radio_class: 1,
};

pub const PROFILE_0842: StationProfile = StationProfile {
    station_id: 0x334a,
    district: 10,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1654,
    battery_floor_mv: 11818,
    max_gate_mm: 1214,
    radio_class: 2,
};

pub const PROFILE_0843: StationProfile = StationProfile {
    station_id: 0x334b,
    district: 11,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1691,
    battery_floor_mv: 11847,
    max_gate_mm: 1231,
    radio_class: 3,
};

pub const PROFILE_0844: StationProfile = StationProfile {
    station_id: 0x334c,
    district: 12,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1728,
    battery_floor_mv: 11876,
    max_gate_mm: 1248,
    radio_class: 4,
};

pub const PROFILE_0845: StationProfile = StationProfile {
    station_id: 0x334d,
    district: 13,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1765,
    battery_floor_mv: 11905,
    max_gate_mm: 1265,
    radio_class: 5,
};

pub const PROFILE_0846: StationProfile = StationProfile {
    station_id: 0x334e,
    district: 14,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1802,
    battery_floor_mv: 11934,
    max_gate_mm: 1282,
    radio_class: 6,
};

pub const PROFILE_0847: StationProfile = StationProfile {
    station_id: 0x334f,
    district: 15,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1839,
    battery_floor_mv: 11963,
    max_gate_mm: 1299,
    radio_class: 7,
};

pub const PROFILE_0848: StationProfile = StationProfile {
    station_id: 0x3350,
    district: 16,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1876,
    battery_floor_mv: 11992,
    max_gate_mm: 1316,
    radio_class: 0,
};

pub const PROFILE_0849: StationProfile = StationProfile {
    station_id: 0x3351,
    district: 17,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1913,
    battery_floor_mv: 12021,
    max_gate_mm: 1333,
    radio_class: 1,
};

pub const PROFILE_0850: StationProfile = StationProfile {
    station_id: 0x3352,
    district: 18,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1950,
    battery_floor_mv: 12050,
    max_gate_mm: 1350,
    radio_class: 2,
};

pub const PROFILE_0851: StationProfile = StationProfile {
    station_id: 0x3353,
    district: 19,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1987,
    battery_floor_mv: 12079,
    max_gate_mm: 1367,
    radio_class: 3,
};

pub const PROFILE_0852: StationProfile = StationProfile {
    station_id: 0x3354,
    district: 20,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2024,
    battery_floor_mv: 12108,
    max_gate_mm: 1384,
    radio_class: 4,
};

pub const PROFILE_0853: StationProfile = StationProfile {
    station_id: 0x3355,
    district: 21,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2061,
    battery_floor_mv: 12137,
    max_gate_mm: 1401,
    radio_class: 5,
};

pub const PROFILE_0854: StationProfile = StationProfile {
    station_id: 0x3356,
    district: 22,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2098,
    battery_floor_mv: 12166,
    max_gate_mm: 1418,
    radio_class: 6,
};

pub const PROFILE_0855: StationProfile = StationProfile {
    station_id: 0x3357,
    district: 23,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2135,
    battery_floor_mv: 12195,
    max_gate_mm: 1435,
    radio_class: 7,
};

pub const PROFILE_0856: StationProfile = StationProfile {
    station_id: 0x3358,
    district: 24,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2172,
    battery_floor_mv: 12224,
    max_gate_mm: 1452,
    radio_class: 0,
};

pub const PROFILE_0857: StationProfile = StationProfile {
    station_id: 0x3359,
    district: 25,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2209,
    battery_floor_mv: 12253,
    max_gate_mm: 1469,
    radio_class: 1,
};

pub const PROFILE_0858: StationProfile = StationProfile {
    station_id: 0x335a,
    district: 26,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2246,
    battery_floor_mv: 12282,
    max_gate_mm: 1486,
    radio_class: 2,
};

pub const PROFILE_0859: StationProfile = StationProfile {
    station_id: 0x335b,
    district: 27,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2283,
    battery_floor_mv: 12311,
    max_gate_mm: 1503,
    radio_class: 3,
};

pub const PROFILE_0860: StationProfile = StationProfile {
    station_id: 0x335c,
    district: 28,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2320,
    battery_floor_mv: 12340,
    max_gate_mm: 1520,
    radio_class: 4,
};

pub const PROFILE_0861: StationProfile = StationProfile {
    station_id: 0x335d,
    district: 29,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2357,
    battery_floor_mv: 12369,
    max_gate_mm: 1537,
    radio_class: 5,
};

pub const PROFILE_0862: StationProfile = StationProfile {
    station_id: 0x335e,
    district: 30,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2394,
    battery_floor_mv: 12398,
    max_gate_mm: 1554,
    radio_class: 6,
};

pub const PROFILE_0863: StationProfile = StationProfile {
    station_id: 0x335f,
    district: 31,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2431,
    battery_floor_mv: 12427,
    max_gate_mm: 1571,
    radio_class: 7,
};

pub const PROFILE_0864: StationProfile = StationProfile {
    station_id: 0x3360,
    district: 0,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2468,
    battery_floor_mv: 12456,
    max_gate_mm: 1588,
    radio_class: 0,
};

pub const PROFILE_0865: StationProfile = StationProfile {
    station_id: 0x3361,
    district: 1,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2505,
    battery_floor_mv: 12485,
    max_gate_mm: 1605,
    radio_class: 1,
};

pub const PROFILE_0866: StationProfile = StationProfile {
    station_id: 0x3362,
    district: 2,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2542,
    battery_floor_mv: 12514,
    max_gate_mm: 1622,
    radio_class: 2,
};

pub const PROFILE_0867: StationProfile = StationProfile {
    station_id: 0x3363,
    district: 3,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2579,
    battery_floor_mv: 12543,
    max_gate_mm: 1639,
    radio_class: 3,
};

pub const PROFILE_0868: StationProfile = StationProfile {
    station_id: 0x3364,
    district: 4,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2616,
    battery_floor_mv: 12572,
    max_gate_mm: 1656,
    radio_class: 4,
};

pub const PROFILE_0869: StationProfile = StationProfile {
    station_id: 0x3365,
    district: 5,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2653,
    battery_floor_mv: 11201,
    max_gate_mm: 1673,
    radio_class: 5,
};

pub const PROFILE_0870: StationProfile = StationProfile {
    station_id: 0x3366,
    district: 6,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2690,
    battery_floor_mv: 11230,
    max_gate_mm: 1690,
    radio_class: 6,
};

pub const PROFILE_0871: StationProfile = StationProfile {
    station_id: 0x3367,
    district: 7,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2727,
    battery_floor_mv: 11259,
    max_gate_mm: 1707,
    radio_class: 7,
};

pub const PROFILE_0872: StationProfile = StationProfile {
    station_id: 0x3368,
    district: 8,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2764,
    battery_floor_mv: 11288,
    max_gate_mm: 1724,
    radio_class: 0,
};

pub const PROFILE_0873: StationProfile = StationProfile {
    station_id: 0x3369,
    district: 9,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 901,
    battery_floor_mv: 11317,
    max_gate_mm: 1741,
    radio_class: 1,
};

pub const PROFILE_0874: StationProfile = StationProfile {
    station_id: 0x336a,
    district: 10,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 938,
    battery_floor_mv: 11346,
    max_gate_mm: 1758,
    radio_class: 2,
};

pub const PROFILE_0875: StationProfile = StationProfile {
    station_id: 0x336b,
    district: 11,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 975,
    battery_floor_mv: 11375,
    max_gate_mm: 1775,
    radio_class: 3,
};

pub const PROFILE_0876: StationProfile = StationProfile {
    station_id: 0x336c,
    district: 12,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1012,
    battery_floor_mv: 11404,
    max_gate_mm: 1792,
    radio_class: 4,
};

pub const PROFILE_0877: StationProfile = StationProfile {
    station_id: 0x336d,
    district: 13,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1049,
    battery_floor_mv: 11433,
    max_gate_mm: 1809,
    radio_class: 5,
};

pub const PROFILE_0878: StationProfile = StationProfile {
    station_id: 0x336e,
    district: 14,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1086,
    battery_floor_mv: 11462,
    max_gate_mm: 1826,
    radio_class: 6,
};

pub const PROFILE_0879: StationProfile = StationProfile {
    station_id: 0x336f,
    district: 15,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1123,
    battery_floor_mv: 11491,
    max_gate_mm: 1843,
    radio_class: 7,
};

pub const PROFILE_0880: StationProfile = StationProfile {
    station_id: 0x3370,
    district: 16,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1160,
    battery_floor_mv: 11520,
    max_gate_mm: 1860,
    radio_class: 0,
};

pub const PROFILE_0881: StationProfile = StationProfile {
    station_id: 0x3371,
    district: 17,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1197,
    battery_floor_mv: 11549,
    max_gate_mm: 1877,
    radio_class: 1,
};

pub const PROFILE_0882: StationProfile = StationProfile {
    station_id: 0x3372,
    district: 18,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1234,
    battery_floor_mv: 11578,
    max_gate_mm: 1894,
    radio_class: 2,
};

pub const PROFILE_0883: StationProfile = StationProfile {
    station_id: 0x3373,
    district: 19,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1271,
    battery_floor_mv: 11607,
    max_gate_mm: 1911,
    radio_class: 3,
};

pub const PROFILE_0884: StationProfile = StationProfile {
    station_id: 0x3374,
    district: 20,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1308,
    battery_floor_mv: 11636,
    max_gate_mm: 1928,
    radio_class: 4,
};

pub const PROFILE_0885: StationProfile = StationProfile {
    station_id: 0x3375,
    district: 21,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1345,
    battery_floor_mv: 11665,
    max_gate_mm: 1945,
    radio_class: 5,
};

pub const PROFILE_0886: StationProfile = StationProfile {
    station_id: 0x3376,
    district: 22,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1382,
    battery_floor_mv: 11694,
    max_gate_mm: 1962,
    radio_class: 6,
};

pub const PROFILE_0887: StationProfile = StationProfile {
    station_id: 0x3377,
    district: 23,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1419,
    battery_floor_mv: 11723,
    max_gate_mm: 1979,
    radio_class: 7,
};

pub const PROFILE_0888: StationProfile = StationProfile {
    station_id: 0x3378,
    district: 24,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1456,
    battery_floor_mv: 11752,
    max_gate_mm: 1996,
    radio_class: 0,
};

pub const PROFILE_0889: StationProfile = StationProfile {
    station_id: 0x3379,
    district: 25,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1493,
    battery_floor_mv: 11781,
    max_gate_mm: 2013,
    radio_class: 1,
};

pub const PROFILE_0890: StationProfile = StationProfile {
    station_id: 0x337a,
    district: 26,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1530,
    battery_floor_mv: 11810,
    max_gate_mm: 2030,
    radio_class: 2,
};

pub const PROFILE_0891: StationProfile = StationProfile {
    station_id: 0x337b,
    district: 27,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1567,
    battery_floor_mv: 11839,
    max_gate_mm: 2047,
    radio_class: 3,
};

pub const PROFILE_0892: StationProfile = StationProfile {
    station_id: 0x337c,
    district: 28,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1604,
    battery_floor_mv: 11868,
    max_gate_mm: 2064,
    radio_class: 4,
};

pub const PROFILE_0893: StationProfile = StationProfile {
    station_id: 0x337d,
    district: 29,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1641,
    battery_floor_mv: 11897,
    max_gate_mm: 2081,
    radio_class: 5,
};

pub const PROFILE_0894: StationProfile = StationProfile {
    station_id: 0x337e,
    district: 30,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1678,
    battery_floor_mv: 11926,
    max_gate_mm: 2098,
    radio_class: 6,
};

pub const PROFILE_0895: StationProfile = StationProfile {
    station_id: 0x337f,
    district: 31,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1715,
    battery_floor_mv: 11955,
    max_gate_mm: 2115,
    radio_class: 7,
};

pub const PROFILE_0896: StationProfile = StationProfile {
    station_id: 0x3380,
    district: 0,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1752,
    battery_floor_mv: 11984,
    max_gate_mm: 2132,
    radio_class: 0,
};

pub const PROFILE_0897: StationProfile = StationProfile {
    station_id: 0x3381,
    district: 1,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1789,
    battery_floor_mv: 12013,
    max_gate_mm: 2149,
    radio_class: 1,
};

pub const PROFILE_0898: StationProfile = StationProfile {
    station_id: 0x3382,
    district: 2,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1826,
    battery_floor_mv: 12042,
    max_gate_mm: 2166,
    radio_class: 2,
};

pub const PROFILE_0899: StationProfile = StationProfile {
    station_id: 0x3383,
    district: 3,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1863,
    battery_floor_mv: 12071,
    max_gate_mm: 2183,
    radio_class: 3,
};

pub const PROFILE_0900: StationProfile = StationProfile {
    station_id: 0x3384,
    district: 4,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1900,
    battery_floor_mv: 12100,
    max_gate_mm: 2200,
    radio_class: 4,
};

pub const PROFILE_0901: StationProfile = StationProfile {
    station_id: 0x3385,
    district: 5,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1937,
    battery_floor_mv: 12129,
    max_gate_mm: 2217,
    radio_class: 5,
};

pub const PROFILE_0902: StationProfile = StationProfile {
    station_id: 0x3386,
    district: 6,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1974,
    battery_floor_mv: 12158,
    max_gate_mm: 2234,
    radio_class: 6,
};

pub const PROFILE_0903: StationProfile = StationProfile {
    station_id: 0x3387,
    district: 7,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2011,
    battery_floor_mv: 12187,
    max_gate_mm: 2251,
    radio_class: 7,
};

pub const PROFILE_0904: StationProfile = StationProfile {
    station_id: 0x3388,
    district: 8,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2048,
    battery_floor_mv: 12216,
    max_gate_mm: 2268,
    radio_class: 0,
};

pub const PROFILE_0905: StationProfile = StationProfile {
    station_id: 0x3389,
    district: 9,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2085,
    battery_floor_mv: 12245,
    max_gate_mm: 2285,
    radio_class: 1,
};

pub const PROFILE_0906: StationProfile = StationProfile {
    station_id: 0x338a,
    district: 10,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2122,
    battery_floor_mv: 12274,
    max_gate_mm: 2302,
    radio_class: 2,
};

pub const PROFILE_0907: StationProfile = StationProfile {
    station_id: 0x338b,
    district: 11,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2159,
    battery_floor_mv: 12303,
    max_gate_mm: 2319,
    radio_class: 3,
};

pub const PROFILE_0908: StationProfile = StationProfile {
    station_id: 0x338c,
    district: 12,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2196,
    battery_floor_mv: 12332,
    max_gate_mm: 2336,
    radio_class: 4,
};

pub const PROFILE_0909: StationProfile = StationProfile {
    station_id: 0x338d,
    district: 13,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2233,
    battery_floor_mv: 12361,
    max_gate_mm: 2353,
    radio_class: 5,
};

pub const PROFILE_0910: StationProfile = StationProfile {
    station_id: 0x338e,
    district: 14,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2270,
    battery_floor_mv: 12390,
    max_gate_mm: 2370,
    radio_class: 6,
};

pub const PROFILE_0911: StationProfile = StationProfile {
    station_id: 0x338f,
    district: 15,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2307,
    battery_floor_mv: 12419,
    max_gate_mm: 2387,
    radio_class: 7,
};

pub const PROFILE_0912: StationProfile = StationProfile {
    station_id: 0x3390,
    district: 16,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2344,
    battery_floor_mv: 12448,
    max_gate_mm: 2404,
    radio_class: 0,
};

pub const PROFILE_0913: StationProfile = StationProfile {
    station_id: 0x3391,
    district: 17,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2381,
    battery_floor_mv: 12477,
    max_gate_mm: 2421,
    radio_class: 1,
};

pub const PROFILE_0914: StationProfile = StationProfile {
    station_id: 0x3392,
    district: 18,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2418,
    battery_floor_mv: 12506,
    max_gate_mm: 2438,
    radio_class: 2,
};

pub const PROFILE_0915: StationProfile = StationProfile {
    station_id: 0x3393,
    district: 19,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2455,
    battery_floor_mv: 12535,
    max_gate_mm: 2455,
    radio_class: 3,
};

pub const PROFILE_0916: StationProfile = StationProfile {
    station_id: 0x3394,
    district: 20,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2492,
    battery_floor_mv: 12564,
    max_gate_mm: 2472,
    radio_class: 4,
};

pub const PROFILE_0917: StationProfile = StationProfile {
    station_id: 0x3395,
    district: 21,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2529,
    battery_floor_mv: 12593,
    max_gate_mm: 2489,
    radio_class: 5,
};

pub const PROFILE_0918: StationProfile = StationProfile {
    station_id: 0x3396,
    district: 22,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2566,
    battery_floor_mv: 11222,
    max_gate_mm: 2506,
    radio_class: 6,
};

pub const PROFILE_0919: StationProfile = StationProfile {
    station_id: 0x3397,
    district: 23,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2603,
    battery_floor_mv: 11251,
    max_gate_mm: 2523,
    radio_class: 7,
};

pub const PROFILE_0920: StationProfile = StationProfile {
    station_id: 0x3398,
    district: 24,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2640,
    battery_floor_mv: 11280,
    max_gate_mm: 2540,
    radio_class: 0,
};

pub const PROFILE_0921: StationProfile = StationProfile {
    station_id: 0x3399,
    district: 25,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2677,
    battery_floor_mv: 11309,
    max_gate_mm: 2557,
    radio_class: 1,
};

pub const PROFILE_0922: StationProfile = StationProfile {
    station_id: 0x339a,
    district: 26,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2714,
    battery_floor_mv: 11338,
    max_gate_mm: 2574,
    radio_class: 2,
};

pub const PROFILE_0923: StationProfile = StationProfile {
    station_id: 0x339b,
    district: 27,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2751,
    battery_floor_mv: 11367,
    max_gate_mm: 2591,
    radio_class: 3,
};

pub const PROFILE_0924: StationProfile = StationProfile {
    station_id: 0x339c,
    district: 28,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2788,
    battery_floor_mv: 11396,
    max_gate_mm: 2608,
    radio_class: 4,
};

pub const PROFILE_0925: StationProfile = StationProfile {
    station_id: 0x339d,
    district: 29,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 925,
    battery_floor_mv: 11425,
    max_gate_mm: 2625,
    radio_class: 5,
};

pub const PROFILE_0926: StationProfile = StationProfile {
    station_id: 0x339e,
    district: 30,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 962,
    battery_floor_mv: 11454,
    max_gate_mm: 2642,
    radio_class: 6,
};

pub const PROFILE_0927: StationProfile = StationProfile {
    station_id: 0x339f,
    district: 31,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 999,
    battery_floor_mv: 11483,
    max_gate_mm: 2659,
    radio_class: 7,
};

pub const PROFILE_0928: StationProfile = StationProfile {
    station_id: 0x33a0,
    district: 0,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1036,
    battery_floor_mv: 11512,
    max_gate_mm: 2676,
    radio_class: 0,
};

pub const PROFILE_0929: StationProfile = StationProfile {
    station_id: 0x33a1,
    district: 1,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1073,
    battery_floor_mv: 11541,
    max_gate_mm: 2693,
    radio_class: 1,
};

pub const PROFILE_0930: StationProfile = StationProfile {
    station_id: 0x33a2,
    district: 2,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1110,
    battery_floor_mv: 11570,
    max_gate_mm: 2710,
    radio_class: 2,
};

pub const PROFILE_0931: StationProfile = StationProfile {
    station_id: 0x33a3,
    district: 3,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1147,
    battery_floor_mv: 11599,
    max_gate_mm: 2727,
    radio_class: 3,
};

pub const PROFILE_0932: StationProfile = StationProfile {
    station_id: 0x33a4,
    district: 4,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1184,
    battery_floor_mv: 11628,
    max_gate_mm: 2744,
    radio_class: 4,
};

pub const PROFILE_0933: StationProfile = StationProfile {
    station_id: 0x33a5,
    district: 5,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1221,
    battery_floor_mv: 11657,
    max_gate_mm: 2761,
    radio_class: 5,
};

pub const PROFILE_0934: StationProfile = StationProfile {
    station_id: 0x33a6,
    district: 6,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1258,
    battery_floor_mv: 11686,
    max_gate_mm: 2778,
    radio_class: 6,
};

pub const PROFILE_0935: StationProfile = StationProfile {
    station_id: 0x33a7,
    district: 7,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1295,
    battery_floor_mv: 11715,
    max_gate_mm: 2795,
    radio_class: 7,
};

pub const PROFILE_0936: StationProfile = StationProfile {
    station_id: 0x33a8,
    district: 8,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1332,
    battery_floor_mv: 11744,
    max_gate_mm: 2812,
    radio_class: 0,
};

pub const PROFILE_0937: StationProfile = StationProfile {
    station_id: 0x33a9,
    district: 9,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1369,
    battery_floor_mv: 11773,
    max_gate_mm: 2829,
    radio_class: 1,
};

pub const PROFILE_0938: StationProfile = StationProfile {
    station_id: 0x33aa,
    district: 10,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1406,
    battery_floor_mv: 11802,
    max_gate_mm: 2846,
    radio_class: 2,
};

pub const PROFILE_0939: StationProfile = StationProfile {
    station_id: 0x33ab,
    district: 11,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1443,
    battery_floor_mv: 11831,
    max_gate_mm: 2863,
    radio_class: 3,
};

pub const PROFILE_0940: StationProfile = StationProfile {
    station_id: 0x33ac,
    district: 12,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1480,
    battery_floor_mv: 11860,
    max_gate_mm: 2880,
    radio_class: 4,
};

pub const PROFILE_0941: StationProfile = StationProfile {
    station_id: 0x33ad,
    district: 13,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1517,
    battery_floor_mv: 11889,
    max_gate_mm: 2897,
    radio_class: 5,
};

pub const PROFILE_0942: StationProfile = StationProfile {
    station_id: 0x33ae,
    district: 14,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1554,
    battery_floor_mv: 11918,
    max_gate_mm: 2914,
    radio_class: 6,
};

pub const PROFILE_0943: StationProfile = StationProfile {
    station_id: 0x33af,
    district: 15,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1591,
    battery_floor_mv: 11947,
    max_gate_mm: 2931,
    radio_class: 7,
};

pub const PROFILE_0944: StationProfile = StationProfile {
    station_id: 0x33b0,
    district: 16,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1628,
    battery_floor_mv: 11976,
    max_gate_mm: 2948,
    radio_class: 0,
};

pub const PROFILE_0945: StationProfile = StationProfile {
    station_id: 0x33b1,
    district: 17,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1665,
    battery_floor_mv: 12005,
    max_gate_mm: 2965,
    radio_class: 1,
};

pub const PROFILE_0946: StationProfile = StationProfile {
    station_id: 0x33b2,
    district: 18,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1702,
    battery_floor_mv: 12034,
    max_gate_mm: 2982,
    radio_class: 2,
};

pub const PROFILE_0947: StationProfile = StationProfile {
    station_id: 0x33b3,
    district: 19,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1739,
    battery_floor_mv: 12063,
    max_gate_mm: 2999,
    radio_class: 3,
};

pub const PROFILE_0948: StationProfile = StationProfile {
    station_id: 0x33b4,
    district: 20,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1776,
    battery_floor_mv: 12092,
    max_gate_mm: 716,
    radio_class: 4,
};

pub const PROFILE_0949: StationProfile = StationProfile {
    station_id: 0x33b5,
    district: 21,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1813,
    battery_floor_mv: 12121,
    max_gate_mm: 733,
    radio_class: 5,
};

pub const PROFILE_0950: StationProfile = StationProfile {
    station_id: 0x33b6,
    district: 22,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1850,
    battery_floor_mv: 12150,
    max_gate_mm: 750,
    radio_class: 6,
};

pub const PROFILE_0951: StationProfile = StationProfile {
    station_id: 0x33b7,
    district: 23,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1887,
    battery_floor_mv: 12179,
    max_gate_mm: 767,
    radio_class: 7,
};

pub const PROFILE_0952: StationProfile = StationProfile {
    station_id: 0x33b8,
    district: 24,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1924,
    battery_floor_mv: 12208,
    max_gate_mm: 784,
    radio_class: 0,
};

pub const PROFILE_0953: StationProfile = StationProfile {
    station_id: 0x33b9,
    district: 25,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1961,
    battery_floor_mv: 12237,
    max_gate_mm: 801,
    radio_class: 1,
};

pub const PROFILE_0954: StationProfile = StationProfile {
    station_id: 0x33ba,
    district: 26,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1998,
    battery_floor_mv: 12266,
    max_gate_mm: 818,
    radio_class: 2,
};

pub const PROFILE_0955: StationProfile = StationProfile {
    station_id: 0x33bb,
    district: 27,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2035,
    battery_floor_mv: 12295,
    max_gate_mm: 835,
    radio_class: 3,
};

pub const PROFILE_0956: StationProfile = StationProfile {
    station_id: 0x33bc,
    district: 28,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2072,
    battery_floor_mv: 12324,
    max_gate_mm: 852,
    radio_class: 4,
};

pub const PROFILE_0957: StationProfile = StationProfile {
    station_id: 0x33bd,
    district: 29,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2109,
    battery_floor_mv: 12353,
    max_gate_mm: 869,
    radio_class: 5,
};

pub const PROFILE_0958: StationProfile = StationProfile {
    station_id: 0x33be,
    district: 30,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2146,
    battery_floor_mv: 12382,
    max_gate_mm: 886,
    radio_class: 6,
};

pub const PROFILE_0959: StationProfile = StationProfile {
    station_id: 0x33bf,
    district: 31,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2183,
    battery_floor_mv: 12411,
    max_gate_mm: 903,
    radio_class: 7,
};

pub const PROFILE_0960: StationProfile = StationProfile {
    station_id: 0x33c0,
    district: 0,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2220,
    battery_floor_mv: 12440,
    max_gate_mm: 920,
    radio_class: 0,
};

pub const PROFILE_0961: StationProfile = StationProfile {
    station_id: 0x33c1,
    district: 1,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2257,
    battery_floor_mv: 12469,
    max_gate_mm: 937,
    radio_class: 1,
};

pub const PROFILE_0962: StationProfile = StationProfile {
    station_id: 0x33c2,
    district: 2,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2294,
    battery_floor_mv: 12498,
    max_gate_mm: 954,
    radio_class: 2,
};

pub const PROFILE_0963: StationProfile = StationProfile {
    station_id: 0x33c3,
    district: 3,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2331,
    battery_floor_mv: 12527,
    max_gate_mm: 971,
    radio_class: 3,
};

pub const PROFILE_0964: StationProfile = StationProfile {
    station_id: 0x33c4,
    district: 4,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2368,
    battery_floor_mv: 12556,
    max_gate_mm: 988,
    radio_class: 4,
};

pub const PROFILE_0965: StationProfile = StationProfile {
    station_id: 0x33c5,
    district: 5,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2405,
    battery_floor_mv: 12585,
    max_gate_mm: 1005,
    radio_class: 5,
};

pub const PROFILE_0966: StationProfile = StationProfile {
    station_id: 0x33c6,
    district: 6,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2442,
    battery_floor_mv: 11214,
    max_gate_mm: 1022,
    radio_class: 6,
};

pub const PROFILE_0967: StationProfile = StationProfile {
    station_id: 0x33c7,
    district: 7,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2479,
    battery_floor_mv: 11243,
    max_gate_mm: 1039,
    radio_class: 7,
};

pub const PROFILE_0968: StationProfile = StationProfile {
    station_id: 0x33c8,
    district: 8,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2516,
    battery_floor_mv: 11272,
    max_gate_mm: 1056,
    radio_class: 0,
};

pub const PROFILE_0969: StationProfile = StationProfile {
    station_id: 0x33c9,
    district: 9,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2553,
    battery_floor_mv: 11301,
    max_gate_mm: 1073,
    radio_class: 1,
};

pub const PROFILE_0970: StationProfile = StationProfile {
    station_id: 0x33ca,
    district: 10,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2590,
    battery_floor_mv: 11330,
    max_gate_mm: 1090,
    radio_class: 2,
};

pub const PROFILE_0971: StationProfile = StationProfile {
    station_id: 0x33cb,
    district: 11,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2627,
    battery_floor_mv: 11359,
    max_gate_mm: 1107,
    radio_class: 3,
};

pub const PROFILE_0972: StationProfile = StationProfile {
    station_id: 0x33cc,
    district: 12,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2664,
    battery_floor_mv: 11388,
    max_gate_mm: 1124,
    radio_class: 4,
};

pub const PROFILE_0973: StationProfile = StationProfile {
    station_id: 0x33cd,
    district: 13,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2701,
    battery_floor_mv: 11417,
    max_gate_mm: 1141,
    radio_class: 5,
};

pub const PROFILE_0974: StationProfile = StationProfile {
    station_id: 0x33ce,
    district: 14,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2738,
    battery_floor_mv: 11446,
    max_gate_mm: 1158,
    radio_class: 6,
};

pub const PROFILE_0975: StationProfile = StationProfile {
    station_id: 0x33cf,
    district: 15,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2775,
    battery_floor_mv: 11475,
    max_gate_mm: 1175,
    radio_class: 7,
};

pub const PROFILE_0976: StationProfile = StationProfile {
    station_id: 0x33d0,
    district: 16,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 912,
    battery_floor_mv: 11504,
    max_gate_mm: 1192,
    radio_class: 0,
};

pub const PROFILE_0977: StationProfile = StationProfile {
    station_id: 0x33d1,
    district: 17,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 949,
    battery_floor_mv: 11533,
    max_gate_mm: 1209,
    radio_class: 1,
};

pub const PROFILE_0978: StationProfile = StationProfile {
    station_id: 0x33d2,
    district: 18,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 986,
    battery_floor_mv: 11562,
    max_gate_mm: 1226,
    radio_class: 2,
};

pub const PROFILE_0979: StationProfile = StationProfile {
    station_id: 0x33d3,
    district: 19,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1023,
    battery_floor_mv: 11591,
    max_gate_mm: 1243,
    radio_class: 3,
};

pub const PROFILE_0980: StationProfile = StationProfile {
    station_id: 0x33d4,
    district: 20,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1060,
    battery_floor_mv: 11620,
    max_gate_mm: 1260,
    radio_class: 4,
};

pub const PROFILE_0981: StationProfile = StationProfile {
    station_id: 0x33d5,
    district: 21,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1097,
    battery_floor_mv: 11649,
    max_gate_mm: 1277,
    radio_class: 5,
};

pub const PROFILE_0982: StationProfile = StationProfile {
    station_id: 0x33d6,
    district: 22,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1134,
    battery_floor_mv: 11678,
    max_gate_mm: 1294,
    radio_class: 6,
};

pub const PROFILE_0983: StationProfile = StationProfile {
    station_id: 0x33d7,
    district: 23,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1171,
    battery_floor_mv: 11707,
    max_gate_mm: 1311,
    radio_class: 7,
};

pub const PROFILE_0984: StationProfile = StationProfile {
    station_id: 0x33d8,
    district: 24,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1208,
    battery_floor_mv: 11736,
    max_gate_mm: 1328,
    radio_class: 0,
};

pub const PROFILE_0985: StationProfile = StationProfile {
    station_id: 0x33d9,
    district: 25,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1245,
    battery_floor_mv: 11765,
    max_gate_mm: 1345,
    radio_class: 1,
};

pub const PROFILE_0986: StationProfile = StationProfile {
    station_id: 0x33da,
    district: 26,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1282,
    battery_floor_mv: 11794,
    max_gate_mm: 1362,
    radio_class: 2,
};

pub const PROFILE_0987: StationProfile = StationProfile {
    station_id: 0x33db,
    district: 27,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1319,
    battery_floor_mv: 11823,
    max_gate_mm: 1379,
    radio_class: 3,
};

pub const PROFILE_0988: StationProfile = StationProfile {
    station_id: 0x33dc,
    district: 28,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1356,
    battery_floor_mv: 11852,
    max_gate_mm: 1396,
    radio_class: 4,
};

pub const PROFILE_0989: StationProfile = StationProfile {
    station_id: 0x33dd,
    district: 29,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1393,
    battery_floor_mv: 11881,
    max_gate_mm: 1413,
    radio_class: 5,
};

pub const PROFILE_0990: StationProfile = StationProfile {
    station_id: 0x33de,
    district: 30,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1430,
    battery_floor_mv: 11910,
    max_gate_mm: 1430,
    radio_class: 6,
};

pub const PROFILE_0991: StationProfile = StationProfile {
    station_id: 0x33df,
    district: 31,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1467,
    battery_floor_mv: 11939,
    max_gate_mm: 1447,
    radio_class: 7,
};

pub const PROFILE_0992: StationProfile = StationProfile {
    station_id: 0x33e0,
    district: 0,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1504,
    battery_floor_mv: 11968,
    max_gate_mm: 1464,
    radio_class: 0,
};

pub const PROFILE_0993: StationProfile = StationProfile {
    station_id: 0x33e1,
    district: 1,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1541,
    battery_floor_mv: 11997,
    max_gate_mm: 1481,
    radio_class: 1,
};

pub const PROFILE_0994: StationProfile = StationProfile {
    station_id: 0x33e2,
    district: 2,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1578,
    battery_floor_mv: 12026,
    max_gate_mm: 1498,
    radio_class: 2,
};

pub const PROFILE_0995: StationProfile = StationProfile {
    station_id: 0x33e3,
    district: 3,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1615,
    battery_floor_mv: 12055,
    max_gate_mm: 1515,
    radio_class: 3,
};

pub const PROFILE_0996: StationProfile = StationProfile {
    station_id: 0x33e4,
    district: 4,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1652,
    battery_floor_mv: 12084,
    max_gate_mm: 1532,
    radio_class: 4,
};

pub const PROFILE_0997: StationProfile = StationProfile {
    station_id: 0x33e5,
    district: 5,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1689,
    battery_floor_mv: 12113,
    max_gate_mm: 1549,
    radio_class: 5,
};

pub const PROFILE_0998: StationProfile = StationProfile {
    station_id: 0x33e6,
    district: 6,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1726,
    battery_floor_mv: 12142,
    max_gate_mm: 1566,
    radio_class: 6,
};

pub const PROFILE_0999: StationProfile = StationProfile {
    station_id: 0x33e7,
    district: 7,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1763,
    battery_floor_mv: 12171,
    max_gate_mm: 1583,
    radio_class: 7,
};

pub const PROFILE_1000: StationProfile = StationProfile {
    station_id: 0x33e8,
    district: 8,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1800,
    battery_floor_mv: 12200,
    max_gate_mm: 1600,
    radio_class: 0,
};

pub const PROFILE_1001: StationProfile = StationProfile {
    station_id: 0x33e9,
    district: 9,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1837,
    battery_floor_mv: 12229,
    max_gate_mm: 1617,
    radio_class: 1,
};

pub const PROFILE_1002: StationProfile = StationProfile {
    station_id: 0x33ea,
    district: 10,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1874,
    battery_floor_mv: 12258,
    max_gate_mm: 1634,
    radio_class: 2,
};

pub const PROFILE_1003: StationProfile = StationProfile {
    station_id: 0x33eb,
    district: 11,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1911,
    battery_floor_mv: 12287,
    max_gate_mm: 1651,
    radio_class: 3,
};

pub const PROFILE_1004: StationProfile = StationProfile {
    station_id: 0x33ec,
    district: 12,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1948,
    battery_floor_mv: 12316,
    max_gate_mm: 1668,
    radio_class: 4,
};

pub const PROFILE_1005: StationProfile = StationProfile {
    station_id: 0x33ed,
    district: 13,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1985,
    battery_floor_mv: 12345,
    max_gate_mm: 1685,
    radio_class: 5,
};

pub const PROFILE_1006: StationProfile = StationProfile {
    station_id: 0x33ee,
    district: 14,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2022,
    battery_floor_mv: 12374,
    max_gate_mm: 1702,
    radio_class: 6,
};

pub const PROFILE_1007: StationProfile = StationProfile {
    station_id: 0x33ef,
    district: 15,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2059,
    battery_floor_mv: 12403,
    max_gate_mm: 1719,
    radio_class: 7,
};

pub const PROFILE_1008: StationProfile = StationProfile {
    station_id: 0x33f0,
    district: 16,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2096,
    battery_floor_mv: 12432,
    max_gate_mm: 1736,
    radio_class: 0,
};

pub const PROFILE_1009: StationProfile = StationProfile {
    station_id: 0x33f1,
    district: 17,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2133,
    battery_floor_mv: 12461,
    max_gate_mm: 1753,
    radio_class: 1,
};

pub const PROFILE_1010: StationProfile = StationProfile {
    station_id: 0x33f2,
    district: 18,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2170,
    battery_floor_mv: 12490,
    max_gate_mm: 1770,
    radio_class: 2,
};

pub const PROFILE_1011: StationProfile = StationProfile {
    station_id: 0x33f3,
    district: 19,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2207,
    battery_floor_mv: 12519,
    max_gate_mm: 1787,
    radio_class: 3,
};

pub const PROFILE_1012: StationProfile = StationProfile {
    station_id: 0x33f4,
    district: 20,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2244,
    battery_floor_mv: 12548,
    max_gate_mm: 1804,
    radio_class: 4,
};

pub const PROFILE_1013: StationProfile = StationProfile {
    station_id: 0x33f5,
    district: 21,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2281,
    battery_floor_mv: 12577,
    max_gate_mm: 1821,
    radio_class: 5,
};

pub const PROFILE_1014: StationProfile = StationProfile {
    station_id: 0x33f6,
    district: 22,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2318,
    battery_floor_mv: 11206,
    max_gate_mm: 1838,
    radio_class: 6,
};

pub const PROFILE_1015: StationProfile = StationProfile {
    station_id: 0x33f7,
    district: 23,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2355,
    battery_floor_mv: 11235,
    max_gate_mm: 1855,
    radio_class: 7,
};

pub const PROFILE_1016: StationProfile = StationProfile {
    station_id: 0x33f8,
    district: 24,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2392,
    battery_floor_mv: 11264,
    max_gate_mm: 1872,
    radio_class: 0,
};

pub const PROFILE_1017: StationProfile = StationProfile {
    station_id: 0x33f9,
    district: 25,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2429,
    battery_floor_mv: 11293,
    max_gate_mm: 1889,
    radio_class: 1,
};

pub const PROFILE_1018: StationProfile = StationProfile {
    station_id: 0x33fa,
    district: 26,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2466,
    battery_floor_mv: 11322,
    max_gate_mm: 1906,
    radio_class: 2,
};

pub const PROFILE_1019: StationProfile = StationProfile {
    station_id: 0x33fb,
    district: 27,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2503,
    battery_floor_mv: 11351,
    max_gate_mm: 1923,
    radio_class: 3,
};

pub const PROFILE_1020: StationProfile = StationProfile {
    station_id: 0x33fc,
    district: 28,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2540,
    battery_floor_mv: 11380,
    max_gate_mm: 1940,
    radio_class: 4,
};

pub const PROFILE_1021: StationProfile = StationProfile {
    station_id: 0x33fd,
    district: 29,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2577,
    battery_floor_mv: 11409,
    max_gate_mm: 1957,
    radio_class: 5,
};

pub const PROFILE_1022: StationProfile = StationProfile {
    station_id: 0x33fe,
    district: 30,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 2614,
    battery_floor_mv: 11438,
    max_gate_mm: 1974,
    radio_class: 6,
};

pub const PROFILE_1023: StationProfile = StationProfile {
    station_id: 0x33ff,
    district: 31,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 2651,
    battery_floor_mv: 11467,
    max_gate_mm: 1991,
    radio_class: 7,
};

pub const PROFILE_1024: StationProfile = StationProfile {
    station_id: 0x3400,
    district: 0,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 2688,
    battery_floor_mv: 11496,
    max_gate_mm: 2008,
    radio_class: 0,
};

pub const PROFILE_1025: StationProfile = StationProfile {
    station_id: 0x3401,
    district: 1,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 2725,
    battery_floor_mv: 11525,
    max_gate_mm: 2025,
    radio_class: 1,
};

pub const PROFILE_1026: StationProfile = StationProfile {
    station_id: 0x3402,
    district: 2,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 2762,
    battery_floor_mv: 11554,
    max_gate_mm: 2042,
    radio_class: 2,
};

pub const PROFILE_1027: StationProfile = StationProfile {
    station_id: 0x3403,
    district: 3,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 2799,
    battery_floor_mv: 11583,
    max_gate_mm: 2059,
    radio_class: 3,
};

pub const PROFILE_1028: StationProfile = StationProfile {
    station_id: 0x3404,
    district: 4,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 936,
    battery_floor_mv: 11612,
    max_gate_mm: 2076,
    radio_class: 4,
};

pub const PROFILE_1029: StationProfile = StationProfile {
    station_id: 0x3405,
    district: 5,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 973,
    battery_floor_mv: 11641,
    max_gate_mm: 2093,
    radio_class: 5,
};

pub const PROFILE_1030: StationProfile = StationProfile {
    station_id: 0x3406,
    district: 6,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1010,
    battery_floor_mv: 11670,
    max_gate_mm: 2110,
    radio_class: 6,
};

pub const PROFILE_1031: StationProfile = StationProfile {
    station_id: 0x3407,
    district: 7,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1047,
    battery_floor_mv: 11699,
    max_gate_mm: 2127,
    radio_class: 7,
};

pub const PROFILE_1032: StationProfile = StationProfile {
    station_id: 0x3408,
    district: 8,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1084,
    battery_floor_mv: 11728,
    max_gate_mm: 2144,
    radio_class: 0,
};

pub const PROFILE_1033: StationProfile = StationProfile {
    station_id: 0x3409,
    district: 9,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1121,
    battery_floor_mv: 11757,
    max_gate_mm: 2161,
    radio_class: 1,
};

pub const PROFILE_1034: StationProfile = StationProfile {
    station_id: 0x340a,
    district: 10,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1158,
    battery_floor_mv: 11786,
    max_gate_mm: 2178,
    radio_class: 2,
};

pub const PROFILE_1035: StationProfile = StationProfile {
    station_id: 0x340b,
    district: 11,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1195,
    battery_floor_mv: 11815,
    max_gate_mm: 2195,
    radio_class: 3,
};

pub const PROFILE_1036: StationProfile = StationProfile {
    station_id: 0x340c,
    district: 12,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1232,
    battery_floor_mv: 11844,
    max_gate_mm: 2212,
    radio_class: 4,
};

pub const PROFILE_1037: StationProfile = StationProfile {
    station_id: 0x340d,
    district: 13,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1269,
    battery_floor_mv: 11873,
    max_gate_mm: 2229,
    radio_class: 5,
};

pub const PROFILE_1038: StationProfile = StationProfile {
    station_id: 0x340e,
    district: 14,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1306,
    battery_floor_mv: 11902,
    max_gate_mm: 2246,
    radio_class: 6,
};

pub const PROFILE_1039: StationProfile = StationProfile {
    station_id: 0x340f,
    district: 15,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1343,
    battery_floor_mv: 11931,
    max_gate_mm: 2263,
    radio_class: 7,
};

pub const PROFILE_1040: StationProfile = StationProfile {
    station_id: 0x3410,
    district: 16,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1380,
    battery_floor_mv: 11960,
    max_gate_mm: 2280,
    radio_class: 0,
};

pub const PROFILE_1041: StationProfile = StationProfile {
    station_id: 0x3411,
    district: 17,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1417,
    battery_floor_mv: 11989,
    max_gate_mm: 2297,
    radio_class: 1,
};

pub const PROFILE_1042: StationProfile = StationProfile {
    station_id: 0x3412,
    district: 18,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1454,
    battery_floor_mv: 12018,
    max_gate_mm: 2314,
    radio_class: 2,
};

pub const PROFILE_1043: StationProfile = StationProfile {
    station_id: 0x3413,
    district: 19,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1491,
    battery_floor_mv: 12047,
    max_gate_mm: 2331,
    radio_class: 3,
};

pub const PROFILE_1044: StationProfile = StationProfile {
    station_id: 0x3414,
    district: 20,
    basin: "north-main",
    pump_family: "axial-18",
    flood_floor_mm: 1528,
    battery_floor_mv: 12076,
    max_gate_mm: 2348,
    radio_class: 4,
};

pub const PROFILE_1045: StationProfile = StationProfile {
    station_id: 0x3415,
    district: 21,
    basin: "east-ditch",
    pump_family: "drywell-22",
    flood_floor_mm: 1565,
    battery_floor_mv: 12105,
    max_gate_mm: 2365,
    radio_class: 5,
};

pub const PROFILE_1046: StationProfile = StationProfile {
    station_id: 0x3416,
    district: 22,
    basin: "harbor-low",
    pump_family: "screw-09",
    flood_floor_mm: 1602,
    battery_floor_mv: 12134,
    max_gate_mm: 2382,
    radio_class: 6,
};

pub const PROFILE_1047: StationProfile = StationProfile {
    station_id: 0x3417,
    district: 23,
    basin: "airport-west",
    pump_family: "trash-guard",
    flood_floor_mm: 1639,
    battery_floor_mv: 12163,
    max_gate_mm: 2399,
    radio_class: 7,
};

pub const PROFILE_1048: StationProfile = StationProfile {
    station_id: 0x3418,
    district: 24,
    basin: "canal-ring",
    pump_family: "lift-44",
    flood_floor_mm: 1676,
    battery_floor_mv: 12192,
    max_gate_mm: 2416,
    radio_class: 0,
};

pub const PROFILE_1049: StationProfile = StationProfile {
    station_id: 0x3419,
    district: 25,
    basin: "ridge-run",
    pump_family: "gatehouse",
    flood_floor_mm: 1713,
    battery_floor_mv: 12221,
    max_gate_mm: 2433,
    radio_class: 1,
};

pub const STATION_PROFILES: &[StationProfile] = &[
    PROFILE_0000,
    PROFILE_0001,
    PROFILE_0002,
    PROFILE_0003,
    PROFILE_0004,
    PROFILE_0005,
    PROFILE_0006,
    PROFILE_0007,
    PROFILE_0008,
    PROFILE_0009,
    PROFILE_0010,
    PROFILE_0011,
    PROFILE_0012,
    PROFILE_0013,
    PROFILE_0014,
    PROFILE_0015,
    PROFILE_0016,
    PROFILE_0017,
    PROFILE_0018,
    PROFILE_0019,
    PROFILE_0020,
    PROFILE_0021,
    PROFILE_0022,
    PROFILE_0023,
    PROFILE_0024,
    PROFILE_0025,
    PROFILE_0026,
    PROFILE_0027,
    PROFILE_0028,
    PROFILE_0029,
    PROFILE_0030,
    PROFILE_0031,
    PROFILE_0032,
    PROFILE_0033,
    PROFILE_0034,
    PROFILE_0035,
    PROFILE_0036,
    PROFILE_0037,
    PROFILE_0038,
    PROFILE_0039,
    PROFILE_0040,
    PROFILE_0041,
    PROFILE_0042,
    PROFILE_0043,
    PROFILE_0044,
    PROFILE_0045,
    PROFILE_0046,
    PROFILE_0047,
    PROFILE_0048,
    PROFILE_0049,
    PROFILE_0050,
    PROFILE_0051,
    PROFILE_0052,
    PROFILE_0053,
    PROFILE_0054,
    PROFILE_0055,
    PROFILE_0056,
    PROFILE_0057,
    PROFILE_0058,
    PROFILE_0059,
    PROFILE_0060,
    PROFILE_0061,
    PROFILE_0062,
    PROFILE_0063,
    PROFILE_0064,
    PROFILE_0065,
    PROFILE_0066,
    PROFILE_0067,
    PROFILE_0068,
    PROFILE_0069,
    PROFILE_0070,
    PROFILE_0071,
    PROFILE_0072,
    PROFILE_0073,
    PROFILE_0074,
    PROFILE_0075,
    PROFILE_0076,
    PROFILE_0077,
    PROFILE_0078,
    PROFILE_0079,
    PROFILE_0080,
    PROFILE_0081,
    PROFILE_0082,
    PROFILE_0083,
    PROFILE_0084,
    PROFILE_0085,
    PROFILE_0086,
    PROFILE_0087,
    PROFILE_0088,
    PROFILE_0089,
    PROFILE_0090,
    PROFILE_0091,
    PROFILE_0092,
    PROFILE_0093,
    PROFILE_0094,
    PROFILE_0095,
    PROFILE_0096,
    PROFILE_0097,
    PROFILE_0098,
    PROFILE_0099,
    PROFILE_0100,
    PROFILE_0101,
    PROFILE_0102,
    PROFILE_0103,
    PROFILE_0104,
    PROFILE_0105,
    PROFILE_0106,
    PROFILE_0107,
    PROFILE_0108,
    PROFILE_0109,
    PROFILE_0110,
    PROFILE_0111,
    PROFILE_0112,
    PROFILE_0113,
    PROFILE_0114,
    PROFILE_0115,
    PROFILE_0116,
    PROFILE_0117,
    PROFILE_0118,
    PROFILE_0119,
    PROFILE_0120,
    PROFILE_0121,
    PROFILE_0122,
    PROFILE_0123,
    PROFILE_0124,
    PROFILE_0125,
    PROFILE_0126,
    PROFILE_0127,
    PROFILE_0128,
    PROFILE_0129,
    PROFILE_0130,
    PROFILE_0131,
    PROFILE_0132,
    PROFILE_0133,
    PROFILE_0134,
    PROFILE_0135,
    PROFILE_0136,
    PROFILE_0137,
    PROFILE_0138,
    PROFILE_0139,
    PROFILE_0140,
    PROFILE_0141,
    PROFILE_0142,
    PROFILE_0143,
    PROFILE_0144,
    PROFILE_0145,
    PROFILE_0146,
    PROFILE_0147,
    PROFILE_0148,
    PROFILE_0149,
    PROFILE_0150,
    PROFILE_0151,
    PROFILE_0152,
    PROFILE_0153,
    PROFILE_0154,
    PROFILE_0155,
    PROFILE_0156,
    PROFILE_0157,
    PROFILE_0158,
    PROFILE_0159,
    PROFILE_0160,
    PROFILE_0161,
    PROFILE_0162,
    PROFILE_0163,
    PROFILE_0164,
    PROFILE_0165,
    PROFILE_0166,
    PROFILE_0167,
    PROFILE_0168,
    PROFILE_0169,
    PROFILE_0170,
    PROFILE_0171,
    PROFILE_0172,
    PROFILE_0173,
    PROFILE_0174,
    PROFILE_0175,
    PROFILE_0176,
    PROFILE_0177,
    PROFILE_0178,
    PROFILE_0179,
    PROFILE_0180,
    PROFILE_0181,
    PROFILE_0182,
    PROFILE_0183,
    PROFILE_0184,
    PROFILE_0185,
    PROFILE_0186,
    PROFILE_0187,
    PROFILE_0188,
    PROFILE_0189,
    PROFILE_0190,
    PROFILE_0191,
    PROFILE_0192,
    PROFILE_0193,
    PROFILE_0194,
    PROFILE_0195,
    PROFILE_0196,
    PROFILE_0197,
    PROFILE_0198,
    PROFILE_0199,
    PROFILE_0200,
    PROFILE_0201,
    PROFILE_0202,
    PROFILE_0203,
    PROFILE_0204,
    PROFILE_0205,
    PROFILE_0206,
    PROFILE_0207,
    PROFILE_0208,
    PROFILE_0209,
    PROFILE_0210,
    PROFILE_0211,
    PROFILE_0212,
    PROFILE_0213,
    PROFILE_0214,
    PROFILE_0215,
    PROFILE_0216,
    PROFILE_0217,
    PROFILE_0218,
    PROFILE_0219,
    PROFILE_0220,
    PROFILE_0221,
    PROFILE_0222,
    PROFILE_0223,
    PROFILE_0224,
    PROFILE_0225,
    PROFILE_0226,
    PROFILE_0227,
    PROFILE_0228,
    PROFILE_0229,
    PROFILE_0230,
    PROFILE_0231,
    PROFILE_0232,
    PROFILE_0233,
    PROFILE_0234,
    PROFILE_0235,
    PROFILE_0236,
    PROFILE_0237,
    PROFILE_0238,
    PROFILE_0239,
    PROFILE_0240,
    PROFILE_0241,
    PROFILE_0242,
    PROFILE_0243,
    PROFILE_0244,
    PROFILE_0245,
    PROFILE_0246,
    PROFILE_0247,
    PROFILE_0248,
    PROFILE_0249,
    PROFILE_0250,
    PROFILE_0251,
    PROFILE_0252,
    PROFILE_0253,
    PROFILE_0254,
    PROFILE_0255,
    PROFILE_0256,
    PROFILE_0257,
    PROFILE_0258,
    PROFILE_0259,
    PROFILE_0260,
    PROFILE_0261,
    PROFILE_0262,
    PROFILE_0263,
    PROFILE_0264,
    PROFILE_0265,
    PROFILE_0266,
    PROFILE_0267,
    PROFILE_0268,
    PROFILE_0269,
    PROFILE_0270,
    PROFILE_0271,
    PROFILE_0272,
    PROFILE_0273,
    PROFILE_0274,
    PROFILE_0275,
    PROFILE_0276,
    PROFILE_0277,
    PROFILE_0278,
    PROFILE_0279,
    PROFILE_0280,
    PROFILE_0281,
    PROFILE_0282,
    PROFILE_0283,
    PROFILE_0284,
    PROFILE_0285,
    PROFILE_0286,
    PROFILE_0287,
    PROFILE_0288,
    PROFILE_0289,
    PROFILE_0290,
    PROFILE_0291,
    PROFILE_0292,
    PROFILE_0293,
    PROFILE_0294,
    PROFILE_0295,
    PROFILE_0296,
    PROFILE_0297,
    PROFILE_0298,
    PROFILE_0299,
    PROFILE_0300,
    PROFILE_0301,
    PROFILE_0302,
    PROFILE_0303,
    PROFILE_0304,
    PROFILE_0305,
    PROFILE_0306,
    PROFILE_0307,
    PROFILE_0308,
    PROFILE_0309,
    PROFILE_0310,
    PROFILE_0311,
    PROFILE_0312,
    PROFILE_0313,
    PROFILE_0314,
    PROFILE_0315,
    PROFILE_0316,
    PROFILE_0317,
    PROFILE_0318,
    PROFILE_0319,
    PROFILE_0320,
    PROFILE_0321,
    PROFILE_0322,
    PROFILE_0323,
    PROFILE_0324,
    PROFILE_0325,
    PROFILE_0326,
    PROFILE_0327,
    PROFILE_0328,
    PROFILE_0329,
    PROFILE_0330,
    PROFILE_0331,
    PROFILE_0332,
    PROFILE_0333,
    PROFILE_0334,
    PROFILE_0335,
    PROFILE_0336,
    PROFILE_0337,
    PROFILE_0338,
    PROFILE_0339,
    PROFILE_0340,
    PROFILE_0341,
    PROFILE_0342,
    PROFILE_0343,
    PROFILE_0344,
    PROFILE_0345,
    PROFILE_0346,
    PROFILE_0347,
    PROFILE_0348,
    PROFILE_0349,
    PROFILE_0350,
    PROFILE_0351,
    PROFILE_0352,
    PROFILE_0353,
    PROFILE_0354,
    PROFILE_0355,
    PROFILE_0356,
    PROFILE_0357,
    PROFILE_0358,
    PROFILE_0359,
    PROFILE_0360,
    PROFILE_0361,
    PROFILE_0362,
    PROFILE_0363,
    PROFILE_0364,
    PROFILE_0365,
    PROFILE_0366,
    PROFILE_0367,
    PROFILE_0368,
    PROFILE_0369,
    PROFILE_0370,
    PROFILE_0371,
    PROFILE_0372,
    PROFILE_0373,
    PROFILE_0374,
    PROFILE_0375,
    PROFILE_0376,
    PROFILE_0377,
    PROFILE_0378,
    PROFILE_0379,
    PROFILE_0380,
    PROFILE_0381,
    PROFILE_0382,
    PROFILE_0383,
    PROFILE_0384,
    PROFILE_0385,
    PROFILE_0386,
    PROFILE_0387,
    PROFILE_0388,
    PROFILE_0389,
    PROFILE_0390,
    PROFILE_0391,
    PROFILE_0392,
    PROFILE_0393,
    PROFILE_0394,
    PROFILE_0395,
    PROFILE_0396,
    PROFILE_0397,
    PROFILE_0398,
    PROFILE_0399,
    PROFILE_0400,
    PROFILE_0401,
    PROFILE_0402,
    PROFILE_0403,
    PROFILE_0404,
    PROFILE_0405,
    PROFILE_0406,
    PROFILE_0407,
    PROFILE_0408,
    PROFILE_0409,
    PROFILE_0410,
    PROFILE_0411,
    PROFILE_0412,
    PROFILE_0413,
    PROFILE_0414,
    PROFILE_0415,
    PROFILE_0416,
    PROFILE_0417,
    PROFILE_0418,
    PROFILE_0419,
    PROFILE_0420,
    PROFILE_0421,
    PROFILE_0422,
    PROFILE_0423,
    PROFILE_0424,
    PROFILE_0425,
    PROFILE_0426,
    PROFILE_0427,
    PROFILE_0428,
    PROFILE_0429,
    PROFILE_0430,
    PROFILE_0431,
    PROFILE_0432,
    PROFILE_0433,
    PROFILE_0434,
    PROFILE_0435,
    PROFILE_0436,
    PROFILE_0437,
    PROFILE_0438,
    PROFILE_0439,
    PROFILE_0440,
    PROFILE_0441,
    PROFILE_0442,
    PROFILE_0443,
    PROFILE_0444,
    PROFILE_0445,
    PROFILE_0446,
    PROFILE_0447,
    PROFILE_0448,
    PROFILE_0449,
    PROFILE_0450,
    PROFILE_0451,
    PROFILE_0452,
    PROFILE_0453,
    PROFILE_0454,
    PROFILE_0455,
    PROFILE_0456,
    PROFILE_0457,
    PROFILE_0458,
    PROFILE_0459,
    PROFILE_0460,
    PROFILE_0461,
    PROFILE_0462,
    PROFILE_0463,
    PROFILE_0464,
    PROFILE_0465,
    PROFILE_0466,
    PROFILE_0467,
    PROFILE_0468,
    PROFILE_0469,
    PROFILE_0470,
    PROFILE_0471,
    PROFILE_0472,
    PROFILE_0473,
    PROFILE_0474,
    PROFILE_0475,
    PROFILE_0476,
    PROFILE_0477,
    PROFILE_0478,
    PROFILE_0479,
    PROFILE_0480,
    PROFILE_0481,
    PROFILE_0482,
    PROFILE_0483,
    PROFILE_0484,
    PROFILE_0485,
    PROFILE_0486,
    PROFILE_0487,
    PROFILE_0488,
    PROFILE_0489,
    PROFILE_0490,
    PROFILE_0491,
    PROFILE_0492,
    PROFILE_0493,
    PROFILE_0494,
    PROFILE_0495,
    PROFILE_0496,
    PROFILE_0497,
    PROFILE_0498,
    PROFILE_0499,
    PROFILE_0500,
    PROFILE_0501,
    PROFILE_0502,
    PROFILE_0503,
    PROFILE_0504,
    PROFILE_0505,
    PROFILE_0506,
    PROFILE_0507,
    PROFILE_0508,
    PROFILE_0509,
    PROFILE_0510,
    PROFILE_0511,
    PROFILE_0512,
    PROFILE_0513,
    PROFILE_0514,
    PROFILE_0515,
    PROFILE_0516,
    PROFILE_0517,
    PROFILE_0518,
    PROFILE_0519,
    PROFILE_0520,
    PROFILE_0521,
    PROFILE_0522,
    PROFILE_0523,
    PROFILE_0524,
    PROFILE_0525,
    PROFILE_0526,
    PROFILE_0527,
    PROFILE_0528,
    PROFILE_0529,
    PROFILE_0530,
    PROFILE_0531,
    PROFILE_0532,
    PROFILE_0533,
    PROFILE_0534,
    PROFILE_0535,
    PROFILE_0536,
    PROFILE_0537,
    PROFILE_0538,
    PROFILE_0539,
    PROFILE_0540,
    PROFILE_0541,
    PROFILE_0542,
    PROFILE_0543,
    PROFILE_0544,
    PROFILE_0545,
    PROFILE_0546,
    PROFILE_0547,
    PROFILE_0548,
    PROFILE_0549,
    PROFILE_0550,
    PROFILE_0551,
    PROFILE_0552,
    PROFILE_0553,
    PROFILE_0554,
    PROFILE_0555,
    PROFILE_0556,
    PROFILE_0557,
    PROFILE_0558,
    PROFILE_0559,
    PROFILE_0560,
    PROFILE_0561,
    PROFILE_0562,
    PROFILE_0563,
    PROFILE_0564,
    PROFILE_0565,
    PROFILE_0566,
    PROFILE_0567,
    PROFILE_0568,
    PROFILE_0569,
    PROFILE_0570,
    PROFILE_0571,
    PROFILE_0572,
    PROFILE_0573,
    PROFILE_0574,
    PROFILE_0575,
    PROFILE_0576,
    PROFILE_0577,
    PROFILE_0578,
    PROFILE_0579,
    PROFILE_0580,
    PROFILE_0581,
    PROFILE_0582,
    PROFILE_0583,
    PROFILE_0584,
    PROFILE_0585,
    PROFILE_0586,
    PROFILE_0587,
    PROFILE_0588,
    PROFILE_0589,
    PROFILE_0590,
    PROFILE_0591,
    PROFILE_0592,
    PROFILE_0593,
    PROFILE_0594,
    PROFILE_0595,
    PROFILE_0596,
    PROFILE_0597,
    PROFILE_0598,
    PROFILE_0599,
    PROFILE_0600,
    PROFILE_0601,
    PROFILE_0602,
    PROFILE_0603,
    PROFILE_0604,
    PROFILE_0605,
    PROFILE_0606,
    PROFILE_0607,
    PROFILE_0608,
    PROFILE_0609,
    PROFILE_0610,
    PROFILE_0611,
    PROFILE_0612,
    PROFILE_0613,
    PROFILE_0614,
    PROFILE_0615,
    PROFILE_0616,
    PROFILE_0617,
    PROFILE_0618,
    PROFILE_0619,
    PROFILE_0620,
    PROFILE_0621,
    PROFILE_0622,
    PROFILE_0623,
    PROFILE_0624,
    PROFILE_0625,
    PROFILE_0626,
    PROFILE_0627,
    PROFILE_0628,
    PROFILE_0629,
    PROFILE_0630,
    PROFILE_0631,
    PROFILE_0632,
    PROFILE_0633,
    PROFILE_0634,
    PROFILE_0635,
    PROFILE_0636,
    PROFILE_0637,
    PROFILE_0638,
    PROFILE_0639,
    PROFILE_0640,
    PROFILE_0641,
    PROFILE_0642,
    PROFILE_0643,
    PROFILE_0644,
    PROFILE_0645,
    PROFILE_0646,
    PROFILE_0647,
    PROFILE_0648,
    PROFILE_0649,
    PROFILE_0650,
    PROFILE_0651,
    PROFILE_0652,
    PROFILE_0653,
    PROFILE_0654,
    PROFILE_0655,
    PROFILE_0656,
    PROFILE_0657,
    PROFILE_0658,
    PROFILE_0659,
    PROFILE_0660,
    PROFILE_0661,
    PROFILE_0662,
    PROFILE_0663,
    PROFILE_0664,
    PROFILE_0665,
    PROFILE_0666,
    PROFILE_0667,
    PROFILE_0668,
    PROFILE_0669,
    PROFILE_0670,
    PROFILE_0671,
    PROFILE_0672,
    PROFILE_0673,
    PROFILE_0674,
    PROFILE_0675,
    PROFILE_0676,
    PROFILE_0677,
    PROFILE_0678,
    PROFILE_0679,
    PROFILE_0680,
    PROFILE_0681,
    PROFILE_0682,
    PROFILE_0683,
    PROFILE_0684,
    PROFILE_0685,
    PROFILE_0686,
    PROFILE_0687,
    PROFILE_0688,
    PROFILE_0689,
    PROFILE_0690,
    PROFILE_0691,
    PROFILE_0692,
    PROFILE_0693,
    PROFILE_0694,
    PROFILE_0695,
    PROFILE_0696,
    PROFILE_0697,
    PROFILE_0698,
    PROFILE_0699,
    PROFILE_0700,
    PROFILE_0701,
    PROFILE_0702,
    PROFILE_0703,
    PROFILE_0704,
    PROFILE_0705,
    PROFILE_0706,
    PROFILE_0707,
    PROFILE_0708,
    PROFILE_0709,
    PROFILE_0710,
    PROFILE_0711,
    PROFILE_0712,
    PROFILE_0713,
    PROFILE_0714,
    PROFILE_0715,
    PROFILE_0716,
    PROFILE_0717,
    PROFILE_0718,
    PROFILE_0719,
    PROFILE_0720,
    PROFILE_0721,
    PROFILE_0722,
    PROFILE_0723,
    PROFILE_0724,
    PROFILE_0725,
    PROFILE_0726,
    PROFILE_0727,
    PROFILE_0728,
    PROFILE_0729,
    PROFILE_0730,
    PROFILE_0731,
    PROFILE_0732,
    PROFILE_0733,
    PROFILE_0734,
    PROFILE_0735,
    PROFILE_0736,
    PROFILE_0737,
    PROFILE_0738,
    PROFILE_0739,
    PROFILE_0740,
    PROFILE_0741,
    PROFILE_0742,
    PROFILE_0743,
    PROFILE_0744,
    PROFILE_0745,
    PROFILE_0746,
    PROFILE_0747,
    PROFILE_0748,
    PROFILE_0749,
    PROFILE_0750,
    PROFILE_0751,
    PROFILE_0752,
    PROFILE_0753,
    PROFILE_0754,
    PROFILE_0755,
    PROFILE_0756,
    PROFILE_0757,
    PROFILE_0758,
    PROFILE_0759,
    PROFILE_0760,
    PROFILE_0761,
    PROFILE_0762,
    PROFILE_0763,
    PROFILE_0764,
    PROFILE_0765,
    PROFILE_0766,
    PROFILE_0767,
    PROFILE_0768,
    PROFILE_0769,
    PROFILE_0770,
    PROFILE_0771,
    PROFILE_0772,
    PROFILE_0773,
    PROFILE_0774,
    PROFILE_0775,
    PROFILE_0776,
    PROFILE_0777,
    PROFILE_0778,
    PROFILE_0779,
    PROFILE_0780,
    PROFILE_0781,
    PROFILE_0782,
    PROFILE_0783,
    PROFILE_0784,
    PROFILE_0785,
    PROFILE_0786,
    PROFILE_0787,
    PROFILE_0788,
    PROFILE_0789,
    PROFILE_0790,
    PROFILE_0791,
    PROFILE_0792,
    PROFILE_0793,
    PROFILE_0794,
    PROFILE_0795,
    PROFILE_0796,
    PROFILE_0797,
    PROFILE_0798,
    PROFILE_0799,
    PROFILE_0800,
    PROFILE_0801,
    PROFILE_0802,
    PROFILE_0803,
    PROFILE_0804,
    PROFILE_0805,
    PROFILE_0806,
    PROFILE_0807,
    PROFILE_0808,
    PROFILE_0809,
    PROFILE_0810,
    PROFILE_0811,
    PROFILE_0812,
    PROFILE_0813,
    PROFILE_0814,
    PROFILE_0815,
    PROFILE_0816,
    PROFILE_0817,
    PROFILE_0818,
    PROFILE_0819,
    PROFILE_0820,
    PROFILE_0821,
    PROFILE_0822,
    PROFILE_0823,
    PROFILE_0824,
    PROFILE_0825,
    PROFILE_0826,
    PROFILE_0827,
    PROFILE_0828,
    PROFILE_0829,
    PROFILE_0830,
    PROFILE_0831,
    PROFILE_0832,
    PROFILE_0833,
    PROFILE_0834,
    PROFILE_0835,
    PROFILE_0836,
    PROFILE_0837,
    PROFILE_0838,
    PROFILE_0839,
    PROFILE_0840,
    PROFILE_0841,
    PROFILE_0842,
    PROFILE_0843,
    PROFILE_0844,
    PROFILE_0845,
    PROFILE_0846,
    PROFILE_0847,
    PROFILE_0848,
    PROFILE_0849,
    PROFILE_0850,
    PROFILE_0851,
    PROFILE_0852,
    PROFILE_0853,
    PROFILE_0854,
    PROFILE_0855,
    PROFILE_0856,
    PROFILE_0857,
    PROFILE_0858,
    PROFILE_0859,
    PROFILE_0860,
    PROFILE_0861,
    PROFILE_0862,
    PROFILE_0863,
    PROFILE_0864,
    PROFILE_0865,
    PROFILE_0866,
    PROFILE_0867,
    PROFILE_0868,
    PROFILE_0869,
    PROFILE_0870,
    PROFILE_0871,
    PROFILE_0872,
    PROFILE_0873,
    PROFILE_0874,
    PROFILE_0875,
    PROFILE_0876,
    PROFILE_0877,
    PROFILE_0878,
    PROFILE_0879,
    PROFILE_0880,
    PROFILE_0881,
    PROFILE_0882,
    PROFILE_0883,
    PROFILE_0884,
    PROFILE_0885,
    PROFILE_0886,
    PROFILE_0887,
    PROFILE_0888,
    PROFILE_0889,
    PROFILE_0890,
    PROFILE_0891,
    PROFILE_0892,
    PROFILE_0893,
    PROFILE_0894,
    PROFILE_0895,
    PROFILE_0896,
    PROFILE_0897,
    PROFILE_0898,
    PROFILE_0899,
    PROFILE_0900,
    PROFILE_0901,
    PROFILE_0902,
    PROFILE_0903,
    PROFILE_0904,
    PROFILE_0905,
    PROFILE_0906,
    PROFILE_0907,
    PROFILE_0908,
    PROFILE_0909,
    PROFILE_0910,
    PROFILE_0911,
    PROFILE_0912,
    PROFILE_0913,
    PROFILE_0914,
    PROFILE_0915,
    PROFILE_0916,
    PROFILE_0917,
    PROFILE_0918,
    PROFILE_0919,
    PROFILE_0920,
    PROFILE_0921,
    PROFILE_0922,
    PROFILE_0923,
    PROFILE_0924,
    PROFILE_0925,
    PROFILE_0926,
    PROFILE_0927,
    PROFILE_0928,
    PROFILE_0929,
    PROFILE_0930,
    PROFILE_0931,
    PROFILE_0932,
    PROFILE_0933,
    PROFILE_0934,
    PROFILE_0935,
    PROFILE_0936,
    PROFILE_0937,
    PROFILE_0938,
    PROFILE_0939,
    PROFILE_0940,
    PROFILE_0941,
    PROFILE_0942,
    PROFILE_0943,
    PROFILE_0944,
    PROFILE_0945,
    PROFILE_0946,
    PROFILE_0947,
    PROFILE_0948,
    PROFILE_0949,
    PROFILE_0950,
    PROFILE_0951,
    PROFILE_0952,
    PROFILE_0953,
    PROFILE_0954,
    PROFILE_0955,
    PROFILE_0956,
    PROFILE_0957,
    PROFILE_0958,
    PROFILE_0959,
    PROFILE_0960,
    PROFILE_0961,
    PROFILE_0962,
    PROFILE_0963,
    PROFILE_0964,
    PROFILE_0965,
    PROFILE_0966,
    PROFILE_0967,
    PROFILE_0968,
    PROFILE_0969,
    PROFILE_0970,
    PROFILE_0971,
    PROFILE_0972,
    PROFILE_0973,
    PROFILE_0974,
    PROFILE_0975,
    PROFILE_0976,
    PROFILE_0977,
    PROFILE_0978,
    PROFILE_0979,
    PROFILE_0980,
    PROFILE_0981,
    PROFILE_0982,
    PROFILE_0983,
    PROFILE_0984,
    PROFILE_0985,
    PROFILE_0986,
    PROFILE_0987,
    PROFILE_0988,
    PROFILE_0989,
    PROFILE_0990,
    PROFILE_0991,
    PROFILE_0992,
    PROFILE_0993,
    PROFILE_0994,
    PROFILE_0995,
    PROFILE_0996,
    PROFILE_0997,
    PROFILE_0998,
    PROFILE_0999,
    PROFILE_1000,
    PROFILE_1001,
    PROFILE_1002,
    PROFILE_1003,
    PROFILE_1004,
    PROFILE_1005,
    PROFILE_1006,
    PROFILE_1007,
    PROFILE_1008,
    PROFILE_1009,
    PROFILE_1010,
    PROFILE_1011,
    PROFILE_1012,
    PROFILE_1013,
    PROFILE_1014,
    PROFILE_1015,
    PROFILE_1016,
    PROFILE_1017,
    PROFILE_1018,
    PROFILE_1019,
    PROFILE_1020,
    PROFILE_1021,
    PROFILE_1022,
    PROFILE_1023,
    PROFILE_1024,
    PROFILE_1025,
    PROFILE_1026,
    PROFILE_1027,
    PROFILE_1028,
    PROFILE_1029,
    PROFILE_1030,
    PROFILE_1031,
    PROFILE_1032,
    PROFILE_1033,
    PROFILE_1034,
    PROFILE_1035,
    PROFILE_1036,
    PROFILE_1037,
    PROFILE_1038,
    PROFILE_1039,
    PROFILE_1040,
    PROFILE_1041,
    PROFILE_1042,
    PROFILE_1043,
    PROFILE_1044,
    PROFILE_1045,
    PROFILE_1046,
    PROFILE_1047,
    PROFILE_1048,
    PROFILE_1049,
];

pub fn lookup(station_id: u16) -> Option<StationProfile> { STATION_PROFILES.iter().copied().find(|p| p.station_id == station_id) }
pub fn basin_for_station(station_id: u16) -> &'static str { lookup(station_id).map(|p| p.basin).unwrap_or("unmapped") }
pub fn radio_budget_for_station(station_id: u16) -> u16 { lookup(station_id).map(|p| 2 + p.radio_class as u16 * 3).unwrap_or(8) }
