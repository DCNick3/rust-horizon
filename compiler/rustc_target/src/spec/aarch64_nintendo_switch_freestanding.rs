use super::{LinkerFlavor, LldFlavor, PanicStrategy, RelroLevel, Target, TargetOptions};

const LINKER_SCRIPT: &str = include_str!("./aarch64_nintendo_switch_freestanding_linker_script.ld");

/// A base target for Nintendo Switch devices using a pure LLVM toolchain.
pub fn target() -> Target {
    Target {
        llvm_target: "aarch64-unknown-none".to_string(),
        pointer_width: 64,
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".to_string(),
        arch: "aarch64".to_string(),
        options: TargetOptions {
            linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
            linker: Some("rust-lld".to_owned()),
            link_script: Some(LINKER_SCRIPT.to_string()),
            os: "horizon".to_string(),
            max_atomic_width: Some(128),
            panic_strategy: PanicStrategy::Abort,
            position_independent_executables: true,
            dynamic_linking: true,
            executables: true,
            relro_level: RelroLevel::Off,
            ..Default::default()
        },
    }
}
