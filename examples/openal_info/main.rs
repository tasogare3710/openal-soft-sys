use openal_soft_sys::alext::*;

fn string_lossy_from_ptr<'a>(ptr: *const std::os::raw::c_char) -> std::borrow::Cow<'a, str> {
    unsafe { std::ffi::CStr::from_ptr(ptr) }.to_string_lossy()
}

fn al_str(s: &str) -> *const ALchar {
    s.as_ptr() as _
}

fn alc_str(s: &str) -> *const ALCchar {
    s.as_ptr() as _
}

fn print_device_list(list: *const ALCchar) {
    let list = string_lossy_from_ptr(list);
    for v in list.split('\0') {
        println!("\t{}", v);
    }
}

fn print_list(list: *const ALCchar, separator: char) {
    let list = string_lossy_from_ptr(list);
    for v in list.split(' ') {
        print!("\t{}{}", v, separator);
    }
}

fn check_alc_errors(device: *mut ALCdevice, lineno: u32) -> ALCenum {
    let err = unsafe { alcGetError(device) };
    if err != ALC_NO_ERROR as ALCenum {
        println!(
            "ALC Error: {} ({:x}), @ {}",
            string_lossy_from_ptr(unsafe { alcGetString(device, err) }),
            err,
            lineno
        );
    }
    err
}

fn print_alc_info(device: *mut ALCdevice) {
    if !device.is_null() {
        print!("\n");
        let mut devname = if unsafe { alcIsExtensionPresent(device, alc_str("ALC_ENUMERATE_ALL_EXT\0")) } !=
            ALC_FALSE as ALCboolean
        {
            unsafe { alcGetString(device, ALC_ALL_DEVICES_SPECIFIER as ALCenum) }
        } else {
            std::ptr::null()
        };

        if check_alc_errors(device, line!()) != ALC_NO_ERROR as ALCenum || devname.is_null() {
            devname = unsafe { alcGetString(device, ALC_DEVICE_SPECIFIER as ALCenum) };
            println!("** Info for device {:?} **", devname);
        }
    }

    let mut major: ALCint = 0;
    let mut minor: ALCint = 0;
    unsafe {
        alcGetIntegerv(device, ALC_MAJOR_VERSION as ALCenum, 1, &mut major as *mut ALCint);
        alcGetIntegerv(device, ALC_MINOR_VERSION as ALCenum, 1, &mut minor as *mut ALCint);
    }

    if check_alc_errors(device, line!()) == ALC_NO_ERROR as ALCenum {
        println!("ALC version: {}.{}", major, minor);
    }

    if !device.is_null() {
        println!("ALC extensions:");
        print_list(unsafe { alcGetString(device, ALC_EXTENSIONS as ALCenum) }, '\n');
        check_alc_errors(device, line!());
    }
}

fn print_hrtf_info(device: *mut ALCdevice) {
    if unsafe { alcIsExtensionPresent(device, alc_str("ALC_SOFT_HRTF\0")) } == ALC_FALSE as ALCboolean {
        println!("HRTF extension not available");
        return;
    }

    let mut num_hrtfs = 0;
    unsafe {
        alcGetIntegerv(
            device,
            ALC_NUM_HRTF_SPECIFIERS_SOFT as ALCenum,
            1,
            &mut num_hrtfs as *mut ALCint,
        )
    };
    if num_hrtfs != 0 {
        println!("Available HRTFs:");
        for i in 0..num_hrtfs {
            // definition AL_ALEXT_PROTOTYPES macro
            let name = string_lossy_from_ptr(unsafe {
                alcGetStringiSOFT(
                    device as *mut ALCdevice,
                    ALC_HRTF_SPECIFIER_SOFT as ALCenum,
                    i as ALCsizei,
                )
            });
            println!("\t{}", name);
        }
    } else {
        println!("No HRTFs found");
    }
    check_alc_errors(device, line!());
}

fn check_al_errors(lineno: u32) -> ALenum {
    let err = unsafe { alGetError() };
    if err != AL_NO_ERROR as ALenum {
        println!(
            "OpenAL Error: {} {:x}, @ {}",
            string_lossy_from_ptr(unsafe { alGetString(err) }),
            err,
            lineno
        );
    }
    err
}

fn print_al_info() {
    println!(
        "OpenAL vendor string: {}",
        string_lossy_from_ptr(unsafe { alGetString(AL_VENDOR as ALenum) })
    );
    println!(
        "OpenAL renderer string: {}",
        string_lossy_from_ptr(unsafe { alGetString(AL_RENDERER as ALenum) })
    );
    println!(
        "OpenAL version string: {}",
        string_lossy_from_ptr(unsafe { alGetString(AL_VERSION as ALenum) })
    );
    println!("OpenAL extensions:");
    print_list(unsafe { alGetString(AL_EXTENSIONS as ALenum) } as *const ALCchar, '\n');
    check_al_errors(line!());
}

fn print_resampler_info() {
    if unsafe { alIsExtensionPresent(al_str("AL_SOFT_source_resampler\0")) } == AL_FALSE as ALboolean {
        println!("Resampler info not available");
        return;
    }

    let num_resamplers = unsafe { alGetInteger(AL_NUM_RESAMPLERS_SOFT as ALenum) };
    if num_resamplers != 0 {
        let def_resampler = unsafe { alGetInteger(AL_DEFAULT_RESAMPLER_SOFT as ALenum) };
        println!("Available resamplers:");
        for i in 0..num_resamplers {
            // definition AL_ALEXT_PROTOTYPES macro
            let name = unsafe { alGetStringiSOFT(AL_RESAMPLER_NAME_SOFT as ALenum, i) };
            println!(
                "\t{}{}",
                string_lossy_from_ptr(name),
                if i == def_resampler { " *" } else { "" }
            );
        }
    } else {
        println!("!!! No resamplers found !!!");
    }
    check_al_errors(line!());
}

