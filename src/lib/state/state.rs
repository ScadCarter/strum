use crate::lib::cmd;

pub struct State {
    pub devices: Vec<Device>,
    pub cursor: usize,
}

impl State {
    pub fn default() -> Result<Self, String> {
        Ok(Self {
            devices: Device::get_all()?,
            cursor: 0,
        })
    }

    pub fn update(&mut self) -> Result<(), String> {
        self.devices = Device::get_all()?;

        Ok(())
    }

    pub fn cursor_up(&mut self) {
        if self.cursor != 0 {
            self.cursor -= 1;
        }
    }

    pub fn cursor_down(&mut self) {
        if self.cursor != 0 {
            self.cursor += 1;
        }
    }
}

#[derive(Debug)]
pub struct Device {
    pub id: String,
    pub name: String,
}

impl Device {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }

    pub fn get_all() -> Result<Vec<Device>, String> {
        let output_string = cmd::get_bluetooth_devices()?;

        let lines: Vec<&str> = output_string.split('\n').collect();

        let devices = lines
            .iter()
            .filter(|s| s.len() != 0)
            .map(|s| {
                let res: Vec<&str> = s.split(' ').collect();

                Device::new(res[1].to_string(), res[2].to_string())
            })
            .collect();

        Ok(devices)
    }
}
