use crate::connection::Device;

pub fn exec(path: &str) {
    let mut device = Device::connect(path);
    if !device.ping() {
        return;
    }

    device.exec();
}
