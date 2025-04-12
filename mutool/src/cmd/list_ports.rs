use serialport::SerialPortType;

pub fn list_ports() {
    let ports = serialport::available_ports().expect("No ports found!");
    println!("Available Ports:");
    for port in ports {
        let SerialPortType::UsbPort(info) = port.port_type else {
            continue;
        };
        if info.vid != 1027 || info.pid != 24592 {
            continue;
        }

        println!("{}", port.port_name);
    }
}
