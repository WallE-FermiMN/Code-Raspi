use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::time::{Duration, Instant};
use std::ops::Add;

/// Ease PWM Command
pub struct EaseServoCommand {
    pub time: Duration,
    pub channel: u8,
    pub val: u16
}

/// Ease Speed + direction command
pub struct EaseDCCommand{
    pub time: Duration,
    pub values: (i16, i16)
}


/// This enum contains all the possible commands
/// The ShutdownThreads command is reserved for program shutdown propagation.
pub enum Command{
    EaseServo(EaseServoCommand),
    EaseDC(EaseDCCommand),
    ClockSync,
    Startup,
    ShutdownThreads
}

impl Command{
    fn into_vec_u8(&self, starting_time: Instant) -> Vec<u8> {
        match self{
            Command::EaseServo(c) => {
                let mut v: Vec<u8> = Vec::with_capacity(8);
                v.push(0xD2);
                v.extend_from_slice(&(starting_time.elapsed().add(c.time).as_millis() as u32).to_le_bytes());
                v.push(c.channel);
                v.extend_from_slice(&c.val.to_le_bytes());
                v
            }
            Command::EaseDC(c) => {
                let mut v: Vec<u8> = Vec::with_capacity(9);
                v.push(0x87);
                v.extend_from_slice(&(starting_time.elapsed().add(c.time).as_millis() as u32).to_le_bytes());
                v.extend_from_slice(&c.values.0.to_le_bytes());
                v.extend_from_slice(&c.values.1.to_le_bytes());
                v
            }
            Command::ClockSync => {
                let mut v: Vec<u8> = Vec::with_capacity(5);
                v.push(0x4B);
                v.extend_from_slice(&(starting_time.elapsed().as_millis() as u32).to_le_bytes());
                v
            }
            Command::Startup => {
                let mut v: Vec<u8> = Vec::with_capacity(1);
                v.push(0xCC);
                v
            }
            Command::ShutdownThreads => {
                vec![]
            }
        }
    }
}


/// Initializes the packet sending service. Needs a sender (to receive packets)
/// and a receiver (cloned) to send time sync packets.
/// This function receives a packet stream, processing it every 50ms
pub fn init(rec: Receiver<Command>, snd: Sender<Command>){
    log::trace!("SerialComThread - Spawning clock thread");
    std::thread::spawn(  move|| clock_sync_thread(snd.clone()));
    log::trace!("SerialComThread - Initializing clock");
    let starting_time = Instant::now();
    loop {
        match rec.try_recv(){
            Ok(packet) => {
                log::trace!("SerialComThread - Sending packet");
                send_packet(&packet, false, starting_time);
                if let Command::ShutdownThreads = packet {
                    log::trace!("SerialComThread - All senders disconnected, terminating...");
                    return;
                }
            }
            Err(e) => {
                if let TryRecvError::Disconnected = e {
                    log::trace!("SerialComThread - All senders disconnected, terminating...");
                    return;
                }
            }
        }
        for x in rec.try_iter() {
            log::trace!("SerialComThread - Sending back to back packet");
            send_packet(&x, true, starting_time);
            if let Command::ShutdownThreads = x {
                log::trace!("SerialComThread - All senders disconnected, terminating...");
                return;
            }
        }
        log::trace!("SerialComThread - Sleeping...");
        std::thread::sleep(Duration::from_millis(50));
    }
}

// Send a vector of bytes (data) to the serial (adds CRC8 etc...)
// The first element in the vector is the command data, and
// only the least significant 4 bits are kept.
fn send_packet(cmd: &Command, back_to_back: bool, starting_time: Instant){
    if let Command::ShutdownThreads = cmd {
        log::trace!("SerialComThread.send_packet - A program shutdown request was issued");
        return;
    }
    let mut raw_data: Vec<u8> = Vec::new();
    let pack = cmd.into_vec_u8(starting_time);
    if !back_to_back { raw_data.push(0x00);}

    for i in &pack[..] {
       match i {
           0x00 => {raw_data.push(0xFF);raw_data.push(0xEE);}
           0xFF => {raw_data.push(0xFF);raw_data.push(0xDD);}
           n => {raw_data.push(*n);}
       }
    }
    raw_data.push(create_crc8(&pack));
    raw_data.push(0x00);
    send_raw(raw_data);
}

// Takes a vector of bytes and sends to the serial port.
fn send_raw(data: Vec<u8>){
    unimplemented!();
}
fn create_crc8(data: &Vec<u8>) -> u8{
    let mut crc: u8 = 0;
    let mut inbyte: u8;
    let mut mix: u8 ;
    for item in data {
        inbyte = *item;
        for _ in 0..8 {
            mix = (crc ^ inbyte) & 0x01;
            crc >>= 1;
            if mix != 0 {crc ^= 0x8C;}
            inbyte >>= 1;
        }
    }
    return crc;
}
fn clock_sync_thread(snd: Sender<Command>){
    for _ in 0..5 {
        log::trace!("ClockSyncThread - Sending startup command");
        snd.send(Command::Startup);
    }
    for _ in 0..5{
        log::trace!("ClockSyncThread - Sending ClockSync command");
        snd.send(Command::ClockSync);
        std::thread::sleep(Duration::from_millis(25));
    }
    loop{
        log::trace!("ClockSyncThread - Sending ClockSync command");
        match snd.send(Command::ClockSync){
            Ok(_) => {}
            Err(_) => {
                log::info!("ClockSyncThread - Channel cut, shutting down...");
                return;
            }
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}