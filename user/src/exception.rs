use core::panic::PanicInfo;
use crate::exit;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[user] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("[user] Panicked: {}", info.message().unwrap());
    }
    exit(-1)
}
