use std::collections::{HashMap, HashSet};
use std::pin::Pin;

pub static NAME_DEV_1: &str = "Socket 1";
pub static NAME_DEV_2: &str = "Socket 2";
pub static NAME_DEV_3: &str = "Thermo 1";

#[derive(Default)]
pub struct SmartHouse {
    pub name: String,
    rooms: HashMap<String, Room>,
}

#[derive(Default)]
struct Room {
    devices: HashSet<String>,
}

pub enum Errors {
    RoomNotFound,
    DeviceExist,
    DeviceNotAvailable,
}

impl SmartHouse {
    /// Creates empty SmartHouse object with dynamic content
    /// ```
    /// use hw5_6::smart_house_lib::smart_house;
    /// let house = smart_house::SmartHouse::new("My house");
    /// ```
    pub fn new(name: &str) -> Self {
        SmartHouse {
            name: name.to_string(),
            rooms: HashMap::new(),
        }
    }

    /// Adding new room, if not exists. If room already exists, nothing happens
    /// ```
    /// use hw5_6::smart_house_lib::smart_house;
    /// let mut house = smart_house::SmartHouse::new("My house");
    /// house.add_room("Room A");
    /// ```
    pub fn add_room(&mut self, name: &str) {
        self.rooms.entry(name.to_string()).or_default();
    }

    /// Delete room from house, if exists
    /// ```
    /// use hw5_6::smart_house_lib::smart_house;
    /// let mut house = smart_house::SmartHouse::new("My house");
    /// house.add_room("Room A");
    /// house.remove_room("Room A");
    /// house.remove_room("Room B");
    /// ```
    pub fn remove_room(&mut self, name: &str) {
        self.rooms.remove(name);
    }

    /// Returns rooms in the house
    /// ```
    /// use hw5_6::smart_house_lib::smart_house;
    /// let mut house = smart_house::SmartHouse::new("My house");
    /// house.add_room("Room A");
    /// let rooms = house.get_rooms();
    /// assert_eq!(rooms[0], "Room A")
    /// ```
    pub fn get_rooms(&self) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::with_capacity(10);
        for room_name in self.rooms.keys() {
            result.push(room_name);
        }
        result
    }

    /// Adding device to the room. If device exists, return Errors::DeviceExist
    pub fn  add_device(&mut self, room: &str, device: &str) -> Result<bool, Errors> {
        self.rooms.entry(romm).and_modify(f)

        if self.rooms.contains_key(room) {
            if self.rooms[room].devices.contains(device) {
                Result::Err(Errors::DeviceExist)
            } else {
                let mut t = &self.rooms[room].devices;
                let x = Pin::new(&mut t).get_mut();
                x.insert(device.to_string());
                Result::Ok(true)
            }
            
        } else {
            Result::Err(Errors::RoomNotFound)
        }    
    }

    /// Returns devices in required room of the house
    /// ```
    /// use hw4::smart_house_dir::smart_house;
    /// let house = smart_house::SmartHouse::new();
    /// let devices = house.devices("Room B");
    /// let comparison = (devices == ["Thermo 1", "Socket 2", ""]) || (devices == ["Socket 2", "Thermo 1", ""]);
    /// assert!(comparison)
    /// ```
    pub fn get_devices(&self, room: &str) -> Result<Vec<&str>, Errors> {
        if self.rooms.contains_key(room) {
            let mut devices: Vec<&str> = Vec::with_capacity(10);
            for dev_name in &self.rooms[room].devices {
                devices.push(dev_name);
            }
            Result::Ok(devices)
        } else {
            Result::Err(Errors::RoomNotFound)
        }
    }

