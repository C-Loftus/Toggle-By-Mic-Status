// Copyright 2025 Colton Loftus
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::convert::TryInto;
use std::ffi::c_void;
use std::ptr::NonNull;

use mac_notification_sys::send_notification;
use objc2_core_audio::{
    AudioObjectGetPropertyData, AudioObjectPropertyAddress,
    kAudioDevicePropertyDeviceIsRunningSomewhere, kAudioHardwarePropertyDefaultInputDevice,
    kAudioObjectPropertyElementMain, kAudioObjectPropertyScopeGlobal, kAudioObjectSystemObject,
};

pub fn notify_and_print(message: &str) {
    println!("{}", message);
    send_notification("Toggle by Mic Update", None, message, None).unwrap();
}

// check if the default input device is running on macos
pub fn microphone_is_in_use() -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let mut default_input_addr = AudioObjectPropertyAddress {
            mSelector: kAudioHardwarePropertyDefaultInputDevice,
            mScope: kAudioObjectPropertyScopeGlobal,
            mElement: kAudioObjectPropertyElementMain,
        };

        let mut default_mic_id_query_output: u32 = 0;
        let mut mic_id_size: u32 = std::mem::size_of::<u32>() as u32;

        let system_object = kAudioObjectSystemObject.try_into().unwrap();

        let default_mic_id_query = AudioObjectGetPropertyData(
            system_object,
            NonNull::from(&mut default_input_addr),
            0,
            std::ptr::null(),
            NonNull::from(&mut mic_id_size),
            NonNull::new((&mut default_mic_id_query_output as *mut u32) as *mut c_void).unwrap(),
        );

        if default_mic_id_query != 0 {
            return Err("failed to get default input device".into());
        }

        // check if that device if is running somewhere ---
        let mut running_addr = AudioObjectPropertyAddress {
            mSelector: kAudioDevicePropertyDeviceIsRunningSomewhere,
            mScope: kAudioObjectPropertyScopeGlobal,
            mElement: kAudioObjectPropertyElementMain,
        };

        let mut is_running_query_output: u32 = 0;
        let mut is_running_size: u32 = std::mem::size_of::<u32>() as u32;

        let default_mic_is_running_query = AudioObjectGetPropertyData(
            default_mic_id_query_output.try_into().unwrap(), // device id as AudioObjectID
            NonNull::from(&mut running_addr),
            0,
            std::ptr::null(),
            NonNull::from(&mut is_running_size),
            NonNull::new((&mut is_running_query_output as *mut u32) as *mut c_void).unwrap(),
        );

        if default_mic_is_running_query != 0 {
            return Err("failed to get default input device".into());
        }

        return Ok(is_running_query_output != 0);
    }
}
