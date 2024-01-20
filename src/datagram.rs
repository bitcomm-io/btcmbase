use bitflags::bitflags;
// use bytes::BytesMut;
use getset::{Getters,Setters,CopyGetters};
// use std::cell::RefCell;
use std::error;
use std::fmt;
// use std::rc::Rc;
use std::sync::Arc;
use bytes::Bytes;
use crate::client::ClientID;
use crate::client::ClientType;

bitflags!{
    #[repr(C)]   // 与C语言兼容
    #[derive(Debug,Clone,Copy,PartialEq,Eq)]
    pub struct ReturnCode:u32 {
        const RETURN_OK     =   0x00000000;
        const RETURN_ERROR1 =   0x00000001;
    }
}

bitflags!{
    #[repr(C)]   // 与C语言兼容
    #[derive(Debug,Clone,Copy,PartialEq,Eq)]
    pub struct BitcommFlag:u32 {
        const BITCOMM_MESSAGE    =   0x4D435442;
        const BITCOMM_COMMAND    =   0x43435442;
        const BITCOMM_NODEF      =   0xFFFFFFFF;
    }
}

bitflags!{
    #[repr(C)]   // 与C语言兼容
    #[derive(Debug,Clone,Copy,PartialEq,Eq)]
    pub struct BitcommVersion:u32 {
        const BITCOMM_VERSION_0_1_0_1  =  0x00010001; // 0.1.0.1
    }
}

bitflags!{
    #[repr(C)]   // 与C语言兼容
    #[derive(Debug,Clone,Copy,PartialEq,Eq)]
    pub struct BitCommand:u32 {
        const LOGIN_COMMAND  = 0x00000001;  // 登录命令
        const LOGOUT_COMMAND = 0x00000002;  // 登出命令
        const RESP_MASK      = 0x80000000;
        const SEND_MESS      = 0x00000004;
    }
}
bitflags!{
    #[repr(C)]   // 与C语言兼容
    #[derive(Debug,Clone,Copy)]
    pub struct MessageType:u32 {
        const MESS_TEXT    = 0x00000001;
        const MESS_IMAGES  = 0x00000002;
        const MESS_VIDEO   = 0x00000003;
        const MESS_FILES   = 0x00000004;
        const MESS_POSTION = 0x00000005;
    }
}




fn get_gram_by_u8<'a,T>(grambuf: &[u8]) -> &'a T {
    // 将字节切片转换为对 T 类型的引用
    unsafe {& *(grambuf[0..].as_ptr() as *const T)}
    // unsafe { & *(grambuf.as_ptr() as *const T) }
}
fn get_mut_gram_by_u8<'a,T>(grambuf: &mut [u8]) -> &'a mut T {
    // 将字节切片转换为对 T 类型的引用
    unsafe {&mut *(grambuf[0..].as_mut_ptr() as *mut T)}
    // unsafe { & *(grambuf.as_mut_ptr() as *mut T) }
}

fn size_of_align_data_gram<T>() -> usize {
    let size = std::mem::size_of::<T>();
    let align = std::mem::align_of::<T>();
    
    // 计算需要的补偿字节数
    let padding = (align - (size % align)) % align;
    
    // 计算最终大小
    size + padding
}

#[repr(C)]   // 与C语言兼容
#[derive(Debug,Clone,Copy,CopyGetters, Setters)]
pub struct MessageDataGram {
    #[getset(set = "pub", get_copy = "pub")]
    bitcomm      : BitcommFlag, 
    #[getset(set = "pub", get_copy = "pub")]
    version      : BitcommVersion, 
    #[getset(set = "pub", get_copy = "pub")]
    command      : BitCommand, 
    
    // #[getset(set = "pub", get_copy = "pub")]
    // splanet      : ClientPlanet, 
    #[getset(set = "pub", get_copy = "pub")]
    sender       : ClientID, 
    #[getset(set = "pub", get_copy = "pub")]
    sendertype   : ClientType, 
    #[getset(set = "pub", get_copy = "pub")]
    messagetype  : MessageType, 
    #[getset(set = "pub", get_copy = "pub")]
    messageid    : u32, 
    #[getset(set = "pub", get_copy = "pub")]
    refmessageid : u32,

    // #[getset(set = "pub", get_copy = "pub")]
    // rplanet      : ClientPlanet, 
    #[getset(set = "pub", get_copy = "pub")]
    receiver     : ClientID,  
    #[getset(set = "pub", get_copy = "pub")]
    receivertype : ClientType, 
    
    reserve1     : u64, 
    reserve2     : u64, 
    reserve3     : u32, 
    reserve4     : u32, 
    reserve5     : u32, 

