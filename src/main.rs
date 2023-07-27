mod smart_house;

use std::borrow::Borrow;
use crate::smart_house::{
    info_provider::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider},
    smart_device::{SmartSocket, SmartThermometer},
    Room, SmartHouse,
};

const LIVING_ROOM: &str = "Гостиная";
const BED_ROOM: &str = "Спальня";
const EMPTY_ROOM: &str = "-";

fn main() {
    // Инициализация устройств
    //use crate::smart_house::SmartHouse;
    //smart_device::SmartDevice;
    //, SmartSocket, SmartThermometer};

    // Инициализация дома
    let mut house = SmartHouse::new("Fantasy house");

    let mut living_room = Room::new();
    living_room
        .devices
        .insert(String::from("TV socket"), Box::new(SmartSocket {}));
    house.rooms.insert(String::from(LIVING_ROOM), living_room);

    let mut bed_room = Room::new();
    bed_room
        .devices
        .insert(String::from("Lamp socket"), Box::new(SmartSocket {}));
    bed_room
        .devices
        .insert(String::from("Floor"), Box::new(SmartThermometer {}));

    house.rooms.insert(String::from(BED_ROOM), bed_room);

    let mut empty_room = Room::new();
    empty_room.devices.insert("DevToDel".to_string(), Box::new(SmartThermometer {}));
    empty_room.devices.remove("DevToDel");

    house.rooms.insert(String::from(EMPTY_ROOM), empty_room);
    house.rooms.remove(EMPTY_ROOM);

    println!("House: {:?}", house.name);
    println!("Rooms: {:?}", house.get_rooms());
    for room in house.get_rooms() {
        println!("\tRoom: {}", room);
        for dev in house.get_devices(&room).unwrap() {
            println!("\t\tDev: {:?}", dev);
        }
    }

    let fake_room_name = "fake room";
    let fake_rom = house.get_devices(fake_room_name);
    match fake_rom {
        Ok(_) => panic!("Room named {} exists", fake_room_name),
        Err(_) => println!("No room named {}", fake_room_name),
    }
    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider {
        socket: SmartSocket {},
    };

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &SmartSocket {},
        thermo: &SmartThermometer {},
    };

    let report1 = house.create_report(&info_provider_1);
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
