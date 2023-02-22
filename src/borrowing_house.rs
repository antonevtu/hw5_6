use crate::smart_house_lib::smart_house;
pub use crate::smart_house_lib::smart_house::FindError;

// Пользовательские устройства:
pub struct SmartSocket {
    pub name: String,
    pub state: String,
}
pub struct SmartThermometer {
    pub name: String,
    pub temperature: f32,
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
    pub house: smart_house::SmartHouse,
}

impl BorrowingDeviceInfoProvider<'_, '_> {
    pub fn new<'a>(
        socket: &'a SmartSocket,
        thermo: &'a SmartThermometer,
    ) -> BorrowingDeviceInfoProvider<'a, 'a> {
        let my_house = smart_house::SmartHouse::new("My house");
        BorrowingDeviceInfoProvider {
            socket,
            thermo,
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

impl<'a, 'b> smart_house::DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_device_info(&self, room: &str, name: &str) -> String {
        let info: String;
        if self.socket.name == name {
            info = format!(
                "room: {}, device: {}, state: {}",
                room, self.socket.name, self.socket.state
            );
        } else if self.thermo.name == name {
            info = format!(
                "room: {}, device: {}, state: {}°C",
                room, self.thermo.name, self.thermo.temperature
            );
        } else {
            info = format!("room: {}, device: {}, not found", room, self.socket.name);
        }
        info
    }
}

pub fn run_borrowing_provider() -> String {
    let socket2 = SmartSocket {
        name: String::from("Socket 2"),
        state: String::from("broken"),
    };
    let thermo = SmartThermometer {
        name: String::from("Thermometer 1"),
        temperature: 25.4,
    };

    let mut info_provider_2 = BorrowingDeviceInfoProvider::new(&socket2, &thermo);
    info_provider_2.house.add_room("Room B");
    info_provider_2
        .house
        .add_device("Room B", "Socket 2")
        .unwrap();
    info_provider_2
        .house
        .add_device("Room B", "Thermometer 1")
        .unwrap();

    let rooms = info_provider_2.get_rooms();
    println!("{} rooms: {:?}", info_provider_2.house.name, rooms);

    // Positive result
    let devices = info_provider_2.get_devices("Room B");
    match devices {
        Ok(v) => println!("{} Room B devices: {:?}", info_provider_2.house.name, v),
        Err(e) => println!("{}", e),
    };

    // Negative result
    let devices = info_provider_2.get_devices("Room A");
    match devices {
        Ok(v) => println!("{} Room A devices: {:?}", info_provider_2.house.name, v),
        Err(e) => println!("{}", e),
    };

    let report2 = info_provider_2.create_report();

    // Выводим отчёты на экран:
    println!("Report #2: {report2}");
    report2
}
