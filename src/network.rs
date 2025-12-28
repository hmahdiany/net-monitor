
use std::{fs, error::Error};

pub fn get_tcp_connections() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string("/proc/net/tcp")?;
    
    Ok(contents)
}

pub fn get_udp_connections() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string("/proc/net/udp")?;

    Ok(contents)
}

pub fn decode_stage(hex_code: &str) -> &str {
    match hex_code {
        "0A" => "LISTEN",
        "01" => "ESTABLISHED",
        "06" => "TIME_WAIT",
        "08" => "CLOSE_WAIT",
        "07" => "UDP_LISTEN",
        _ => "UNKNOWN",
    }
}

pub fn decode_address(address: String) -> String {
    let (ip_hex, port_hex) = address.split_once(":").unwrap();

    let ip_bytes = hex::decode(ip_hex).expect("Faild decoding address");
    let port_bytes = hex::decode(port_hex).expect("Faild decoding address");

    let ip_parts: Vec<String> = ip_bytes.iter()
        .rev()
        .map(|byte| byte.to_string())
        .collect();
    let ip_addr = ip_parts.join(".");

    let port_num = u16::from_le_bytes([port_bytes[0], port_bytes[1]]);
    let port_str = port_num.to_string();

    format!("{}:{}", ip_addr,port_str)
}

pub fn find_pid_from_inode(target_inode: &str) -> Option<String> {
    let proc_dirs = fs::read_dir("/proc").ok()?;

    let mut proc_pids = proc_dirs.filter_map(|entry_result|{
        let entry = entry_result.ok()?;

        let file_name = entry.file_name();

        let pid_str = file_name.to_string_lossy().parse::<u32>().ok()?;

        Some(pid_str.to_string())
    });

    proc_pids.find(|pid_str| {
        let fd_path = format!("/proc/{pid_str}/fd");

        let mut fd_dir = match fs::read_dir(fd_path) {
            Ok(dir) => dir,
            Err(_) => return false
        };

        fd_dir.any(|fd_entry_result| {
            let fd_entry = match fd_entry_result {
                Ok(entry) => entry,
                Err(_) => return false,
            };

            let link_target = match fs::read_link(fd_entry.path()) {
                Ok(path) => path,
                Err(_) => return false,
            };

            let link_str = link_target.to_string_lossy();
            let target = format!("socket:[{target_inode}]");
            
            link_str.starts_with(&target)
        })
    })
}