    #[getset(set = "pub", get_copy = "pub")]
    datasize     : u32, 
}



impl MessageDataGram {
    // pub const BITCOMM_MESSAGE : u32  = 0x4D435442; // BTCM // 消息报文

    pub fn get_size() -> usize {
        size_of_align_data_gram::<Self>()
    }
    pub fn is_message_from_bytes(datas:&[u8]) -> bool {
        if datas.len() >= Self::get_size() {
            Self::is_message(datas)
        } else {
            false
        }
    }
    fn is_message(datas:&[u8]) -> bool {
        let bytes: [u8; 4] = [datas[0],datas[1],datas[2],datas[3]];//[b0, b1, b2, b3];
        // 将字节数组转换为u32
        let value = u32::from_le_bytes(bytes);
        value == BitcommFlag::BITCOMM_MESSAGE.bits()
    }

    pub fn create_gram_buf<'a>(datasize: usize) -> Vec<u8> {
        // 创建一个指定大小的 Vec<u8>
        #[allow(unused_mut)]
        let mut vec_u8: Vec<u8> = vec![0x00; datasize + size_of_align_data_gram::<MessageDataGram>()];
        vec_u8
    }
    pub fn create_message_data_gram_by_mut_vec8<'a>(byte_array: &'a mut Vec<u8>) -> &'a mut MessageDataGram {
        let grambuf: &mut[u8] = byte_array.as_mut_slice();
        MessageDataGram::create_message_data_gram_by_mut_u8(grambuf)
    }
    pub fn create_message_data_gram_by_mut_u8<'a>(grambuf:&'a mut[u8]) -> &'a mut MessageDataGram {
        #[allow(unused_mut)]
        let mut data_gram_ref: &mut MessageDataGram = get_mut_gram_by_u8::<MessageDataGram>(grambuf);//unsafe {&mut *(grambuf[0..].as_mut_ptr() as *mut DataGram)};
        // 设置结构体下,缓冲区的大小
        data_gram_ref.set_bitcomm(BitcommFlag::BITCOMM_MESSAGE);
        data_gram_ref.set_version(BitcommVersion::BITCOMM_VERSION_0_1_0_1);
        data_gram_ref.datasize = (grambuf.len() - size_of_align_data_gram::<MessageDataGram>()) as u32;
        data_gram_ref       
    }
    pub fn get_message_data_gram_by_u8<'a>(grambuf:&'a [u8]) -> &'a MessageDataGram {
        #[allow(unused_mut)]
        let data_gram_ref: &MessageDataGram = get_gram_by_u8::<MessageDataGram>(grambuf);//unsafe {& *(grambuf[0..].as_ptr() as *const DataGram)};
        data_gram_ref       
    }
    pub fn create_message_data_gram_from_mdg_u8<'a>(grambuf:&'a mut[u8],value:&'a MessageDataGram) -> &'a mut MessageDataGram {
        #[allow(unused_mut)]
        let mut message: &mut MessageDataGram = get_mut_gram_by_u8::<MessageDataGram>(grambuf);//unsafe {&mut *(grambuf[0..].as_mut_ptr() as *mut DataGram)};
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                message,
                1, // 1 表示复制一个元素
            );
        }
        message.set_bitcomm(value.bitcomm());
        message.set_command(value.command() | BitCommand::RESP_MASK);
        message.set_datasize(0);

        message
    }
}

#[repr(C)]   // 与C语言兼容
#[derive(Debug,Clone,Copy,CopyGetters, Setters)]
pub struct CommandDataGram {
    #[getset(set = "pub", get_copy = "pub")]
    bitcomm      : BitcommFlag, 
    #[getset(set = "pub", get_copy = "pub")]
    version      : BitcommVersion, 
    #[getset(set = "pub", get_copy = "pub")]
    command      : BitCommand,
    #[getset(set = "pub", get_copy = "pub")]
    deviceid     : u64, 
    #[getset(set = "pub", get_copy = "pub")]
    sender       : ClientID, 
    #[getset(set = "pub", get_copy = "pub")]
    sendertype   : ClientType, 
    #[getset(set = "pub", get_copy = "pub")]
    messagetype  : MessageType, 
    #[getset(set = "pub", get_copy = "pub")]
    messageid    : u32, 
    #[getset(set = "pub", get_copy = "pub")]
    returncode   : ReturnCode,
    #[getset(set = "pub", get_copy = "pub")]
    datasize     : u32, 
}

impl CommandDataGram {
    // pub const BITCOMM_COMMAND : u32  = 0x43435442; // BTCC // 命令报文

