use crate::smart_house_lib::smart_house;
pub use crate::smart_house_lib::smart_house::FindError;

// Пользовательские устройства:
#[derive(Default)]
pub struct SmartSocket {
    pub name: String,
    pub state: String,
}
// pub struct SmartThermometer {
//     pub name: String,
//     pub temperature: f32
// }

#[derive(Default)]
pub struct OwningDeviceInfoProvider {
    socket: SmartSocket,
    pub house: smart_house::SmartHouse,
}

impl OwningDeviceInfoProvider {
    pub fn new() -> OwningDeviceInfoProvider {
        let socket1 = SmartSocket {
            name: String::from("Socket 1"),
            state: String::from("working"),
        };

        let mut my_house = smart_house::SmartHouse::new("My house");
        my_house.add_room("Room A");
        my_house.add_device("Room A", "Socket 1").unwrap();

        OwningDeviceInfoProvider {
            socket: socket1,
            house: my_house,
        }
    }

    pub fn create_report(&self) -> String {
        self.house.create_report(self)
    }

    pub fn get_rooms(&self) -> Vec<&str> {
        self.house.get_rooms()
    }

    pub fn get_devices(&self, room_name: &str) -> Result<Vec<&str>, FindError> {
        self.house.get_devices(room_name)
    }
}

impl smart_house::DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_info(&self, room: &str, name: &str) -> String {
        let info: String = if self.socket.name == name {
            format!(
                "room: {}, device: {}, state: {}",
                room, self.socket.name, self.socket.state
            )
        } else {
            format!("room: {}, device: {}, not found", room, self.socket.name)
        };
        info
    }
}

pub fn run_owning_provider() -> String {
    let info_provider_1 = OwningDeviceInfoProvider::new();

    let rooms = info_provider_1.get_rooms();
    println!("{} rooms: {:?}", info_provider_1.house.name, rooms);

    // Positive result
    let devices = info_provider_1.get_devices("Room A");
    match devices {
        Ok(v) => println!("{} Room A devices: {:?}", info_provider_1.house.name, v),
        Err(e) => println!("{}", e),
    };

    // Negative result
    let devices = info_provider_1.get_devices("Room B");
    match devices {
        Ok(v) => println!("{} Room A devices: {:?}", info_provider_1.house.name, v),
        Err(e) => println!("{}", e),
    };

    let report1 = info_provider_1.create_report();

    println!("Report #1: {report1}");
    report1
}
