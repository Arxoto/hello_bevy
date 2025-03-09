//! # course: Hello Bevy
//!
//! - course 0: Build project [`main`]
//! - course 1: Hello ECS [`crate::course::course1`]

mod course;

use bevy::prelude::*;
use course::course1;

/// # course 0: Build project
///
/// see [bevy-quick-start](https://bevyengine.org/learn/quick-start/getting-started/setup/)
///
/// ## Create project in Windows
///
/// ```powershell
/// cargo new hello_bevy
/// cd hello_bevy
/// cargo add bevy
/// ```
///
/// ## 多项目配置 If using Cargo Workspaces (Optional)
///
/// edit Cargo.toml
///
/// ```toml
/// [workspace]
/// resolver = "2" # Important! wgpu/Bevy needs this!
/// ```
///
/// ## 优化编译性能 Compile with Performance Optimizations
///
/// edit Cargo.toml
///
/// ```toml
/// # Enable a small amount of optimization in the dev profile.
/// [profile.dev]
/// opt-level = 1
///
/// # Enable a large amount of optimization in the dev profile for dependencies.
/// [profile.dev.package."*"]
/// opt-level = 3
/// ```
///
/// ## 发布模式优化 Release Mode Optimizations (Optional)
///
/// edit Cargo.toml
///
/// ```toml
/// # Enable more optimization in the release profile at the cost of compile time.
/// [profile.release]
/// # Compile the entire crate as one unit.
/// # Slows compile times, marginal improvements.
/// codegen-units = 1
/// # Do a second optimization pass over the entire program, including dependencies.
/// # Slows compile times, marginal improvements.
/// lto = "thin"
/// ```
///
/// P.S. 还有个 wasm-release 模式的配置没写 因为不涉及该项目 可参考官网进行配置
///
/// ## 启动快速编译 Enable Fast Compiles (Optional)
///
/// ### 动态连接 Dynamic Linking
///
/// ```powershell
/// cargo add bevy -F dynamic_linking
/// ```
///
/// ### 替代链接器 Alternative Linkers (LLD)
///
/// ```powershell
/// rustup update # update toolchains
/// cargo install -f cargo-binutils
/// rustup component add llvm-tools-preview
/// ```
///
/// edit .cargo/config.toml
///
/// ```toml
/// # for Windows
/// [target.x86_64-pc-windows-msvc]
/// linker = "rust-lld.exe"
/// ```
///
/// ## 提高运行时性能 Improve Runtime Performance (Optional)
///
/// edit Cargo.toml
///
/// ```toml
/// [dependencies]
/// log = { version = "*", features = ["max_level_debug", "release_max_level_warn"] }
/// ```
///
/// ## How to run
///
/// edit .vscode/launch.json "configurations"
///
/// ```json
/// {
///     "command": "cargo run",
///     "name": "Cargo Run",
///     "request": "launch",
///     "cwd": "${workspaceFolder}/target/debug",
///     "type": "node-terminal"
/// }
/// ```
///
/// ## tips
///
/// - see course doc
///   - run `cargo doc --open`
/// - update bevy version
///   - edit the version in Cargo.toml "dependencies"
///   - run `cargo update`
/// - add subproject
///   - run `cargo new subproject --lib`
///   - edit Cargo.toml "dependencies" `subproject = { path = "./subproject" }`
fn main() {
    // 仅会打印一次 因为里面没做循环 需要 `.add_plugins(DefaultPlugins)` 才能循环打印
    App::new()
        .add_systems(Update, || {
            println!("hello world!");
        })
        .run();

    course1::main();
}
