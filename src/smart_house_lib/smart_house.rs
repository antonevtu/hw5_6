use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

#[derive(Default)]
pub struct SmartHouse {
    pub name: String,
    rooms: HashMap<String, Room>,
}

#[derive(Default)]
struct Room {
    devices: HashSet<String>,
}

#[derive(Debug)]
pub enum FindError {
    RoomNotFound,
    DeviceNotFound,
}

impl fmt::Display for FindError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "room or device not found")
    }
}

impl Error for FindError {}

pub trait DeviceInfoProvider {
    fn get_device_info(&self, room: &str, name: &str) -> String;
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

    /// Adding new room, if not present. If room present, nothing happens
    /// ```
    /// use hw5_6::smart_house_lib::smart_house;
    /// let mut house = smart_house::SmartHouse::new("My house");
    /// house.add_room("Room A");
    /// ```
    pub fn add_room(&mut self, name: &str) {
        self.rooms.entry(name.to_string()).or_default();
    }

    /// Delete room from house, if present. Returns None, if not present
    /// ```
    /// use hw5_6::smart_house_lib::smart_house;
    /// let mut house = smart_house::SmartHouse::new("My house");
    /// house.add_room("Room A");
    /// house.remove_room("Room A");
    /// ```
    pub fn remove_room(&mut self, name: &str) -> Option<bool> {
        let room = self.rooms.remove(name);
        // match room {
        //     Some(_) => Option::Some(true),
        //     None => Option::None,
        // }
        room.map(|_| true)
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

    /// Adding device to the room. If room not exists, returns error.
    /// If device exists, nothing happens
    /// ```
    /// use hw5_6::smart_house_lib::smart_house;
    /// let mut house = smart_house::SmartHouse::new("My house");
    /// house.add_room("Room A");
    /// let res = house.add_device("Room A", "Socket 1");
    /// assert_eq!(res.unwrap(), true);
    /// ```
    pub fn add_device(&mut self, room_name: &str, device: &str) -> Result<bool, FindError> {
        let opt = self.rooms.remove(room_name);
        let mut room = match opt {
            Some(s) => s,
            None => return Result::Err(FindError::RoomNotFound),
        };
        room.devices.insert(device.to_string());
        self.rooms.insert(room_name.to_string(), room);
        Result::Ok(true)
    }

    /// Remove device. If device or room not present, returns error
    /// ```
    /// use hw5_6::smart_house_lib::smart_house;
    /// let mut house = smart_house::SmartHouse::new("My house");
    /// house.add_room("Room A");
    /// let res = house.add_device("Room A", "Socket 1");
    /// res.unwrap();
    /// let res1 = house.remove_device("Room A", "Socket 1");
    /// ```
    pub fn remove_device(&mut self, room_name: &str, device: &str) -> Result<bool, FindError> {
        let opt = self.rooms.get_mut(room_name);
        let room = match opt {
            Some(s) => s,
            None => return Result::Err(FindError::RoomNotFound),
        };
        room.devices.insert(String::from("xxx"));  // попробовать 
        let ok = room.devices.remove(device);
        if ok {
            Result::Ok(true)
        } else {
            Result::Err(FindError::DeviceNotFound)
        }
    }

    /// Returns devices in required room of the house
    /// ```
    /// use hw5_6::smart_house_lib::smart_house;
    /// let mut house = smart_house::SmartHouse::new("My house");
    /// house.add_room("Room A");
    /// house.add_device("Room A", "Socket 1");
    /// house.add_device("Room A", "Thermometer 1");
    /// ```
    pub fn get_devices(&self, room: &str) -> Result<Vec<&str>, FindError> {
        if self.rooms.contains_key(room) {
            let mut devices: Vec<&str> = Vec::with_capacity(10);
            for dev_name in &self.rooms[room].devices {
                devices.push(dev_name);
            }
            Result::Ok(devices)
        } else {
            Result::Err(FindError::RoomNotFound)
        }
    }

    /// Returns report, using user's info provider about devices state
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
        house.remove_room("Room A").unwrap();
        let all_rooms = house.get_rooms();
        assert_eq!(all_rooms.len(), 0);
    }

    #[test]
    fn test_add_device() {
        let mut house = SmartHouse::new("My house");
        house.add_room("Room A");
        house.add_device("Room A", "Socket 1").unwrap();
    }

    #[test]
    #[should_panic(expected = "RoomNotFound")]
    fn test_add_device_negative() {
        let mut house = SmartHouse::new("My house");
        house.add_room("Room A");
        house.add_device("Room B", "Socket 1").unwrap();
    }

    #[test]
    fn test_remove_device() {
        let mut house = SmartHouse::new("My house");
        house.add_room("Room A");
        house.add_device("Room A", "Socket 1").unwrap();
        house.remove_device("Room A", "Socket 1").unwrap();
    }

    #[test]
    #[should_panic(expected = "RoomNotFound")]
    fn test_remove_device_negative() {
        let mut house = SmartHouse::new("My house");
        house.add_room("Room A");
        house.add_device("Room A", "Socket 1").unwrap();
        house.remove_device("Room B", "Socket 1").unwrap();
    }

    #[test]
    #[should_panic(expected = "DeviceNotFound")]
    fn test_remove_device_negative_2() {
        let mut house = SmartHouse::new("My house");
        house.add_room("Room A");
        let res = house.add_device("Room A", "Socket 1");
        res.unwrap();
        let res1 = house.remove_device("Room A", "Socket 2");
        res1.unwrap();
    }

    #[test]
    fn test_get_devices() {
        let mut house = SmartHouse::new("My house");
        house.add_room("Room A");
        house.add_device("Room A", "Socket 1").unwrap();
        house.add_device("Room A", "Thermometer 1").unwrap();
        let devices = house.get_devices("Room A").unwrap();
        let result = (devices == vec!["Socket 1", "Thermometer 1"])
            || (devices == vec!["Thermometer 1", "Socket 1"]);
        assert!(result);
    }

    pub struct SmartSocket {
        pub name: String,
        pub state: String,
    }
    pub struct OwningDeviceInfoProvider {
        socket: SmartSocket,
    }
    impl DeviceInfoProvider for OwningDeviceInfoProvider {
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

    #[test]
    fn test_report() {
        let mut house = SmartHouse::new("My house");
        house.add_room("Room A");
        house.add_device("Room A", "Socket 1").unwrap();
        let socket1 = SmartSocket {
            name: "Socket 1".to_string(),
            state: String::from("working"),
        };
        let info_provider = OwningDeviceInfoProvider { socket: socket1 };
        let report = house.create_report(&info_provider);
        let keywords = ["Room A", "Socket 1", "state"];
        let mut result = true;
        for word in keywords {
            if !report.contains(word) {
                result = false;
            }
        }
        assert!(result);
    }
}