    pub fn is_command_from_bytes(datas:&[u8]) -> bool {
        if datas.len() == Self::get_size() {
            Self::is_command(datas)
        } else {
            false
        }
    }
    fn is_command(datas:&[u8]) -> bool {
        let bytes: [u8; 4] = [datas[0],datas[1],datas[2],datas[3]];//[b0, b1, b2, b3];
        // 将字节数组转换为u32
        let value = u32::from_le_bytes(bytes);
        value == BitcommFlag::BITCOMM_COMMAND.bits()
    }

    pub fn get_command_data_gram_by_u8<'a>(grambuf:&'a [u8]) -> &'a CommandDataGram {
        #[allow(unused_mut)]
        let data_gramhead_ref: &CommandDataGram = get_gram_by_u8::<CommandDataGram>(grambuf);//unsafe {& *(grambuf[0..].as_ptr() as *const DataGramHead)};
        // 设置结构体下,缓冲区的大小
        // data_gram_ref.datasize = (grambuf.len() - size_of_align_data_gram()) as u32;
        data_gramhead_ref       
    }
    pub fn create_command_data_gram_by_mut_vec8<'a>(byte_array: &'a mut Vec<u8>) -> &'a mut CommandDataGram {
        let grambuf: &mut[u8] = byte_array.as_mut_slice();
        Self::create_command_data_gram_by_mut_u8(grambuf)
    }
    pub fn create_command_data_gram_by_mut_u8<'a>(grambuf:&'a mut[u8]) -> &'a mut CommandDataGram {
        #[allow(unused_mut)]
        let mut data_gram_ref: &mut CommandDataGram = get_mut_gram_by_u8::<CommandDataGram>(grambuf);//unsafe {&mut *(grambuf[0..].as_mut_ptr() as *mut DataGram)};
        // 设置结构体标志
        data_gram_ref.datasize = (grambuf.len() - size_of_align_data_gram::<CommandDataGram>()) as u32;
        data_gram_ref.set_bitcomm(BitcommFlag::BITCOMM_COMMAND);
        data_gram_ref.set_version(BitcommVersion::BITCOMM_VERSION_0_1_0_1);
        data_gram_ref       
    }
    // pub fn get_command_data_gram_by_bytes<'a>(data:&'a bytes::Bytes) -> &'a CommandDataGram {
        // let slice = data.as_ref();
        // CommandDataGram::get_command_data_gram_by_u8(slice)
    // }
    pub fn get_size() -> usize {
        size_of_align_data_gram::<Self>()
    }
    // pub fn create_gram_buf<'a>() -> Vec<u8> {
    //     // 创建一个指定大小的 Vec<u8>
    //     #[allow(unused_mut)]
    //     let mut vec_u8: Vec<u8> = vec![0x00; size_of_align_data_gram::<CommandDataGram>()];
    //     vec_u8
    // }
    pub fn create_gram_buf<'a>(datasize: usize) -> Vec<u8> {
        // 创建一个指定大小的 Vec<u8>
        #[allow(unused_mut)] 
        let mut vec_u8: Vec<u8> = vec![0x00; datasize + size_of_align_data_gram::<CommandDataGram>()];
        vec_u8
    }
    pub fn create_command_gram_from_gram<'a>(buf:&'a mut [u8],value:&'a CommandDataGram) -> &'a mut CommandDataGram {
        let command = Self::create_command_data_gram_by_mut_u8(buf);
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                command,
                1, // 1 表示复制一个元素
            );
        }
        command.set_bitcomm(value.bitcomm());
        command.set_command(value.command() | BitCommand::RESP_MASK);
        command
    }
}

#[derive(Debug)]
pub enum InnerDataGram {
    Command {reqcmdbuff:Arc<Bytes>,reqcmdgram:Arc<CommandDataGram>},//rescmdbuff:RefCell<BytesMut>,rescmdgram:RefCell<CommandDataGram>},
    Message {reqmsgbuff:Arc<Bytes>,reqmsggram:Arc<MessageDataGram>} //,resmsgbuff:RefCell<BytesMut>,resmsggram:RefCell<MessageDataGram>},
}

#[derive(Debug,Getters, Setters)]
pub struct DataGramError {
    #[getset(set = "pub", get = "pub")]
    errcode: u32,
    #[getset(set = "pub", get = "pub")]
    details: String,
}

impl DataGramError {
    pub fn new(code:u32,tails:&str) -> Self {
        DataGramError{errcode:code,details:String::from(tails)}
    }
}

// 为自定义错误类型实现 Display 和 Error trait
impl fmt::Display for DataGramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for DataGramError {
    fn description(&self) -> &str {
        &self.details
    }
}