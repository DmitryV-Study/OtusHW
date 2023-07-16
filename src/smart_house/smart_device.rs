pub trait SmartDevice {
    fn name(&self) -> &str;

    fn state(&self) -> &str {
        "работаю"
    }
}

// Пользовательские устройства:
#[derive(Copy, Clone, Debug)]
pub struct SmartSocket {}

impl SmartDevice for SmartSocket {
    fn name(&self) -> &str {
        "SmartSocket"
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SmartThermometer {}

impl SmartDevice for SmartThermometer {
    fn name(&self) -> &str {
        "SmartThermometer"
    }
}
