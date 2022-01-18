use crate::platforms::Platform;

pub struct WhileyPlatform {

}

impl Platform for WhileyPlatform {
    fn name(&self) -> &'static str {
        "whiley"
    }
}

pub const WHILEY_PLATFORM : WhileyPlatform = WhileyPlatform{};
