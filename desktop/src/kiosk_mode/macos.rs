use super::permission::has_accessibility_permission;
use super::types::KioskError;
use core_foundation::base::TCFType;
use core_foundation::mach_port::CFMachPort;
use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop, CFRunLoopRunResult};
use core_graphics::event::{CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventType};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

const KEYCODE_ESCAPE: i64 = 53;

// Whitelist of allowed keycodes (letters, numbers, basic punctuation)
const ALLOWED_KEYS: [i64; 66] = [
    // Letters A-Z
    0,  // A
    11, // B
    8,  // C
    2,  // D
    14, // E
    3,  // F
    5,  // G
    4,  // H
    34, // I
    38, // J
    40, // K
    37, // L
    46, // M
    45, // N
    31, // O
    35, // P
    12, // Q
    15, // R
    1,  // S
    17, // T
    32, // U
    9,  // V
    13, // W
    7,  // X
    16, // Y
    6,  // Z
    // Numbers 0-9
    29, // 0
    18, // 1
    19, // 2
    20, // 3
    21, // 4
    23, // 5
    22, // 6
    26, // 7
    28, // 8
    25, // 9
    // Special keys
    49, // Space
    51, // Backspace/Delete
    36, // Return/Enter
    53, // Escape
    48, // Tab
    // Punctuation
    27, // -
    24, // =
    33, // [
    30, // ]
    42, // \
    41, // ;
    39, // '
    43, // ,
    47, // .
    44, // /
    50, // `
    // Arrow keys
    123, // Left
    124, // Right
    125, // Down
    126, // Up
    // Keypad numbers
    82, // Keypad 0
    83, // Keypad 1
    84, // Keypad 2
    85, // Keypad 3
    86, // Keypad 4
    87, // Keypad 5
    88, // Keypad 6
    89, // Keypad 7
    91, // Keypad 8
    92, // Keypad 9
];

type CGEventRef = *mut std::ffi::c_void;
type CGEventTapProxy = *mut std::ffi::c_void;
type CFMachPortRef = *mut std::ffi::c_void;

type CGEventTapCallBack = extern "C" fn(
    proxy: CGEventTapProxy,
    event_type: u32,
    event: CGEventRef,
    user_info: *mut std::ffi::c_void,
) -> CGEventRef;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGEventTapCreate(
        tap: u32,
        place: u32,
        options: u32,
        events_of_interest: u64,
        callback: CGEventTapCallBack,
        user_info: *mut std::ffi::c_void,
    ) -> CFMachPortRef;

    fn CGEventTapEnable(tap: CFMachPortRef, enable: bool);
    fn CGEventGetFlags(event: CGEventRef) -> u64;
    fn CGEventGetIntegerValueField(event: CGEventRef, field: u32) -> i64;
}

const K_CG_EVENT_FLAG_MASK_COMMAND: u64 = 0x00100000;
const K_CG_EVENT_FLAG_MASK_CONTROL: u64 = 0x00040000;
const K_CG_EVENT_FLAG_MASK_ALTERNATE: u64 = 0x00080000;
const K_CG_KEYBOARD_EVENT_KEYCODE: u32 = 9;

/// macOS kiosk mode implementation using CGEventTap
pub struct KioskMode {
    enabled: Arc<AtomicBool>,
    thread_handle: Option<JoinHandle<()>>,
}

impl KioskMode {
    /// Starts kiosk mode, intercepting keyboard events at the system level
    pub fn start() -> Result<Self, KioskError> {
        if !has_accessibility_permission() {
            return Err(KioskError::PermissionDenied);
        }

        let enabled = Arc::new(AtomicBool::new(true));
        let enabled_clone = enabled.clone();

        let handle = thread::spawn(move || {
            run_event_tap_loop(enabled_clone);
        });

        Ok(Self {
            enabled,
            thread_handle: Some(handle),
        })
    }

    /// Stops kiosk mode
    pub fn stop(&mut self) {
        self.enabled.store(false, Ordering::SeqCst);
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }

    /// Returns whether kiosk mode is currently running
    #[allow(dead_code)]
    pub fn is_running(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }
}

impl Drop for KioskMode {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Determines if a keyboard event should be blocked
fn should_block_event(event: CGEventRef) -> bool {
    let flags = unsafe { CGEventGetFlags(event) };
    let keycode = unsafe { CGEventGetIntegerValueField(event, K_CG_KEYBOARD_EVENT_KEYCODE) };

    eprintln!("Key pressed: keycode={}, flags={:#x}", keycode, flags);

    // Block any modifier key combinations (Cmd, Ctrl, Alt) except for Escape
    let has_cmd = (flags & K_CG_EVENT_FLAG_MASK_COMMAND) != 0;
    let has_ctrl = (flags & K_CG_EVENT_FLAG_MASK_CONTROL) != 0;
    let has_alt = (flags & K_CG_EVENT_FLAG_MASK_ALTERNATE) != 0;

    if has_cmd || has_ctrl || has_alt {
        if keycode == KEYCODE_ESCAPE {
            eprintln!("  -> ALLOW (Escape with modifier)");
            return false;
        }
        eprintln!("  -> BLOCK (Modifier combo)");
        return true;
    }

    // Whitelist approach: only allow known safe keys
    if ALLOWED_KEYS.contains(&keycode) {
        eprintln!("  -> ALLOW (Whitelisted key)");
        return false;
    }

    // Block everything else (F keys, media keys, etc.)
    eprintln!("  -> BLOCK (Not in whitelist)");
    true
}

/// Callback function invoked for each keyboard event
extern "C" fn event_tap_callback(
    _proxy: CGEventTapProxy,
    event_type: u32,
    event: CGEventRef,
    _user_info: *mut std::ffi::c_void,
) -> CGEventRef {
    let key_down = CGEventType::KeyDown as u32;
    let key_up = CGEventType::KeyUp as u32;

    if event_type == key_down || event_type == key_up {
        if should_block_event(event) {
            return std::ptr::null_mut();
        }
    }

    event
}

/// Runs the event tap loop in a background thread
fn run_event_tap_loop(enabled: Arc<AtomicBool>) {
    eprintln!("Event tap thread starting...");
    let event_mask: u64 = (1 << CGEventType::KeyDown as u64) | (1 << CGEventType::KeyUp as u64);

    let tap = unsafe {
        CGEventTapCreate(
            CGEventTapLocation::Session as u32,
            CGEventTapPlacement::HeadInsertEventTap as u32,
            CGEventTapOptions::Default as u32,
            event_mask,
            event_tap_callback,
            std::ptr::null_mut(),
        )
    };

    if tap.is_null() {
        eprintln!("ERROR: Failed to create CGEventTap. Make sure accessibility permission is granted.");
        return;
    }

    eprintln!("CGEventTap created successfully!");

    let mach_port = unsafe { CFMachPort::wrap_under_create_rule(tap as *mut _) };
    let run_loop_source = mach_port
        .create_runloop_source(0)
        .expect("Failed to create run loop source");

    let current_run_loop = CFRunLoop::get_current();

    unsafe {
        current_run_loop.add_source(&run_loop_source, kCFRunLoopCommonModes);
        CGEventTapEnable(tap, true);
    }

    while enabled.load(Ordering::SeqCst) {
        let result = CFRunLoop::run_in_mode(
            unsafe { core_foundation::runloop::kCFRunLoopDefaultMode },
            Duration::from_millis(100),
            true,
        );

        if result == CFRunLoopRunResult::Finished {
            break;
        }
    }

    unsafe {
        CGEventTapEnable(tap, false);
    }
}
