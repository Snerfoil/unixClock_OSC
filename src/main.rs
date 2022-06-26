extern crate rosc;

use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;
//use std::{env, f32, thread};

fn get_addr_from_arg(arg: &str) -> SocketAddrV4 {
    SocketAddrV4::from_str(arg).unwrap()
}

fn send_float_to(x: f32, y: &String) {
	//setup the socket
	let host_addr = get_addr_from_arg("127.0.0.1:9001");
    let to_addr = get_addr_from_arg("127.0.0.1:9000");
	let sock = UdpSocket::bind(host_addr).unwrap();
	
	// build the msg payload
    let addr_seg = format!("/avatar/parameters/{}", y);
	let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
        addr: addr_seg.to_string(),
        args: vec![OscType::Float(x)], //fill value here
    }))
    .unwrap();
	
	//send msg
	sock.send_to(&msg_buf, to_addr).unwrap();
}
/*
fn get_unix_seg(a: u32,b: u32) -> f32{
	//converts this section of unix time to a float percentage.
	let unix_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
	let bott = (1_u64).overflowing_shl(b+1).0;
	let top = ((unix_time>>a)%bott) as f32;
	top/(bott as f32)
	//this doesnt like overflow. odd, normally that works
	//also i'd imagine there is a better way to do this.
	//BUG: I dont think this works correctly, see note.txt
}
*/

fn get_unix_seg2(a: u64,b: u64) -> f32{
	//converts this section of unix time to a float percentage.
	let unix_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
	let top = ((unix_time>>a)%b) as f32;
	top/(b as f32)
	//this doesnt like overflow. odd, normally that works
	//also i'd imagine there is a better way to do this.
	//BUG: I dont think this works correctly, see note.txt
}

fn main() {
    println!("running clock, turn on VRC's OSC to get clock data.");
	loop{
		let params_osc = [
			(get_unix_seg2( 0,64),"ringSec"),
			(get_unix_seg2( 6,64),"ringMin"),
			(get_unix_seg2(12,16),"ringHr"),
			(get_unix_seg2(16, 8),"ringDay"),
			(get_unix_seg2(19, 4),"ringWeek"),
			(get_unix_seg2(21,16),"ringMonth"),
			(get_unix_seg2(25, 8),"ringYear"),
			(get_unix_seg2(28,16),"ringDecade"),
			(get_unix_seg2(32, 8),"ringCentury"),
			(get_unix_seg2( 0,9223372036854775807),"ringUnix")
		];
		
		for i in params_osc{
			send_float_to(i.0,&i.1.to_string());
		}
		thread::sleep(Duration::from_millis(1000));
	}
}

//turns out blasting either a 0.0 or 1.0, and having it be wack is just a quirk of VRC not OSC. but you can safely ignore this, as its a problem with the debugger UI.
