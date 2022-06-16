use super::{LinkerFlavor, LldFlavor, PanicStrategy, RelroLevel, Target, TargetOptions, TlsModel};

//const LINKER_SCRIPT: &str = include_str!("./aarch64_nintendo_switch_homebrew_linker_script.ld");

/// A base target for Nintendo Switch devices using a pure LLVM toolchain.
pub fn target() -> Target {
    Target {
        llvm_target: "aarch64-unknown-none".into(),
        pointer_width: 64,
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".into(),
        arch: "aarch64".into(),
        options: TargetOptions {
            linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
            linker: Some("rust-lld".into()),
            // TODO: enable it back when I figure it out
            //link_script: Some(LINKER_SCRIPT.into()),
            os: "horizon".into(),
            max_atomic_width: Some(128),
            panic_strategy: PanicStrategy::Abort,

            // we always need PIE (argh, PIE/PIC/whatever stuff is complicated)
            position_independent_executables: true,

            // We don't support dynamic linking for now (it's complicated)
            dynamic_linking: false,
            // But we can produce executables
            executables: true,

            // We don't pass argc and argv in horizon-rt
            main_needs_argc_argv: false,

            // We don't support RELRO
            // It's something about making .got and .plt read-only I think?
            // Not sure we care enough, it's just an exploit mitigation
            relro_level: RelroLevel::Off,

            // please emit unwind tables so that I can get the stack traces in debugger
            default_uwtable: true,

            // we do have support for thread-local stuff in horizon-rt & (TODO) on thread creation
            has_thread_local: true,
            tls_model: TlsModel::InitialExec,

            ..Default::default()
        },
    }
}
