use libc;
use cpuio;
use std::env;
use std::time::Duration;

/// Direct I/O ports (CPU registers). Requires root privilegies to use this.
static IBF: u8 = 1;
static OBF: u8 = 0;
static EC_SC: u64 = 0x66;
static EC_DATA: u64 = 0x62;
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
    let fan_duty: u8 = args[1].trim().parse::<u8>().expect("Error: wrong arg");

    sysio_init(EC_DATA, EC_SC);
    set_fan_duty(fan_duty);
}

fn set_fan_duty(fan_duty: u8) {
    if fan_duty < 50 || fan_duty > 100 {
        panic!("Error: wrong arg duty: {}", fan_duty);
    }
    let value = (fan_duty as f32 / 100.0) * 255.0;

    sysio_write(0x99, 0x01, value as u8);
    println!("Change fan duty to: {}%", fan_duty);
}

fn get_cpu_temp() -> u8 {
    sysio_read(EC_SC as u16, EC_REG_CPU_TEMP as u16)
}
    
/// Init the cpu registers for r/w
fn sysio_init(first_port: u64, second_port: u64) {
    unsafe {
        assert_eq!(ioperm(first_port, 1, 1), 0, "Error: sysio_init can't r/w. Try to run as root");
        assert_eq!(ioperm(second_port, 1, 1), 0, "Error: sysio_init can't r/w. Try to run as root");
    }
}


/// Wait cpu registers be ready to r/w values
///
/// #Example
/// ```
/// let wait = sysio_wait(EC_SC, IBF, 0);
/// match wait {
///     Ok(()) => (),
///     Err(error) => panic!("{}", error),
/// };
/// ```
fn sysio_wait(port: u16, flag: u8, value: u8) -> Result<(), String> {
    let mut data: u8;

    unsafe {
        data = cpuio::inb(port);
    }

    for i in 0..102 {
        if ((data >> flag) & 0x1) != value {
            std::thread::sleep(Duration::from_micros(1000));
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

/// Read some values from registers.
/// E.g: cpu temp, gpu temp, fan rpm...
///
/// #Example
/// ```
/// sysio_read(EC_SC as u16, EC_REG_CPU_TEMP as u16);
/// ```
fn sysio_read(port: u16, read_port: u16) -> u8 {
    let mut wait = sysio_wait(port, IBF, 0);
    match wait {
        Ok(()) => unsafe { cpuio::outb(EC_SC_READ_CMD as u8, port); },
        Err(error) => panic!("{}", error),
    };

    wait = sysio_wait(port, IBF, 0);
    match wait {
        Ok(()) => unsafe { cpuio::outb(read_port as u8, EC_DATA as u16); },
        Err(error) => panic!("{}", error),
    };


    let value: u8;
    wait = sysio_wait(port, OBF, 1);
    match wait {
        Ok(()) => unsafe { value = cpuio::inb(EC_DATA as u16); },
        Err(error) => panic!("{}", error),
    }; 

    return value;
}

/// Write value in cpu register
///
/// #Example
/// Set fan duty to 65%
/// ```
/// sysio_write(0x99, 0x01, 165, EC_SC);
/// ```
fn sysio_write(cmd: u8, port: u8, value: u8) {
    let mut wait;
    wait = sysio_wait(EC_SC as u16, IBF, 0);
    match wait {
        Ok(()) => unsafe {
            let mut select = cpuio::UnsafePort::<u8>::new(EC_SC as u16);
            select.write(cmd);
        }
        Err(error) => panic!("{}", error),
    };
    wait = sysio_wait(EC_SC as u16, IBF, 0);
    match wait {
        Ok(()) => unsafe {
            let mut command = cpuio::UnsafePort::<u8>::new(EC_DATA as u16);
            command.write(port) 
        }, 
        Err(error) => panic!("{}", error),
    };

    wait = sysio_wait(EC_SC as u16, IBF, 0);
    match wait {
        Ok(()) => unsafe {
            let mut write_port = cpuio::UnsafePort::<u8>::new(EC_DATA as u16);
            write_port.write(value);
        },
        Err(error) => panic!("{}", error),
    };

    wait = sysio_wait(EC_SC as u16, IBF, 0);
    match wait {
        Ok(()) => (),
        Err(error) => panic!("{}", error),
    };
}
