use libc::{c_char, c_int, kill, pid_t, SIGKILL};
use regex::Regex;
use std::{ffi::CStr, mem, thread::sleep, time::Duration};

unsafe extern "C" {
    unsafe fn proc_listpids(
        p_type: c_int,
        typeinfo: u32,
        buffer: *mut pid_t,
        buffersize: c_int,
    ) -> c_int;
    unsafe fn proc_name(pid: c_int, buffer: *mut c_char, buffersize: u32) -> c_int;
}

#[cfg(target_os = "macos")]
fn macos_kill_process() {
    let re = Regex::new(r"^Discord(?: PTB| Canary)?(?: Helper(?: \([^)]+\))?)?$").unwrap();
    unsafe {
        let mut pids: [pid_t; 4096] = mem::zeroed();
        let pids_size = (pids.len() * std::mem::size_of::<pid_t>()) as i32;
        let bytes_written = proc_listpids(1, 0, pids.as_mut_ptr(), pids_size);

        let num_pids = bytes_written as usize / std::mem::size_of::<pid_t>();
        for i in 0..num_pids {
            let pid = pids[i];
            if pid < 0 {
                continue;
            }
            let mut name_buffer: [i8; 256] = mem::zeroed();
            let name_len = proc_name(pid, name_buffer.as_mut_ptr(), 256);
            if name_len > 0 {
                let name = CStr::from_ptr(name_buffer.as_ptr());
                // println!("PID: {}  Name: {}", pid, name.to_string_lossy());
                let name = name.to_str().unwrap();
                if re.is_match(&name) {
                    // println!("Killing {} (PID {})", name, pid);
                    let _ = kill(pid, SIGKILL);
                }
            }
        }
    }
}

fn main() {
    loop {
        #[cfg(target_os = "macos")]
        macos_kill_process();
        sleep(Duration::from_secs(3));
    }
}
