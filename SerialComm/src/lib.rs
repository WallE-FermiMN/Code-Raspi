use std::sync::mpsc::Receiver;
// Priority 0 are Startup, Shutdown and Stop commands
// all other commands are priority 1.
fn init(rec_1: Receiver<Vec<u8>>, rec_0: Receiver<Vec<u8>>){

}
fn send_packet(pack: Vec<u8>){
    unimplemented!();


}