/*
    /// Returns report, using user's info provider about devices state
    /// ```
    /// use hw4::smart_house_dir::smart_house;
    /// pub struct SmartSocket {
    ///     pub name: String,
    ///     pub state: String
    /// }
    /// pub struct OwningDeviceInfoProvider {
    ///     socket: SmartSocket,
    /// }
    /// impl smart_house::DeviceInfoProvider for OwningDeviceInfoProvider {
    ///     fn get_device_info(&self, room: &str, name: &str) -> String{
    ///         let info: String;
    ///         if self.socket.name == name {
    ///             info = format!("room: {}, device: {}, state: {}", room, self.socket.name, self.socket.state);
    ///
    ///         } else {
    ///             info = format!("room: {}, device: {}, not found", room, self.socket.name);
    ///         }
    ///         info
    ///     }
    /// }
    /// let house = smart_house::SmartHouse::new();
    /// let socket1 = SmartSocket {name: smart_house::NAME_DEV_1.to_string(), state: String::from("working")};
    /// let info_provider = OwningDeviceInfoProvider{socket: socket1};
    /// let report = house.create_report(&info_provider);
    /// let keywords = ["Room A", "Room B", "Socket 1", "state", "not found"];
    /// let mut result = true;
    /// for word in keywords {
    /// if !report.contains(word) {
    /// result = false;
    /// }
    /// }
    /// assert!(result);
    pub fn create_report(&self, informer: &impl DeviceInfoProvider) -> String {
        let mut report = String::from("Report: \n");
        for (room, val) in &self.rooms {
            for device in &val.devices {
                let info = informer.get_device_info(room, device);
                report.push_str(&info);
                report.push('\n');
            }
        }
        report
    }
*/
}

pub trait DeviceInfoProvider {
    fn get_device_info(&self, room: &str, name: &str) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let name = "My house";
        let house = SmartHouse::new(name);
        assert_eq!(house.name, name);
    }

    #[test]
    fn test_add_room() {
        let mut house = SmartHouse::new("My house");
        house.add_room("Room A");
        let all_rooms = house.get_rooms();
        assert_eq!("Room A", all_rooms[0]);
    }

    #[test]
    fn test_remove_room() {
        let mut house = SmartHouse::new("My house");
        house.add_room("Room A");
        house.remove_room("Room A");
        let all_rooms = house.get_rooms();
        assert_eq!(all_rooms.len(), 0);
    }
    // #[test]
    // fn test_rooms() {
    //     let house = SmartHouse::new();
    //     let rooms = house.get_rooms();
    //     let comparison = (rooms == ["Room A", "Room B"]) || (rooms == ["Room B", "Room A"]);
    //     assert!(comparison)
    // }

    // #[test]
    // fn test_devices() {
    //     let house = SmartHouse::new();
    //     let devices = house.devices("Room B");
    //     let comparison =
    //         (devices == ["Thermo 1", "Socket 2", ""]) || (devices == ["Socket 2", "Thermo 1", ""]);
    //     assert!(comparison)
    // }

    // pub struct SmartSocket {
    //     pub name: String,
    //     pub state: String,
    // }
    // pub struct OwningDeviceInfoProvider {
    //     socket: SmartSocket,
    // }
    // impl DeviceInfoProvider for OwningDeviceInfoProvider {
    //     fn get_device_info(&self, room: &str, name: &str) -> String {
    //         let info: String = if self.socket.name == name {
    //             format!(
    //                 "room: {}, device: {}, state: {}",
    //                 room, self.socket.name, self.socket.state
    //             )
    //         } else {
    //             format!("room: {}, device: {}, not found", room, self.socket.name)
    //         };
    //         info
    //     }
    // }

    // #[test]
    // fn test_report() {
    //     let house = SmartHouse::new();
    //     let socket1 = SmartSocket {
    //         name: NAME_DEV_1.to_string(),
    //         state: String::from("working"),
    //     };
    //     let info_provider = OwningDeviceInfoProvider { socket: socket1 };
    //     let report = house.create_report(&info_provider);
    //     let keywords = ["Room A", "Room B", "Socket 1", "state", "not found"];
    //     let mut result = true;
    //     for word in keywords {
    //         if !report.contains(word) {
    //             result = false;
    //         }
    //     }
    //     assert!(result);
    // }
}
