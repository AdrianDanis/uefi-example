use core;
use core::time::Duration;
use uefi;
use fencedvar::FencedVar;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Action {
    Spin,
    Restart,
    RestartDelay(Duration),
    Shutdown,
    ShutdownDelay(Duration),
}

static mut PANIC_ACTION: FencedVar<Action> = FencedVar{ construction_value: Action::Shutdown };

pub fn set_panic_action(action: Action) -> () {
    unsafe {PANIC_ACTION.set(action)}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(msg: core::fmt::Arguments,
                               file: &'static str,
                               line: u32,
                               column: u32) -> ! {
    println!("Panic at {} {}:{}: {}", file, line, column, msg);
    let table = || { unsafe {::UEFI_SYSTEM_TABLE.unwrap() } };
    let boot = || { table().boot };
    let run = || { table().runtime };
    let to_micros = |x: Duration| { x.as_secs() * 1000000 + x.subsec_micros() as u64 };
    let sleep = |x: Duration| { boot().stall(to_micros(x) as usize) };
    let reset = |reset_type| { run().reset(reset_type, uefi::Status::Aborted, None) };
    let restart = || { reset(uefi::table::runtime::ResetType::Warm) };
    let shutdown = || { reset(uefi::table::runtime::ResetType::Shutdown) };
    match unsafe {PANIC_ACTION.get()} {
        Action::Spin => loop {},
        Action::Restart => restart(),
        Action::RestartDelay(duration) => {
            println!("Restarting in {} seconds...", duration.as_secs());
            sleep(duration);
            restart()
        },
        Action::Shutdown => shutdown(),
        Action::ShutdownDelay(duration) => {
            println!("Powering down in {} seconds...", duration.as_secs());
            sleep(duration);
            shutdown()
        },
    }
}
