use openal_soft_sys::alext::*;
use std::borrow::Cow;
use std::ptr;

fn string_lossy_from_ptr<'a>(ptr: *const std::os::raw::c_char) -> Cow<'a, str> {
    unsafe { std::ffi::CStr::from_ptr(ptr) }.to_string_lossy()
}

fn str_ptr<T>(s: &str) -> *const T {
    s.as_ptr() as _
}

mod alc {
    use openal_soft_sys::alext::*;
    use std::ffi::CStr;

    pub fn is_extension_present(device: *mut ALCdevice, ext: &str) -> bool {
        let p = unsafe { alcIsExtensionPresent(device, super::str_ptr(ext)) };
        p == (ALC_TRUE as ALCboolean)
    }

    pub fn get_string<'a>(device: *mut ALCdevice, param: ALCenum) -> &'a CStr {
        let s = unsafe { alcGetString(device, param) };
        let s = unsafe { std::ffi::CStr::from_ptr(s) };
        s
    }
}

mod al {
    use openal_soft_sys::alext::*;
    use std::ffi::CStr;

    pub fn is_extension_present(ext: &str) -> bool {
        let p = unsafe { alIsExtensionPresent(super::str_ptr(ext)) };
        p == (AL_TRUE as ALboolean)
    }

    pub fn get_string<'a>(param: ALenum) -> &'a CStr {
        let s = unsafe { alGetString(param) };
        let s = unsafe { std::ffi::CStr::from_ptr(s) };
        s
    }
}

fn print_list_with_terminator(list: Cow<str>, separator: char, terminator: char) {
    for v in list.split(terminator) {
        print!("\t{}{}", v, separator);
    }
}

