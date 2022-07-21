use crate::RadioDriver;

pub struct SR1020<DRIVE: RadioDriver> {
    driver: DRIVE,
}

impl<DRIVE: RadioDriver> SR1020<DRIVE> {
    pub fn new(driver: DRIVE) -> Self {
        Self { driver }
    }
}
