fn main() {
    //std::env::set_current_dir("winipt")
    //    .expect("error setting cwd to winipt dir. Did you forget to clone the submodules?");
    cc::Build::new()
        //.opt_level(3)
        .file("winipt/libipt/win32.c")
        .define("UNICODE", None)
        .include("winipt/inc")
        .compile("cwinipt");
}
