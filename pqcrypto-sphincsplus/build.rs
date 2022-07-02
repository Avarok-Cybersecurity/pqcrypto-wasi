extern crate cc;
extern crate glob;

use std::env;
use std::path::{Path, PathBuf};

macro_rules! build_clean {
    ($variant:expr) => {
        let internals_include_path = &std::env::var("DEP_PQCRYPTO_INTERNALS_INCLUDEPATH").unwrap();
        let common_dir = Path::new("pqclean/common");

        let mut builder = cc::Build::new();
        if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default().as_str() == "wasi" {
            if env::var("CARGO_CFG_TARGET_ARCH")
                .unwrap_or_default()
                .as_str()
                == "wasm64"
            {
                builder.target("wasm64-wasi");
            } else {
                builder.target("wasm32-wasi");
            }
        }

        let target_dir: PathBuf = ["pqclean", "crypto_sign", $variant, "clean"]
            .iter()
            .collect();

        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
        if target_os == "wasi" {
            let wasi_sdk_path =
                &std::env::var("WASI_SDK_DIR").expect("missing environment variable: WASI_SDK_DIR");
            builder.flag(format!("--sysroot={}", wasi_sdk_path).as_str());
        }

        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();

        builder
            .include(internals_include_path)
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile(format!("{}_clean", $variant).as_str());
    };
}

macro_rules! build_aesni {
    ($variant:expr) => {
        let internals_include_path = &std::env::var("DEP_PQCRYPTO_INTERNALS_INCLUDEPATH").unwrap();
        let common_dir = Path::new("pqclean/common");

        let mut builder = cc::Build::new();
        if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default().as_str() == "wasi" {
            if env::var("CARGO_CFG_TARGET_ARCH")
                .unwrap_or_default()
                .as_str()
                == "wasm64"
            {
                builder.target("wasm64-wasi");
            } else {
                builder.target("wasm32-wasi");
            }
        }

        let target_dir: PathBuf = ["pqclean", "crypto_sign", $variant, "aesni"]
            .iter()
            .collect();

        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
        if target_os == "wasi" {
            let wasi_sdk_path =
                &std::env::var("WASI_SDK_DIR").expect("missing environment variable: WASI_SDK_DIR");
            builder.flag(format!("--sysroot={}", wasi_sdk_path).as_str());
        }

        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        if cfg!(target_env = "msvc") {
            builder.flag("/arch:AVX2");
        } else {
            builder.flag("-maes");
        }

        builder
            .include(internals_include_path)
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile(format!("{}_aesni", $variant).as_str());
    };
}

macro_rules! build_avx2 {
    ($variant:expr) => {
        let internals_include_path = &std::env::var("DEP_PQCRYPTO_INTERNALS_INCLUDEPATH").unwrap();
        let common_dir = Path::new("pqclean/common");

        let mut builder = cc::Build::new();
        if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default().as_str() == "wasi" {
            if env::var("CARGO_CFG_TARGET_ARCH")
                .unwrap_or_default()
                .as_str()
                == "wasm64"
            {
                builder.target("wasm64-wasi");
            } else {
                builder.target("wasm32-wasi");
            }
        }

        let target_dir: PathBuf = ["pqclean", "crypto_sign", $variant, "avx2"]
            .iter()
            .collect();

        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
        if target_os == "wasi" {
            let wasi_sdk_path =
                &std::env::var("WASI_SDK_DIR").expect("missing environment variable: WASI_SDK_DIR");
            builder.flag(format!("--sysroot={}", wasi_sdk_path).as_str());
        }

        let scheme_files = glob::glob(target_dir.join("*.[csS]").to_str().unwrap()).unwrap();
        if cfg!(target_env = "msvc") {
            builder.flag("/arch:AVX2");
        } else {
            builder
                .flag("-mavx2")
                .flag("-mbmi2")
                .flag("-mbmi")
                .flag("-maes")
                .flag("-mpopcnt")
                .flag("-mpclmul");
        }

        builder
            .include(internals_include_path)
            .include(&common_dir)
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile(format!("{}_avx2", $variant).as_str());
    };
}

