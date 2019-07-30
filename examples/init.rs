use livox_sys as livox;

fn main() {
    unsafe {
        livox::Init();
        livox::Uninit();
    }
}
