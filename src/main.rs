#![windows_subsystem = "windows"]

use hidapi::{DeviceInfo, HidApi, HidDevice};

const VID: u16 = 0x1532;
const PID_WIRE: u16 = 0x00c2;
const PID_WIRELESS: u16 = 0x00c3;
const USAGE: u16 = 2;
const USAGEPAGE: u16 = 1;

fn get_device_info<'a>(
    hidapi: &'a HidApi,
    args: &'a AppArgs,
) -> Result<&'a DeviceInfo, &'static str> {
    for device in hidapi.device_list() {
        if device.vendor_id() != args.vid
            || (device.product_id() != args.pid_wire && device.product_id() != args.pid_wireless)
            || device.usage() != args.usage
            || device.usage_page() != args.usagepage
        {
            continue;
        }

        if cfg!(debug_assertions) {
            println!("path: {:?}", device.path());
            println!(
                "Bus {:03} Device {:03} Face {} ID {:04x}:{:04x}",
                device.usage(),
                device.usage_page(),
                device.interface_number(),
                device.vendor_id(),
                device.product_id()
            );

            println!("path: {:?}", device.path());
            println!("{:04x}:{:04x}", device.vendor_id(), device.product_id());
        }

        return Ok(device);
    }
    return Err("invalid version");
}

fn send_cmd(hid_result: &HidDevice, cmd: Vec<u8>, args: Vec<u8>, footer: u8) {
    // Buffer used to communicate
    let mut buf = Vec::with_capacity(256);

    // 1 // HID report number
    // + 1 // Status
    // + 4 // Padding
    // = 6
    let zeros = vec![0x00; 6];
    buf.extend(zeros);
    for cmd_byte in &cmd {
        buf.push(cmd_byte.to_le_bytes()[0]);
    }
    for arg_byte in &args {
        buf.push(arg_byte.to_le_bytes()[0]);
    }
    let zeros = vec![0x00; 89 - buf.len()];
    buf.extend(zeros);
    buf.push(footer.to_le_bytes()[0]);
    buf.push(0x00);

    #[cfg(debug_assertions)]
    println!("{:02X?}", &buf);

    hid_result.send_feature_report(&buf).unwrap();
}

#[derive(Debug)]
struct AppArgs {
    vid: u16,
    pid_wire: u16,
    pid_wireless: u16,
    usage: u16,
    usagepage: u16,

