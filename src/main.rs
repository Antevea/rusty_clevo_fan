use libc;
use cpuio;
use std::env;
use std::time::Duration;

/// Direct I/O ports. Requires root privilegies to use this.
static IBF: u8 = 1;
static OBF: u8 = 0;
static EC_DATA: u64 = 0x62;
static EC_SC: u64 = 0x66;
static EC_SC_READ_CMD: u64 = 0x80;
static EC_REG_SIZE: u64 = 0x100;
static EC_REG_CPU_TEMP: u64 = 0x07;
static EC_REG_GPU_TEMP: u64 = 0xCD;
static EC_TEG_FAN_DUTY: u64 = 0xCE;
static EC_REG_FAN_RPMS_HI: u64 = 0xD0;
static EC_REG_FAN_RPMS_LO: u64 = 0xD1;

static MAX_FAN_RPM: f32 = 4400.0;

extern "C" {
    pub fn ioperm(
        from: libc::c_ulong, 
        num: libc::c_ulong, 
        turn_on: libc::c_int
    ) -> libc::c_int;
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage: rusty_clevo_fan [fan_duty_percentage]");
    }

    sysio_init(EC_DATA, EC_SC);
    
    println!("CPU temp: {}", get_cpu_temp());
    println!("Success!");
}

fn get_cpu_temp() -> u8 {
    sysio_read(EC_SC as u16, EC_REG_CPU_TEMP as u16)
}
    
fn sysio_init(port_f: u64, port_s: u64) {
    unsafe {
        assert_eq!(ioperm(port_f, 1, 1), 0, "Error: sysio_init can't r/w. Try to run as root");
        assert_eq!(ioperm(port_s, 1, 1), 0, "Error: sysio_init can't r/w. Try to run as root");
    }
}

fn sysio_wait(port: u16, flag: u8, value: u8) -> Result<(), String> {
    let mut data: u8;

    unsafe {
        data = cpuio::inb(port);
    }

    for i in 0..102 {
        if ((data >> flag) & 0x1) != value {
            std::thread::sleep(Duration::from_millis(500));
            unsafe { data = cpuio::inb(port); }
        } else {
            break;
        }

        if i >= 100 {
           return Err(format!("Error: sysio_wait runtime exeption on port: {}, data: {}, flag: {}, value: {}",
               port, data, flag, value));
        }
    }

    Ok(())
}

fn sysio_read(port: u16, read_port: u16) -> u8 {
    let mut wait = sysio_wait(port, IBF, 0);
    match wait {
        Ok(()) => (),
        Err(error) => panic!("{}", error),
    };
    unsafe {
        cpuio::outb(EC_SC_READ_CMD as u8, port);
    }

    wait = sysio_wait(port, IBF, 0);
    match wait {
        Ok(()) => (),
        Err(error) => panic!("{}", error),
    };
    unsafe {
        cpuio::outb(read_port as u8, EC_DATA as u16);
    }

    wait = sysio_wait(port, OBF, 1);
    match wait {
        Ok(()) => (),
        Err(error) => panic!("{}", error),
    }
    let value: u8;
    unsafe {
        value = cpuio::inb(EC_DATA as u16);
    }

    return value;
}