fn print_list(list: Cow<str>, separator: char) {
    print_list_with_terminator(list, separator, ' ');
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
    let s = alc::get_string(device, ALC_ALL_DEVICES_SPECIFIER as ALCenum);
    if check_alc_errors(device, line!()) == ALC_NO_ERROR as ALCenum {
        println!(
            "playback device{}:\n\t{:?}",
            if device.is_null() { " (null)" } else { "" },
            s.to_string_lossy()
        );
    }

    let s = alc::get_string(device, ALC_CAPTURE_DEVICE_SPECIFIER as ALCenum);
    if check_alc_errors(device, line!()) == ALC_NO_ERROR as ALCenum {
        println!(
            "capture device{}:\n\t{:?}",
            if device.is_null() { " (null)" } else { "" },
            s.to_string_lossy()
        );
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

    println!("ALC extensions:");
    let s = alc::get_string(device, ALC_EXTENSIONS as ALCenum);
    if check_alc_errors(device, line!()) == ALC_NO_ERROR as ALCenum {
        let s = s.to_string_lossy();
        print_list(s, '\n');
    }
}

fn print_hrtf_info(device: *mut ALCdevice) {
    println!("Available HRTFs:");
    if !alc::is_extension_present(device, "ALC_SOFT_HRTF\0") {
        println!("\tHRTF extension not available");
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
    check_alc_errors(device, line!());

    if num_hrtfs == 0 {
        println!("\tNo HRTFs found");
        return;
    }

    for i in 0..num_hrtfs {
        let name = unsafe {
            alcGetStringiSOFT(
                device as *mut ALCdevice,
                ALC_HRTF_SPECIFIER_SOFT as ALCenum,
                i as ALCsizei,
            )
        };
        let name = string_lossy_from_ptr(name);
        println!("\t{}", name);
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
        "OpenAL vendor: {:?}",
        al::get_string(AL_VENDOR as ALenum).to_string_lossy()
    );
    println!(
        "OpenAL renderer: {:?}",
        al::get_string(AL_RENDERER as ALenum).to_string_lossy()
    );
    println!(
        "OpenAL version: {:?}",
        al::get_string(AL_VERSION as ALenum).to_string_lossy()
    );
    println!("OpenAL extensions:");

    let s = al::get_string(AL_EXTENSIONS as ALenum);
    print_list(s.to_string_lossy(), '\n');
    check_al_errors(line!());
}

fn print_resampler_info() {
    println!("Available resamplers:");
    if !al::is_extension_present("AL_SOFT_source_resampler\0") {
        println!("Resampler extension not available");
        return;
    }

    let num_resamplers = unsafe { alGetInteger(AL_NUM_RESAMPLERS_SOFT as ALenum) };
    check_al_errors(line!());

    if num_resamplers == 0 {
        println!("Resamplers not found");
        return;
    }

    let def_resampler = unsafe { alGetInteger(AL_DEFAULT_RESAMPLER_SOFT as ALenum) };
    check_al_errors(line!());

    for i in 0..num_resamplers {
        let name = unsafe { alGetStringiSOFT(AL_RESAMPLER_NAME_SOFT as ALenum, i) };
        check_al_errors(line!());
        println!(
            "\t{}{}",
            string_lossy_from_ptr(name),
            if i == def_resampler { " <default>" } else { "" }
        );
    }
}

fn print_efx_info(device: *mut ALCdevice) {
    println!("Available EFX");
    if !alc::is_extension_present(device, "ALC_EXT_EFX\0") {
        println!("\tEFX extension not available");
        return;
    }

    let mut major: ALCint = 0;
    let mut minor: ALCint = 0;
    unsafe {
        alcGetIntegerv(device, ALC_EFX_MAJOR_VERSION as ALCenum, 1, &mut major as *mut ALCint);
        alcGetIntegerv(device, ALC_EFX_MINOR_VERSION as ALCenum, 1, &mut minor as *mut ALCint);
    }

    if check_alc_errors(device, line!()) == ALC_NO_ERROR as ALCenum {
        println!("\tversion: {}.{}", major, minor);
    }

    let mut sends: ALCint = 0;
    unsafe { alcGetIntegerv(device, ALC_MAX_AUXILIARY_SENDS as ALCenum, 1, &mut sends) };
    if check_alc_errors(device, line!()) == ALC_NO_ERROR as ALCenum {
        println!("\tMax auxiliary sends: {}", sends);
    }

    fn valid_enum(filter: &&&str) -> bool {
        let val = unsafe { alGetEnumValue(str_ptr(filter)) };
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
    if alc::is_extension_present(device, "ALC_EXT_DEDICATED\0") {
        [
            "AL_EFFECT_DEDICATED_DIALOGUE\0",
            "AL_EFFECT_DEDICATED_LOW_FREQUENCY_EFFECT\0",
        ]
        .iter()
        .filter(valid_enum)
        .for_each(print_enum);
    }
}

// FIXME: 書き直し
fn main() -> anyhow::Result<()> {
    assert!(alc::is_extension_present(ptr::null_mut(), "ALC_ENUMERATE_ALL_EXT\0"));

    let s = alc::get_string(ptr::null_mut(), ALC_DEFAULT_ALL_DEVICES_SPECIFIER as ALCenum);
    println!("DEFAULT_ALL_DEVICES:\n\t{:?}", s.to_string_lossy());

    let s = alc::get_string(ptr::null_mut(), ALC_ALL_DEVICES_SPECIFIER as ALCenum);
    println!("ALL_DEVICES:\n\t{:?}", s.to_string_lossy());

    let s = alc::get_string(ptr::null_mut(), ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER as ALCenum);
    println!("CAPTURE_DEFAULT_DEVICE:\n\t{:?}", s.to_string_lossy());

    let s = alc::get_string(ptr::null_mut(), ALC_CAPTURE_DEVICE_SPECIFIER as ALCenum);
    println!("CAPTURE_DEVICE:\n\t{:?}", s.to_string_lossy());

    println!();

    let device = unsafe { alcOpenDevice(ptr::null()) };
    if device.is_null() {
        return Err(anyhow::anyhow!("Failed to open NULL"));
    }

    let context = unsafe { alcCreateContext(device, ptr::null()) };
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

    for &device in &[ptr::null_mut(), device] {
        println!("ALC info");
        print_alc_info(device);
        print_hrtf_info(device);
        print_efx_info(device);
        println!();
    }

    println!("AL info");
    print_al_info();
    print_resampler_info();

    // destroy
    unsafe {
        alcMakeContextCurrent(ptr::null_mut());
        alcDestroyContext(context);
        alcCloseDevice(device);
    }
    Ok(())
}
