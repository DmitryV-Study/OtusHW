use crate::smart_house::smart_device::{SmartDevice, SmartSocket, SmartThermometer};

pub trait DeviceInfoProvider {
    fn device_info(&self) -> String;
}

#[derive(Copy, Clone)]
pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn device_info(&self) -> String {
        let ret = format!("{} : {}", &self.socket.name(), &self.socket.state());
        ret
    }
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn device_info(&self) -> String {
        let ret = format!(
            "{} : {}\n{} : {}",
            &self.socket.name(),
            &self.socket.state(),
            &self.thermo.name(),
            &self.thermo.state()
        );
        ret
    }
}
