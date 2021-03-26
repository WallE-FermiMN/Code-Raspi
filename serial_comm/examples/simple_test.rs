use std::sync::mpsc;
use serial_comm::{Command, EaseDcCommand, EaseServoCommand};
use std::time::Duration;

fn main(){
    let (snd,rec) = mpsc::channel::<Command>();
    let snd2 = snd.clone();
    std::thread::sleep(Duration::from_millis(500));
    std::thread::spawn(move || serial_comm::init(rec,snd2));
    std::thread::sleep(Duration::from_millis(500));
    let c2 = serial_comm::Command::ClockSync;
    let c3 = serial_comm::Command::EaseDc(EaseDcCommand{ time: Duration::from_millis(345), values: (10, -20) });
    let c4 = serial_comm::Command::EaseServo(EaseServoCommand{
        time: Duration::from_millis(5232),
        channel: 12,
        val: 4000
    });
    let c5 = serial_comm::Command::ShutdownThreads;
    snd.send(c2);
    snd.send(c3);
    snd.send(c4);
    std::thread::sleep(Duration::from_millis(500));
    snd.send(c5);
    std::thread::sleep(Duration::from_millis(500));
}