fn main() {
    #[allow(unused_variables)]
    let avx2_enabled = env::var("CARGO_FEATURE_AVX2").is_ok();
    #[allow(unused_variables)]
    let aes_enabled = env::var("CARGO_FEATURE_AES").is_ok();
    #[allow(unused_variables)]
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    #[allow(unused_variables)]
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    #[allow(unused_variables)]
    let is_windows = target_os == "windows";
    #[allow(unused_variables)]
    let is_macos = target_os == "macos";

    build_clean!("sphincs-haraka-128f-robust");
    if aes_enabled && target_arch == "x86_64" {
        build_aesni!("sphincs-haraka-128f-robust");
    }
    build_clean!("sphincs-haraka-128f-simple");
    if aes_enabled && target_arch == "x86_64" {
        build_aesni!("sphincs-haraka-128f-simple");
    }
    build_clean!("sphincs-haraka-128s-robust");
    if aes_enabled && target_arch == "x86_64" {
        build_aesni!("sphincs-haraka-128s-robust");
    }
    build_clean!("sphincs-haraka-128s-simple");
    if aes_enabled && target_arch == "x86_64" {
        build_aesni!("sphincs-haraka-128s-simple");
    }
    build_clean!("sphincs-haraka-192f-robust");
    if aes_enabled && target_arch == "x86_64" {
        build_aesni!("sphincs-haraka-192f-robust");
    }
    build_clean!("sphincs-haraka-192f-simple");
    if aes_enabled && target_arch == "x86_64" {
        build_aesni!("sphincs-haraka-192f-simple");
    }
    build_clean!("sphincs-haraka-192s-robust");
    if aes_enabled && target_arch == "x86_64" {
        build_aesni!("sphincs-haraka-192s-robust");
    }
    build_clean!("sphincs-haraka-192s-simple");
    if aes_enabled && target_arch == "x86_64" {
        build_aesni!("sphincs-haraka-192s-simple");
    }
    build_clean!("sphincs-haraka-256f-robust");
    if aes_enabled && target_arch == "x86_64" {
        build_aesni!("sphincs-haraka-256f-robust");
    }
    build_clean!("sphincs-haraka-256f-simple");
    if aes_enabled && target_arch == "x86_64" {
        build_aesni!("sphincs-haraka-256f-simple");
    }
    build_clean!("sphincs-haraka-256s-robust");
    if aes_enabled && target_arch == "x86_64" {
        build_aesni!("sphincs-haraka-256s-robust");
    }
    build_clean!("sphincs-haraka-256s-simple");
    if aes_enabled && target_arch == "x86_64" {
        build_aesni!("sphincs-haraka-256s-simple");
    }
    build_clean!("sphincs-shake256-128f-robust");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-shake256-128f-robust");
    }
    build_clean!("sphincs-shake256-128f-simple");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-shake256-128f-simple");
    }
    build_clean!("sphincs-shake256-128s-robust");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-shake256-128s-robust");
    }
    build_clean!("sphincs-shake256-128s-simple");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-shake256-128s-simple");
    }
    build_clean!("sphincs-shake256-192f-robust");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-shake256-192f-robust");
    }
    build_clean!("sphincs-shake256-192f-simple");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-shake256-192f-simple");
    }
    build_clean!("sphincs-shake256-192s-robust");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-shake256-192s-robust");
    }
    build_clean!("sphincs-shake256-192s-simple");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-shake256-192s-simple");
    }
    build_clean!("sphincs-shake256-256f-robust");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-shake256-256f-robust");
    }
    build_clean!("sphincs-shake256-256f-simple");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-shake256-256f-simple");
    }
    build_clean!("sphincs-shake256-256s-robust");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-shake256-256s-robust");
    }
    build_clean!("sphincs-shake256-256s-simple");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-shake256-256s-simple");
    }
    build_clean!("sphincs-sha256-128f-robust");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-sha256-128f-robust");
    }
    build_clean!("sphincs-sha256-128f-simple");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-sha256-128f-simple");
    }
    build_clean!("sphincs-sha256-128s-robust");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-sha256-128s-robust");
    }
    build_clean!("sphincs-sha256-128s-simple");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-sha256-128s-simple");
    }
    build_clean!("sphincs-sha256-192f-robust");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-sha256-192f-robust");
    }
    build_clean!("sphincs-sha256-192f-simple");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-sha256-192f-simple");
    }
    build_clean!("sphincs-sha256-192s-robust");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-sha256-192s-robust");
    }
    build_clean!("sphincs-sha256-192s-simple");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-sha256-192s-simple");
    }
    build_clean!("sphincs-sha256-256f-robust");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-sha256-256f-robust");
    }
    build_clean!("sphincs-sha256-256f-simple");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-sha256-256f-simple");
    }
    build_clean!("sphincs-sha256-256s-robust");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-sha256-256s-robust");
    }
    build_clean!("sphincs-sha256-256s-simple");
    if avx2_enabled && target_arch == "x86_64" {
        build_avx2!("sphincs-sha256-256s-simple");
    }

    if avx2_enabled && target_arch == "x86_64" {
        // Print enableing flag for AVX2 implementation
        println!("cargo:rustc-cfg=enable_avx2");
    }
    if aes_enabled && target_arch == "x86_64" {
        // Print enableing flag for AES implementation
        println!("cargo:rustc-cfg=enable_aes");
    }
}
