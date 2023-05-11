fn main() {
    use std::path::PathBuf;
    use std::env;
    println!("cargo:rerun-if-changed=ui/ui.fl");
    let g = fl2rust::Generator::default();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    g.in_out("ui/ui.fl", out_path.join("ui_gen.rs").to_str().unwrap()).expect("Failed to generate rust from fl file!");
}
