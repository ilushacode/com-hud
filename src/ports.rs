use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use serialport::available_ports;
use windows::core::{GUID, PCWSTR};
use windows::Win32::Devices::DeviceAndDriverInstallation::{
    SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW,
    SetupDiGetDeviceRegistryPropertyW, DIGCF_PRESENT, HDEVINFO, SP_DEVINFO_DATA,
    SETUP_DI_REGISTRY_PROPERTY,
};

const SPDRP_FRIENDLYNAME: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(0x0000000C);

fn get_friendly_name(device_info_set: HDEVINFO, device_index: u32) -> Option<String> {
    let mut dev_info = SP_DEVINFO_DATA {
        cbSize: std::mem::size_of::<SP_DEVINFO_DATA>() as u32,
        ..Default::default()
    };

    unsafe {
        if SetupDiEnumDeviceInfo(device_info_set, device_index, &mut dev_info).is_err() {
            return None;
        }

        let mut buffer = [0u8; 1024];
        let mut required_size: u32 = 0;

        let result = SetupDiGetDeviceRegistryPropertyW(
            device_info_set,
            &dev_info,
            SPDRP_FRIENDLYNAME,
            None,
            Some(&mut buffer[..]),
            Some(&mut required_size),
        );

        if result.is_ok() {
            let u16_slice: &[u16] = std::slice::from_raw_parts(
                buffer.as_ptr() as *const u16,
                buffer.len() / 2,
            );
            let len = u16_slice.iter().position(|&c| c == 0).unwrap_or(u16_slice.len());
            let name = OsString::from_wide(&u16_slice[..len]);
            return name.to_str().map(|s| s.to_string());
        }
    }
    None
}

fn sort_com_ports(lines: &mut [String]) {
    lines.sort_by_key(|line| {
        line.strip_prefix("COM")
            .and_then(|rest| rest.split_whitespace().next())
            .and_then(|num_str| num_str.parse::<u32>().ok())
            .unwrap_or(u32::MAX)
    });
}

/// Возвращает отформатированный и отсортированный список активных COM-портов.
pub fn get_com_port_display_strings() -> Vec<String> {
    let mut result = Vec::new();
    let guid_ports = GUID::from_u128(0x4d36e978_e325_11ce_bfc1_08002be10318);

    unsafe {
        let device_info_set = match SetupDiGetClassDevsW(
            Some(&guid_ports),
            PCWSTR::null(),
            None,
            DIGCF_PRESENT,
        ) {
            Ok(h) => h,
            Err(_) => return result,
        };

        if let Ok(ports) = available_ports() {
            for port in ports {
                let port_name = port.port_name;
                let mut index = 0u32;
                loop {
                    let mut dev_info = SP_DEVINFO_DATA {
                        cbSize: std::mem::size_of::<SP_DEVINFO_DATA>() as u32,
                        ..Default::default()
                    };

                    if SetupDiEnumDeviceInfo(device_info_set, index, &mut dev_info).is_err() {
                        break;
                    }

                    if let Some(friendly) = get_friendly_name(device_info_set, index) {
                        if friendly.contains(&format!("({})", port_name)) {
                            let desc_part = friendly.split(" (").next().unwrap_or(&friendly);
                            let first_two: Vec<&str> =
                                desc_part.split_whitespace().take(2).collect();
                            result.push(format!("{} — {}", port_name, first_two.join(" ")));
                            break;
                        }
                    }
                    index += 1;
                }
            }
        }
        let _ = SetupDiDestroyDeviceInfoList(device_info_set);
    }

    if result.is_empty() {
        result.push("Нет активных COM-портов".to_string());
    }

    sort_com_ports(&mut result);
    result
}