#[allow(improper_ctypes)]
#[link(name = "modulosys", kind = "static")]
extern "C" {
    pub fn show_window();
}