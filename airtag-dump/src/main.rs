use std::time::Duration;
use std::thread;
use std::fs::File;
use std::io::Write;
use std::process;

use pwn::tubes::remote::Remote;
use pwn::tubes::tube::Tube;
use pwn::util::packing::p32;

use probe_rs::Session;
use probe_rs::MemoryInterface;

fn dump() -> Result<(), probe_rs::Error> {
    let mut session = Session::auto_attach("nrf52")?;
    let mut core = session.core(0)?;

    println!("Attempting read the firmware...");

    let mut buf = [0u8; 0x210];
    core.read_8(0x10001000, &mut buf)?;

    let mut file = File::create("airtag.uicr.bin").unwrap();
    file.write_all(&buf).unwrap();

    let mut buf = [0u8; 0x80000];
    core.read_8(0x0, &mut buf)?;
 
    let mut file = File::create("airtag.flash.bin").unwrap();
    file.write_all(&buf).unwrap();

    Ok(())
}


fn main() {

    let mut sock = Remote::remote("10.168.1.85", 6666);
    sock.clean(None);

    loop {
        for glitch in 1..500 {
            println!("Width: {:}", glitch);

            let delay = 1150500; 

            let mut cmd = b"l".repeat(1);
            sock.send(cmd);
            thread::sleep(Duration::from_millis(500));

            let mut cmd = b"d".repeat(1);
            cmd.append(&mut p32(delay).unwrap());
            sock.recvuntil(b"Y"); sock.send(cmd);
            
            let mut cmd = b"p".repeat(1);;
            cmd.append(&mut p32(glitch).unwrap());
            sock.recvuntil(b"Y"); sock.send(cmd);

            let mut cmd = b"g".repeat(1);;
            sock.recvuntil(b"Y"); sock.send(cmd);
            
            sock.recvuntil(b"Y");
            thread::sleep(Duration::from_millis(50));

            match dump() {
                Ok(_) => {
                    println!("Dumped succesfully");
                    process::exit(0);
                },
                Err(e) => {
                    println!("Error dumping: {:?}", e);
                }
            }

        }
    }
}
