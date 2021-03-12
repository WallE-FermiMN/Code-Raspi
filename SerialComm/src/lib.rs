use std::sync::mpsc::Receiver;
// Priority 0 are Startup, Shutdown and Stop commands
// all other commands are priority 1.
fn init(rec_1: Receiver<Vec<u8>>, rec_0: Receiver<Vec<u8>>){

}

// Send a vector of bytes (data) to the serial (adds CRC8 etc...)
// The first element in the vector is the command data, and
// only the least significant 4 bits are kept.
pub fn send_packet(pack: Vec<u8>){
    let mut raw_data: Vec<u8> = Vec::new();
    raw_data.push(0x00);
    raw_data.push(create_hamming(pack[0]));
    for i in &pack[1..] {
       match i {
           0x00 => {raw_data.push(0xFF);raw_data.push(0xEE);}
           0xFF => {raw_data.push(0xFF);raw_data.push(0xDD);}
           n => {raw_data.push(*n);}
       }
    }
    raw_data.push(create_crc8(&pack));
    raw_data.push(0x00);
    send_raw(raw_data);
    unimplemented!();
}

// Takes a vector of bytes and sends to the serial port.
fn send_raw(data: Vec<u8>){
    unimplemented!();
}
fn create_hamming(com: u8) -> u8{
    unimplemented!();
    0
}
fn create_crc8(data: &Vec<u8>) -> u8{
    unimplemented!();
    0
}