#![no_std]

mod arch;

pub struct Rtc {
    arch_rtc: arch::Rtc,
}

impl Rtc {
    pub fn new() -> Self {
        Rtc {
            arch_rtc: arch::Rtc::new(),
        }
    }

    /// Returns the current time in seconds since UNIX epoch.
    pub fn get_unix_timestamp(&self) -> u64 {
        self.arch_rtc.get_unix_timestamp()
    }
}
