use usbd_hid::descriptor::generator_prelude::*;

#[derive(Copy, Clone)]
pub enum Reports {
    MouseReport = 0x01,
    CpiReport = 0x02,
    LiftReport = 0x03,
    CommandResponse = 0x04,
    SensorImage = 0x05,
}

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = MOUSE) = {
        (report_id = 0x01,) = {
            (collection = PHYSICAL, usage = POINTER) = {
                (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = BUTTON_8) = {
                    #[packed_bits 8] #[item_settings data,variable,absolute] buttons=input;
                };
                (usage_page = GENERIC_DESKTOP,) = {
                    (usage = X,) = {
                        #[item_settings data,variable,relative] x=input;
                    };
                    (usage = Y,) = {
                        #[item_settings data,variable,relative] y=input;
                    };
                    (usage = WHEEL,) = {
                        #[item_settings data,variable,relative] wheel=input;
                    };
                };
            };
        };
    },
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = KEYBOARD) = {
        (report_id = 0x02,) = {
            (usage_page = KEYBOARD, usage_min = 0xE0, usage_max = 0xE7) = {
                #[packed_bits 8] #[item_settings data,variable,absolute] modifier=input;
            };
            (usage_page = KEYBOARD, usage_min = 0x00, usage_max = 0xDD) = {
                #[item_settings data,array,absolute] keys=input;
            };
        };
    },
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = KEYBOARD) = {
        (report_id = 0x03,) = {
            (usage_page = KEYBOARD, usage_min = 0xE0, usage_max = 0xE7) = {
                #[packed_bits 8] #[item_settings data,variable,absolute] modifier=input;
            };
            (usage_page = KEYBOARD, usage_min = 0x00, usage_max = 0xDD) = {
                #[item_settings data,array,absolute] keys=input;
            };
        };
    },
    (collection = APPLICATION, usage_page = VENDOR_DEFINED_START, usage = 0x01) = {
        (report_id = 0x04,) = {
            (usage = 0x01, usage_min = 0x00, usage_max = 0xFF) = {
                #[item_settings data,variable] command=feature;
            };
            (usage = 0x01, usage_min = 0x00, usage_max = 0xFF) = {
                #[item_settings data,array] args=feature;
            };
            (usage = 0x01, usage_min = 0x00, usage_max = 0xFF) = {
                #[item_settings data,variable] response=input;
            };
            (usage = 0x01, usage_min = 0x00, usage_max = 0xFF) = {
                #[item_settings data,array] data=input;
            };
        };
    },
    (collection = APPLICATION, usage_page = VENDOR_DEFINED_START, usage = 0x02) = {
        (report_id = 0x05,) = {
            (usage = 0x02, usage_min = 0x00, usage_max = 0xFF) = {
                #[item_settings data,variable] large_data_ind=input;
            };
            (usage = 0x02, usage_min = 0x00, usage_max = 0xFF) = {
                #[item_settings data,array] large_data=input;
            };
        };
    }
)]
#[allow(dead_code)]
pub struct MouseReport {
    pub buttons: u8,
    pub x: i16,
    pub y: i16,
    pub wheel: i8,
    //
    pub modifier: u8,
    pub keys: [u8; 6],
    //
    pub command: u8,
    pub args: [u8; 4],
    pub response: u8,
    pub data: [u8; 4],
    // Sensor Data
    pub large_data_ind: u8,
    pub large_data: [u8; 62],
}

impl MouseReport {
    pub fn get_report<'a>(&self, report: Reports, buf: &'a mut [u8]) -> &'a [u8] {
        buf[0] = report as u8;
        let size = match report {
            Reports::MouseReport => {
                buf[1] = self.buttons;
                buf[2..4].copy_from_slice(&self.x.to_le_bytes());
                buf[4..6].copy_from_slice(&self.y.to_le_bytes());
                buf[6] = self.wheel as u8;
                // Length plus report ID byte
                6 + 1
            }
            Reports::CpiReport | Reports::LiftReport => {
                buf[1] = self.modifier;
                buf[2..8].copy_from_slice(&self.keys);
                // Length plus report ID byte
                7 + 1
            }
            Reports::CommandResponse => {
                buf[1] = self.response;
                buf[2..6].copy_from_slice(&self.data);
                // Length plus report ID byte
                5 + 1
            }
            Reports::SensorImage => {
                buf[1] = self.large_data_ind;
                buf[2..64].copy_from_slice(&self.large_data);
                // Length plus report ID byte
                63 + 1
            }
        };
        &buf[..size]
    }
}
