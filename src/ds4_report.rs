pub const REPORT_LENGTH: usize = 79;

pub struct DS4Report<'a> {
    buffer: &'a [u8],
    bluetooth: bool
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
            let bluetooth = buffer[0] == 0x11 && buffer[1] == 0xc0 && buffer[2] == 0x00;
            Some(DS4Report { buffer, bluetooth })
        }
    }

    pub fn is_bluetooth(&self) -> bool {
        self.bluetooth
    }

    // IMU
    pub fn get_accelerometer_z(&self) -> f32 {
        let idx = if self.is_bluetooth() { 15 } else { 13 };
        raw_imu_to_g_force(read_u16(&self.buffer[idx..]))
    }

    pub fn get_accelerometer_y(&self) -> f32 {
        let idx = if self.is_bluetooth() { 17 } else { 15 };
        raw_imu_to_g_force(read_u16(&self.buffer[idx..]))
    }

    pub fn get_accelerometer_x(&self) -> f32 {
        let idx = if self.is_bluetooth() { 19 } else { 17 };
        raw_imu_to_g_force(read_u16(&self.buffer[idx..]))
    }

    pub fn get_gyroscope_z(&self) -> f32 {
        let idx = if self.is_bluetooth() { 25 } else { 23 };
        raw_imu_to_g_force(read_u16(&self.buffer[idx..]))
    }

    pub fn get_gyroscope_y(&self) -> f32 {
        let idx = if self.is_bluetooth() { 23 } else { 21 };
        raw_imu_to_g_force(read_u16(&self.buffer[idx..]))
    }

    pub fn get_gyroscope_x(&self) -> f32 {
        let idx = if self.is_bluetooth() { 21 } else { 19 };
        raw_imu_to_g_force(read_u16(&self.buffer[idx..]))
    }

    // Joysticks
    pub fn get_left_joystick(&self) -> Joystick {
        let x_idx = if self.is_bluetooth() { 3 } else { 1 };
        let y_idx = if self.is_bluetooth() { 4 } else { 2 };
        Joystick{ x: joystick_convert(self.buffer[x_idx]), y: joystick_convert(self.buffer[y_idx]) }
    }

    pub fn get_right_joystick(&self) -> Joystick {
        let x_idx = if self.is_bluetooth() { 4 } else { 2 };
        let y_idx = if self.is_bluetooth() { 5 } else { 3 };
        Joystick{ x: joystick_convert(self.buffer[x_idx]), y: joystick_convert(self.buffer[y_idx]) }
    }

    // Analog triggers
    pub fn get_l2_trigger_analog(&self) -> u8 {
        let idx = if self.is_bluetooth() { 10 } else { 8 };
        self.buffer[idx]
    }

    pub fn get_r2_trigger_analog(&self) -> u8 {
        let idx = if self.is_bluetooth() { 11 } else { 9 };
        self.buffer[idx]
    }

    // Shape Buttons
    pub fn get_button_triangle(&self) -> bool {
        let idx = if self.is_bluetooth() { 7 } else { 5 };
        check_bit( self.buffer[idx], 7 )
    }

    pub fn get_button_circle(&self) -> bool {
        let idx = if self.is_bluetooth() { 7 } else { 5 };
        check_bit( self.buffer[idx], 6 )
    }

    pub fn get_button_cross(&self) -> bool {
        let idx = if self.is_bluetooth() { 7 } else { 5 };
        check_bit( self.buffer[idx], 5 )
    }

    pub fn get_button_square(&self) -> bool {
        let idx = if self.is_bluetooth() { 7 } else { 5 };
        check_bit( self.buffer[idx], 4 )
    }

    pub fn get_button_dpad(&self) -> Dpad {
        let idx = if self.is_bluetooth() { 7 } else { 5 };
        match self.buffer[idx] & 0x0f {
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
        let idx = if self.is_bluetooth() { 8 } else { 6 };
        check_bit( self.buffer[idx], 7 )
    }

    pub fn get_button_l3(&self) -> bool {
        let idx = if self.is_bluetooth() { 8 } else { 6 };
        check_bit( self.buffer[idx], 6 )
    }

    pub fn get_button_options(&self) -> bool {
        let idx = if self.is_bluetooth() { 8 } else { 6 };
        check_bit( self.buffer[idx], 5 )
    }

    pub fn get_button_share(&self) -> bool {
        let idx = if self.is_bluetooth() { 8 } else { 6 };
        check_bit( self.buffer[idx], 4 )
    }

    pub fn get_button_r2(&self) -> bool {
        let idx = if self.is_bluetooth() { 8 } else { 6 };
        check_bit( self.buffer[idx], 3 )
    }

    pub fn get_button_l2(&self) -> bool {
        let idx = if self.is_bluetooth() { 8 } else { 6 };
        check_bit( self.buffer[idx], 2 )
    }

    pub fn get_button_r1(&self) -> bool {
        let idx = if self.is_bluetooth() { 8 } else { 6 };
        check_bit( self.buffer[idx], 1 )
    }

    pub fn get_button_l1(&self) -> bool {
        let idx = if self.is_bluetooth() { 8 } else { 6 };
        check_bit( self.buffer[idx], 0 )
    }

    pub fn get_button_touchpad(&self) -> bool {
        let idx = if self.is_bluetooth() { 9 } else { 7 };
        check_bit( self.buffer[idx], 1 )
    }

    pub fn get_button_playstation(&self) -> bool {
        let idx = if self.is_bluetooth() { 9 } else { 7 };
        check_bit( self.buffer[idx], 0 )
    }

    // Metadata
    pub fn get_report_id(&self) -> u8 {
        let idx = if self.is_bluetooth() { 2 } else { 0 };
        self.buffer[idx]
    }

    pub fn get_report_count(&self) -> u8 {
        let idx = if self.is_bluetooth() { 9 } else { 7 };
        self.buffer[idx] >> 2
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