fn print_efx_info(device: *mut ALCdevice) {
    if unsafe { alcIsExtensionPresent(device, alc_str("ALC_EXT_EFX\0")) } == ALC_FALSE as ALCboolean {
        println!("EFX not available");
        return;
    }

    let mut major: ALCint = 0;
    let mut minor: ALCint = 0;
    unsafe {
        alcGetIntegerv(device, ALC_EFX_MAJOR_VERSION as ALCenum, 1, &mut major as *mut ALCint);
        alcGetIntegerv(device, ALC_EFX_MINOR_VERSION as ALCenum, 1, &mut minor as *mut ALCint);
    }

    if check_alc_errors(device, line!()) == ALC_NO_ERROR as ALCenum {
        println!("EFX version: {}.{}", major, minor);
    }

    let mut sends: ALCint = 0;
    unsafe { alcGetIntegerv(device, ALC_MAX_AUXILIARY_SENDS as ALCenum, 1, &mut sends) };
    if check_alc_errors(device, line!()) == ALC_NO_ERROR as ALCenum {
        println!("Max auxiliary sends: {}", sends);
    }

    fn valid_enum(filter: &&&str) -> bool {
        let val = unsafe { alGetEnumValue(alc_str(filter)) };
        (unsafe { alGetError() } == AL_NO_ERROR as ALenum) && val != 0 && val != -1
    }
    fn print_enum(e: &&str) {
        println!("\t{}", &e[..e.len() - 1]);
    }
    println!("Supported filters:");
    ["AL_FILTER_LOWPASS\0", "AL_FILTER_HIGHPASS\0", "AL_FILTER_BANDPASS\0"]
        .iter()
        .filter(valid_enum)
        .for_each(print_enum);

    println!("Supported effects:");
    [
        "AL_EFFECT_EAXREVERB\0",
        "AL_EFFECT_REVERB\0",
        "AL_EFFECT_CHORUS\0",
        "AL_EFFECT_DISTORTION\0",
        "AL_EFFECT_ECHO\0",
        "AL_EFFECT_FLANGER\0",
        "AL_EFFECT_FREQUENCY_SHIFTER\0",
        "AL_EFFECT_VOCAL_MORPHER\0",
        "AL_EFFECT_PITCH_SHIFTER\0",
        "AL_EFFECT_RING_MODULATOR\0",
        "AL_EFFECT_AUTOWAH\0",
        "AL_EFFECT_COMPRESSOR\0",
        "AreL_EFFECT_EQUALIZER\0",
    ]
    .iter()
    .filter(valid_enum)
    .for_each(print_enum);

    println!("Supported dedeffects:");
    if unsafe { alcIsExtensionPresent(device, alc_str("ALC_EXT_DEDICATED\0")) } == ALC_TRUE as ALCboolean {
        [
            "AL_EFFECT_DEDICATED_DIALOGUE\0",
            "AL_EFFECT_DEDICATED_LOW_FREQUENCY_EFFECT\0",
        ]
        .iter()
        .filter(valid_enum)
        .for_each(print_enum);
    }
}

fn main() -> anyhow::Result<()> {
    let device = unsafe { alcOpenDevice(std::ptr::null()) };
    if device.is_null() {
        return Err(anyhow::anyhow!("Failed to open NULL"));
    }

    let context = unsafe { alcCreateContext(device, std::ptr::null()) };
    if context.is_null() {
        unsafe { alcCloseDevice(device) };
        return Err(anyhow::anyhow!("!!! Failed to set a context !!!"));
    }

    if unsafe { alcMakeContextCurrent(context) } == ALC_FALSE as ALCboolean {
        unsafe {
            alcDestroyContext(context);
            alcCloseDevice(device);
        };
        return Err(anyhow::anyhow!("!!! Failed to set a context !!!"));
    }

    if unsafe { alcIsExtensionPresent(device, alc_str("ALC_ENUMERATE_ALL_EXT\0")) } != ALC_FALSE as ALCboolean {
        let d = string_lossy_from_ptr(unsafe {
            alcGetString(std::ptr::null_mut(), ALC_DEFAULT_ALL_DEVICES_SPECIFIER as ALCenum)
        });
        println!("Default playback device:\n\t{}", d);
        println!("Available playback devices:");
        print_device_list(unsafe { alcGetString(device, ALC_ALL_DEVICES_SPECIFIER as ALCenum) });
    } else {
        let d = string_lossy_from_ptr(unsafe {
            alcGetString(std::ptr::null_mut(), ALC_DEFAULT_DEVICE_SPECIFIER as ALCenum)
        });
        println!("Default playback device:\n\t{}", d);
        println!("Available playback devices:");
        print_device_list(unsafe { alcGetString(device, ALC_DEVICE_SPECIFIER as ALCenum) });
    }

    println!("Available capture devices:");
    print_device_list(unsafe { alcGetString(device, ALC_CAPTURE_DEVICE_SPECIFIER as ALCenum) });

    print_alc_info(device);
    print_hrtf_info(device);
    print_al_info();
    print_resampler_info();
    print_efx_info(device);

    // destroy
    unsafe {
        alcMakeContextCurrent(std::ptr::null_mut());
        alcDestroyContext(context);
        alcCloseDevice(device);
    }
    Ok(())
}