    dpi: Option<u16>,
    hz: Option<u16>,
    whz: Option<u16>,
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    fn parse_number(s: &str) -> Result<u16, &'static str> {
        s.parse().map_err(|_| "not a number")
    }

    let args = AppArgs {
        vid: pargs
            .opt_value_from_fn("--vid", parse_number)?
            .unwrap_or(VID),

        pid_wire: pargs
            .opt_value_from_fn("--pid_wire", parse_number)?
            .unwrap_or(PID_WIRE),

        pid_wireless: pargs
            .opt_value_from_fn("--pid_wireless", parse_number)?
            .unwrap_or(PID_WIRELESS),

        usage: pargs
            .opt_value_from_fn("--usage", parse_number)?
            .unwrap_or(USAGE),

        usagepage: pargs
            .opt_value_from_fn("--usagepage", parse_number)?
            .unwrap_or(USAGEPAGE),

        dpi: pargs.opt_value_from_str("--dpi")?,

        hz: pargs.opt_value_from_str("--hz")?,

        whz: pargs.opt_value_from_str("--whz")?,
    };

    Ok(args)
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            panic!("Error: {}.", e);
        }
    };

    let hidapi: HidApi = match HidApi::new() {
        Ok(api) => api,
        Err(e) => {
            panic!("Error: {}.", e);
        }
    };

    let device_info_path = match get_device_info(&hidapi, &args) {
        Ok(device_info) => device_info.path(),
        Err(e) => {
            panic!("Error: {}.", e);
        }
    };

    let hid_result = hidapi.open_path(device_info_path).unwrap();

    let mut errorcode: i32 = 0;

    if let Some(dpi) = args.dpi {
        let cmd: Vec<u8> = vec![0x07, 0x04, 0x05, 0x01];
        let mut args = dpi.to_be_bytes().to_vec();
        args.extend(args.clone()); // Duplicate it's size
        let footer = 0x07;
        errorcode += send_cmd(&hid_result, cmd, args, footer);
    }

    if let Some(hz) = args.hz {
        // 125
        // let cmd: Vec<u8> = vec![0x01, 0x00, 0x05, 0x08];
        // let frequency: u8 = 0x00;
        // let footer: u8 = 0x0c;

        // 500
        // let cmd: Vec<u8> = vec![0x01, 0x00, 0x05, 0x02];
        // let frequency: u8 = 0x00;
        // let footer: u8 = 0x06;

        // 1000
        // let cmd: Vec<u8> = vec![0x01, 0x00, 0x05, 0x01];
        // let frequency: u8 = 0x00;
        // let footer: u8 = 0x05;
        let (cmd, cmdargs, footer) = match hz {
            125 => (vec![0x01, 0x00, 0x05, 0x08], 0x00, 0x0c),
            500 => (vec![0x01, 0x00, 0x05, 0x08], 0x00, 0x0c),
            1000 => (vec![0x01, 0x00, 0x05, 0x08], 0x00, 0x0c),
            500 => (vec![0x01, 0x00, 0x05, 0x02], 0x00, 0x06),
            1000 => (vec![0x01, 0x00, 0x05, 0x01], 0x00, 0x05),
            _ => unreachable!(),
        };
        send_cmd(&hid_result, cmd, vec![cmdargs], footer);
    }

    if let Some(whz) = args.whz {
        // 8000 Wireless
        // let cmd: Vec<u8> = vec![0x02, 0x00, 0x40, 0x01];
        // let args: Vec<u8> = vec![0x01];
        // let footer: u8 = 0x42;

        // 4000 Wireless
        // let cmd: Vec<u8> = vec![0x02, 0x00, 0x40, 0x01];
        // let args: Vec<u8> = vec![0x02];
        // let footer: u8 = 0x41;

        // 2000 Wireless
        // let cmd: Vec<u8> = vec![0x02, 0x00, 0x40, 0x01];
        // let args: Vec<u8> = vec![0x04];
        // let footer: u8 = 0x47;

        // 1000 Wireless
        // let cmd: Vec<u8> = vec![0x02, 0x00, 0x40, 0x00];
        // let args: Vec<u8> = vec![0x08];
        // let footer: u8 = 0x4a;

        // 500 Wireless
        // let cmd: Vec<u8> = vec![0x02, 0x00, 0x40, 0x01];
        // let args: Vec<u8> = vec![0x10];
        // let footer: u8 = 0x53;

        // 125 Wireless
        // let cmd: Vec<u8> = vec![0x02, 0x00, 0x40, 0x01];
        // let args: Vec<u8> = vec![0x40];
        // let footer: u8 = 0x03;
        let (cmd, cmdargs, footer) = match whz {
            125 => (vec![0x02, 0x00, 0x40, 0x01], 0x40, 0x03),
            500 => (vec![0x02, 0x00, 0x40, 0x01], 0x10, 0x53),
            1000 => (vec![0x02, 0x00, 0x40, 0x00], 0x08, 0x4a),
            2000 => (vec![0x02, 0x00, 0x40, 0x01], 0x04, 0x47),
            4000 => (vec![0x02, 0x00, 0x40, 0x01], 0x02, 0x41),
            8000 => (vec![0x02, 0x00, 0x40, 0x01], 0x01, 0x42),
            _ => unreachable!(),
        };
        send_cmd(&hid_result, cmd, vec![cmdargs], footer);
    }
}
