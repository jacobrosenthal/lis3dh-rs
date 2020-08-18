#![allow(non_camel_case_types)]

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Register {
    STATUS_AUX = 0x07,
    OUT_ADC1_L = 0x08,
    OUT_ADC1_H = 0x09,
    OUT_ADC2_L = 0x0A,
    OUT_ADC2_H = 0x0B,
    OUT_ADC3_L = 0x0C,
    OUT_ADC3_H = 0x0D,
    WHOAMI = 0x0F,
    CTRL0 = 0x1E,
    TEMP_CFG = 0x1F,
    CTRL1 = 0x20,
    CTRL2 = 0x21,
    CTRL3 = 0x22,
    CTRL4 = 0x23,
    CTRL5 = 0x24,
    CTRL6 = 0x25,
    REFERENCE = 0x26,
    STATUS = 0x27,
    OUT_X_L = 0x28,
    OUT_X_H = 0x29,
    OUT_Y_L = 0x2A,
    OUT_Y_H = 0x2B,
    OUT_Z_L = 0x2C,
    OUT_Z_H = 0x2D,
    FIFO_CTRL = 0x2E,
    FIFO_SRC = 0x2F,
    INT1_CFG = 0x30,
    INT1_SRC = 0x31,
    INT1_THS = 0x32,
    INT1_DURATION = 0x33,
    INT2_CFG = 0x34,
    INT2_SRC = 0x35,
    INT2_THS = 0x36,
    INT2_DURATION = 0x37,
    CLICK_CFG = 0x38,
    CLICK_SRC = 0x39,
    CLICK_THS = 0x3A,
    TIME_LIMIT = 0x3B,
    TIME_LATENCY = 0x3C,
    TIME_WINDOW = 0x3D,
    ACT_THS = 0x3E,
    ACT_DUR = 0x3F,
}

impl Register {
    pub fn addr(self) -> u8 {
        self as u8
    }

    pub fn read_only(self) -> bool {
        match self {
            Register::STATUS_AUX
            | Register::OUT_ADC1_L
            | Register::OUT_ADC1_H
            | Register::OUT_ADC2_L
            | Register::OUT_ADC2_H
            | Register::OUT_ADC3_L
            | Register::OUT_ADC3_H
            | Register::WHOAMI
            | Register::STATUS
            | Register::OUT_X_L
            | Register::OUT_X_H
            | Register::OUT_Y_L
            | Register::OUT_Y_H
            | Register::OUT_Z_L
            | Register::OUT_Z_H
            | Register::FIFO_SRC
            | Register::INT1_SRC
            | Register::INT2_SRC
            | Register::CLICK_SRC => true,
            _ => false,
        }
    }
}

/// Data status structure. Decoded from the `STATUS_REG` register.
///
/// `STATUS_REG` has the following bit fields:
///   * `ZYXOR` - X, Y and Z-axis data overrun
///   * `ZOR` - Z-axis data overrun
///   * `YOR` - Y-axis data overrun
///   * `XOR` - X-axis data overrun
///   * `ZYXDA` - X, Y and Z-axis new data available
///   * `ZDA` - Z-axis new data available
///   * `YDA` Y-axis new data available
///   * `XDA` X-axis new data available
///
/// This struct splits the fields into more convenient groups:
///  * `zyxor` -> `ZYXOR`
///  * `xyzor` -> (`XOR`, `YOR`, `ZOR`)
///  * `zyxda` -> `ZYXDA`
///  * `xyzda` -> (`XDA`, `YDA`, `ZDA`)
#[derive(Debug)]
pub struct DataStatus {
    /// ZYXOR bit
    pub zyxor: bool,

    /// (XOR, YOR, ZOR) bits
    pub xyzor: (bool, bool, bool),

    /// ZYXDA bit
    pub zyxda: bool,

    /// (XDA, YDA, ZDA) bits
    pub xyzda: (bool, bool, bool),
}
// === STATUS_REG (27h) ===

pub const ZYXOR: u8 = 0b1000_0000;
pub const ZOR: u8 = 0b0100_0000;
pub const YOR: u8 = 0b0010_0000;
pub const XOR: u8 = 0b0001_0000;
pub const ZYXDA: u8 = 0b0000_1000;
pub const ZDA: u8 = 0b0000_0100;
pub const YDA: u8 = 0b0000_0010;
pub const XDA: u8 = 0b0000_0001;
