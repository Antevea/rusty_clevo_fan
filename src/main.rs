use std::env;
mod fan_control;

enum EParsedArgs {
    Help,
    Dump,
    Duty(u8),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage: rusty_clevo_fan [fan_duty_percentage]");
        return;
    }

    let parsed_arg = parse_args(args);
    match parsed_arg {
        EParsedArgs::Help => print_help(),
        EParsedArgs::Dump => {
            if let Err(err) = fan_control::fan_control_init() {
                println!("{}", err);
                return;
            }
            println!("Dump fan and cpu info:\n");
            println!("CPU temp: {}", fan_control::get_cpu_temp());
            println!("GPU temp: {}", fan_control::get_gpu_temp());
        },
        EParsedArgs::Duty(duty) => {
            if let Err(err) = fan_control::fan_control_init() {
                println!("{}", err);
                return;
            }
            fan_control::set_fan_duty(duty);
        },
    };
}

fn parse_args(args: Vec<String>) -> EParsedArgs {
    if args[1].contains("-h") {
        return EParsedArgs::Help;
    } else if args[1].contains("-d") {
        return EParsedArgs::Dump;
    } else {
        return EParsedArgs::Duty(args[1].trim()
            .parse::<u8>()
            .expect(&format!("Error: wrong argument: {}", args[1])));
    }
}

fn print_help() {
    println!("Fan control utility for Clevo laptops\n");
    println!("Usage: rusty_clevo_fan [fan_duty_percentage]");
    println!("Arguments:\n\t[fan_duty_percentage]\tTarget fan duty in percentage, from 40 to 100\n");
    println!("\t-g\t\t\tStart GUI\n");
    println!("\t-d\t\t\tDump fan and temp information\n");
    println!("\t-h\t\t\tPrint this help and exit");
    println!("To use without sudo:");
    println!("\tsudo chown root [path/to/rusty_clevo_fan]");
    println!("\tsudo chmod u+s [path/to/rusty_clevo_fan]");
    println!("DO NOT MANIPULATE OR QUERY EC I/O PORTS WHILE THIS PROGRAM IS RUNNING.");
}
