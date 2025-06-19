use bindgen::builder;
use std::{env, io::Write};

// All this crate does is run bindgen on cimplot and store the result
// in the src folder of the implot-sys crate. We add those bindings
// to git so people don't have to install clang just to use implot-rs.

fn main() {
    let cwd = env::current_dir().expect("Could not read current directory");
    let sys_crate_path = cwd.join("..").join("implot-sys").canonicalize().expect("Could not find sys crate directory");
    let bindings = builder()
        .headers([
            &sys_crate_path.join("third-party").join("cimgui").join("cimgui.h").to_str().expect("Could not convert cimgui.h path to string")[4..],
            &sys_crate_path.join("third-party").join("cimplot").join("cimplot.h").to_str().expect("Could not turn cimplot.h path into string")[4..],
        ])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(["-DCIMGUI_DEFINE_ENUMS_AND_STRUCTS=1"])
        // Reuse the imgui types that implot requires from imgui_sys so we don't define
        // our own new types.
        .raw_line("pub use imgui_sys::*;")
        .layout_tests(false)
        .allowlist_recursively(false)
        .allowlist_function("ImPlot.*")
        .allowlist_type("ImPlot.*")
        // We do want to create bindings for the scalar typedefs
        .allowlist_type("Im[U|S][0-9]{1,2}")
        .allowlist_type("ImAxis.*")
        // Remove some functions that would take a variable-argument list
        .blocklist_function("ImPlot_AnnotateVVec4")
        .blocklist_function("ImPlot_AnnotateVStr")
        .blocklist_function("ImPlot_AnnotateClampedVVec4")
        .blocklist_function("ImPlot_AnnotateClampedVStr")
        .blocklist_function("ImPlot_AnnotationV")
        .blocklist_function("ImPlot_TagXV")
        .blocklist_function("ImPlot_TagYV")
        .generate()
        .expect("Unable to generate bindings");

    // The above type re-export shenanigans make bindgen unable to derive Copy, Clone and Debug on
    // some types, but they would work - we hence manually re-add them here.
    let mut bindings_string = bindings.to_string();
    ["ImPlotInputMap", "ImPlotStyle"].iter().for_each(|name| {
        bindings_string = bindings_string.replace(
            &format!("pub struct {}", name),
            &format!("#[derive(Clone, Copy, Debug)]\npub struct {}", name),
        );
    });

    // Finally we write the bindings to a file.
    let out_path = sys_crate_path.join("src");
    let mut out_file =
        std::fs::File::create(&out_path.join("bindings.rs")).expect("Could not open bindings file");
    out_file
        .write_all(&bindings_string.into_bytes()[..])
        .expect("Couldn't write bindings");
}
