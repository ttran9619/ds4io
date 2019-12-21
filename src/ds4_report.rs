pub const REPORT_LENGTH: usize = 64;

pub struct DS4Report<'a> {
    buffer: &'a [u8],
}

pub struct Joystick {
    pub x: i16,
    pub y: i16
}

pub struct Dpad {
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub left: bool
}

#[allow(dead_code)]
impl<'a> DS4Report<'a> {

    pub fn new(buffer: &'a [u8]) -> Option<DS4Report<'a>> {
        if buffer.len() < REPORT_LENGTH {
            None
        } else {
            Some(DS4Report { buffer })
        }
    }

    // IMU
    pub fn get_accelerometer_z(&self) -> f32 {
        raw_imu_to_g_force(read_u16(&self.buffer[13..]))
    }

    pub fn get_accelerometer_y(&self) -> f32 {
        raw_imu_to_g_force(read_u16(&self.buffer[15..]))
    }

    pub fn get_accelerometer_x(&self) -> f32 {
        raw_imu_to_g_force(read_u16(&self.buffer[17..]))
    }

    pub fn get_gyroscope_z(&self) -> f32 {
        raw_imu_to_g_force(read_u16(&self.buffer[23..]))
    }

    pub fn get_gyroscope_y(&self) -> f32 {
        raw_imu_to_g_force(read_u16(&self.buffer[21..]))
    }

    pub fn get_gyroscope_x(&self) -> f32 {
        raw_imu_to_g_force(read_u16(&self.buffer[19..]))
    }

    // Joysticks
    pub fn get_left_joystick(&self) -> Joystick {
        Joystick{ x: joystick_convert(self.buffer[1]), y: joystick_convert(self.buffer[2]) }
    }

    pub fn get_right_joystick(&self) -> Joystick {
        Joystick{ x: joystick_convert(self.buffer[2]), y: joystick_convert(self.buffer[3]) }
    }

    // Analog triggers
    pub fn get_l2_trigger_analog(&self) -> u8 {
        self.buffer[8]
    }

    pub fn get_r2_trigger_analog(&self) -> u8 {
        self.buffer[9]
    }

    // Shape Buttons
    pub fn get_button_triangle(&self) -> bool {
        check_bit( self.buffer[5], 7 )
    }

    pub fn get_button_circle(&self) -> bool {
        check_bit( self.buffer[5], 6 )
    }

    pub fn get_button_cross(&self) -> bool {
        check_bit( self.buffer[5], 5 )
    }

    pub fn get_button_square(&self) -> bool {
        check_bit( self.buffer[5], 4 )
    }

    pub fn get_button_dpad(&self) -> Dpad {
        match self.buffer[5] & 0x0f {
            0b0000 => Dpad{ up: true,   right: false,   down: false,    left: false },
            0b0001 => Dpad{ up: true,   right: true,    down: false,    left: false },
            0b0010 => Dpad{ up: false,  right: true,    down: false,    left: false },
            0b0011 => Dpad{ up: false,  right: true,    down: true,     left: false },
            0b0100 => Dpad{ up: false,  right: false,   down: true,     left: false },
            0b0101 => Dpad{ up: false,  right: false,   down: true,     left: true  },
            0b0110 => Dpad{ up: false,  right: false,   down: false,    left: true  },
            0b0111 => Dpad{ up: true,   right: false,   down: false,    left: true  },
            _ =>      Dpad{ up: false,  right: false,   down: false,    left: false }
        }
    }

    pub fn get_button_r3(&self) -> bool {
        check_bit( self.buffer[6], 7 )
    }

    pub fn get_button_l3(&self) -> bool {
        check_bit( self.buffer[6], 6 )
    }

    pub fn get_button_options(&self) -> bool {
        check_bit( self.buffer[6], 5 )
    }

    pub fn get_button_share(&self) -> bool {
        check_bit( self.buffer[6], 4 )
    }

    pub fn get_button_r2(&self) -> bool {
        check_bit( self.buffer[6], 3 )
    }

    pub fn get_button_l2(&self) -> bool {
        check_bit( self.buffer[6], 2 )
    }

    pub fn get_button_r1(&self) -> bool {
        check_bit( self.buffer[6], 1 )
    }

    pub fn get_button_l1(&self) -> bool {
        check_bit( self.buffer[6], 0 )
    }

    pub fn get_button_touchpad(&self) -> bool {
        check_bit( self.buffer[7], 1 )
    }

    pub fn get_button_playstation(&self) -> bool {
        check_bit( self.buffer[7], 0 )
    }

    // Metadata
    pub fn get_report_id(&self) -> u8 {
        self.buffer[0]
    }

    pub fn get_report_count(&self) -> u8 {
        self.buffer[7] >> 2
    }
}

fn joystick_convert(raw: u8) -> i16 {
    raw as i16 - 128
}

fn check_bit(data: u8, bit: u8) -> bool {
    data & ( 1 << bit ) != 0
}

fn read_u16(data: &[u8]) -> u16 {
    (data[0] as u16) |
    ((data[1] as u16) << 8)
}

fn raw_imu_to_g_force(raw: u16) -> f32 {
    raw as i16 as f32 / 8192.0
}