#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn system_open_close() {
        unsafe {
            let mut sys = mem::zeroed();

            let err = acOpenSystem(&mut sys);
            assert!(err == AC_ERROR_LIST_AC_ERR_SUCCESS);

            let mut num_devices: usize = 0;

            let err = acSystemUpdateDevices(sys, 200);
            assert!(err == AC_ERROR_LIST_AC_ERR_SUCCESS);

            let err = acSystemGetNumDevices(sys, &mut num_devices);
            assert!(err == AC_ERROR_LIST_AC_ERR_SUCCESS);

            println!("Device count: {}", num_devices);

            let err = acCloseSystem(sys);
            assert!(err == AC_ERROR_LIST_AC_ERR_SUCCESS);
        }
    }
}
