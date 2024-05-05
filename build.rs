use std::{env, path::PathBuf};

fn main() {
    let dst = cmake::Config::new("monado")
        .env("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
        .env("XRT_HAVE_SYSTEM_CJSON", "NO")
        .cflag("-W")
        .cxxflag("-W")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=monado");

    let mut header_paths = walkdir::WalkDir::new("monado/src/xrt/include/xrt")
        .into_iter()
        .filter_map(|maybe_entry| maybe_entry.ok())
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();
    let mut two = walkdir::WalkDir::new("monado/src/xrt/auxiliary/util")
        .into_iter()
        .filter_map(|maybe_entry| maybe_entry.ok())
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();
    header_paths.append(&mut two);
    let header_files_paths = header_paths
        .iter()
        .filter(|&path| {
            path.extension()
                .filter(|ext| ext.to_string_lossy() == "h")
                .is_some()
        })
        .cloned()
        .collect::<Vec<_>>();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let builder = bindgen::Builder::default()
        .derive_default(true)
        .blocklist_item("std::value")
        .blocklist_item("__gnu_cxx::__min")
        .blocklist_item("__gnu_cxx::__max")
        .clang_arg("-xc++")
        .clang_arg("-Imonado/src/xrt/include")
        .clang_arg(format!("-I{}/build/src/xrt/include", out_path.to_string_lossy()))
        .header("monado/src/xrt/include/xrt/xrt_compiler.h")
        .header("monado/src/xrt/include/xrt/xrt_compositor.h")
        .header("monado/src/xrt/include/xrt/xrt_config_os.h")
        .header("monado/src/xrt/include/xrt/xrt_config.h")
        .header("monado/src/xrt/include/xrt/xrt_defines.h")
        .header("monado/src/xrt/include/xrt/xrt_deleters.hpp")
        .header("monado/src/xrt/include/xrt/xrt_device.h")
        .header("monado/src/xrt/include/xrt/xrt_device.hpp")
        .header("monado/src/xrt/include/xrt/xrt_documentation.h")
        .header("monado/src/xrt/include/xrt/xrt_frame.h")
        .header("monado/src/xrt/include/xrt/xrt_frameserver.h")
        .header("monado/src/xrt/include/xrt/xrt_gfx_egl.h")
        .header("monado/src/xrt/include/xrt/xrt_gfx_gl.h")
        .header("monado/src/xrt/include/xrt/xrt_gfx_gles.h")
        .header("monado/src/xrt/include/xrt/xrt_gfx_vk.h")
        .header("monado/src/xrt/include/xrt/xrt_gfx_xlib.h")
        .header("monado/src/xrt/include/xrt/xrt_handles.h")
        .header("monado/src/xrt/include/xrt/xrt_instance.h")
        .header("monado/src/xrt/include/xrt/xrt_limits.h")
        //.header("monado/src/xrt/include/xrt/xrt_openxr_includes.h") todo:
        .header("monado/src/xrt/include/xrt/xrt_prober.h")
        .header("monado/src/xrt/include/xrt/xrt_results.h")
        .header("monado/src/xrt/include/xrt/xrt_session.h")
        .header("monado/src/xrt/include/xrt/xrt_settings.h")
        .header("monado/src/xrt/include/xrt/xrt_space.h")
        .header("monado/src/xrt/include/xrt/xrt_system.h")
        .header("monado/src/xrt/include/xrt/xrt_tracking.h")
        //.header("monado/src/xrt/include/xrt/xrt_visibility_mask.h")
        //.header("monado/src/xrt/include/xrt/xrt_vulkan_includes.h")
        ;
    for header in header_files_paths {
        println!("cargo:rerun-if-changed={}", header.to_string_lossy());
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
