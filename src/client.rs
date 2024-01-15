use bitflags::bitflags;
use getset::{Setters,CopyGetters};

#[repr(C)]   // 与C语言兼容
#[derive(Debug,Clone,Copy,CopyGetters, Setters)]
pub struct ClientID {
    #[getset(set = "pub", get_copy = "pub")]
    planet   :   ClientPlanet,
    #[getset(set = "pub", get_copy = "pub")]
    object   :   u32,
}
impl Into<u64> for ClientID {
    fn into(self) -> u64 {
        ((self.planet.bits() as u64) << 32) | (self.object as u64)
    }
}
impl ClientID {
    pub fn new(planet: ClientPlanet, object: u32) -> Self { Self { planet, object } }
    pub fn get_guid(&self) -> u64 {
        ((self.planet.bits() as u64) << 32) | (self.object as u64)
    }
    pub fn get_hex(&self) -> String {
        format!("{:x}", self.get_guid())
    }
}

bitflags!{
    #[repr(C)]   // 与C语言兼容
    #[derive(Debug,Clone,Copy)]
    pub struct ClientType:u32 {
        const CLIENT_PEOPLE    = 0x0001;
        const CLIENT_GROUP     = 0x0002;
        const CLIENT_DEVICE    = 0x0003;
        const CLIENT_ROBOT     = 0x0004;
    }
}
bitflags!{
    #[repr(C)]   // 与C语言兼容
    #[derive(Debug,Clone,Copy)]
    pub struct ClientPlanet:u32 {
        const PLANET_EARTH    = 0x00010001;
        const PLANET_MAR      = 0x00010002;
    }
}
