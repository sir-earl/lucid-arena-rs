# Lucid Arena SDK - Rust bindings

This package contains low-level Rust bindings for the Lucid Arena SDK.

## Installation

- Ensure the Lucid Arena SDK is installed on your system.

- If your SDK is installed in a different location, you can specify the library path in the `LUCID_ARENA_SDK_LIB_PATH` environment variable.

## Usage

``` toml
[dependencies]
lucid-arena-sys = "0.1.0"
```

## Example usage

``` rust
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
```

## Codegen

To regenerate bindings, the following command is used:

> `bindgen wrapper.h -o src/bindings.rs -- "-Ilucid_arena_sdk_include_path"`
