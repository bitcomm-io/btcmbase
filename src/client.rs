use bitflags::bitflags;
use getset::{ Setters, CopyGetters };

#[repr(C)] // 与C语言兼容c
#[derive(Debug, Clone, Copy, CopyGetters, Setters,PartialEq,Eq)]
pub struct ClientID {
    #[getset(set = "pub", get_copy = "pub")]
    planet: ClientPlanet,
    #[getset(set = "pub", get_copy = "pub")]
    object: u32,
}
impl Into<u64> for ClientID {
    fn into(self) -> u64 {
        ((self.planet.bits() as u64) << 32) | (self.object as u64)
    }
}
impl ClientID {
    //
    pub fn new(planet: ClientPlanet, object: u32) -> Self {
        Self { planet, object }
    }
    //
    pub fn get_guid(&self) -> u64 {
        ((self.planet.bits() as u64) << 32) | (self.object as u64)
    }
    //
    pub fn get_hex(&self) -> String {
        format!("{:x}", self.get_guid())
    }
    //
    pub fn get_key(&self, deviceid: u32) -> u128 {
        let num_u64 = ((self.planet.bits() as u64) << 32) | (self.object as u64);
        ((num_u64 as u128) << 64) | (deviceid as u128)
    }
}

bitflags! {
    #[repr(C)]   // 与C语言兼容
    #[derive(Debug,Clone,Copy,PartialEq, Eq)]
    pub struct ClientType:u32 {
        const CLIENT_PEOPLE    = 0x0001;
        const CLIENT_GROUP     = 0x0002;
        const CLIENT_DEVICE    = 0x0003;
        const CLIENT_ROBOT     = 0x0004;
        const CLIENT_SERVICE   = 0x0010;
    }
}
bitflags! {
    #[repr(C)]   // 与C语言兼容
    #[derive(Debug,Clone,Copy,PartialEq,Eq)]
    pub struct ClientPlanet:u32 {
        const PLANET_EARTH    = 0x00000000;
        const PLANET_MAR      = 0x00000010;
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy, CopyGetters, Setters, PartialEq, Eq)]
pub struct DeviceConnInfo {
    #[getset(set = "pub", get_copy = "pub")]
    device_id: u32,
    #[getset(set = "pub", get_copy = "pub")]
    device_state: DeviceConnState,
}
impl DeviceConnInfo {
    pub fn new(device_id: u32, device_state: DeviceConnState) -> Self {
        Self { device_id, device_state }
    }
}
bitflags! {
    #[repr(C)]   // 与C语言兼容
    #[derive(Debug,Clone,Copy,PartialEq,Eq)]
    pub struct DeviceConnState:u32 {
        const STATE_ONLINE    = 0x00010001;
        const STATE_ONBACK    = 0x00010002;
        const STATE_OFFLINE   = 0x00010004;
    }
}
