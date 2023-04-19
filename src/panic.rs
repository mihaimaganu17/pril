use crate::print;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("!!! PANIC !!!\n");
    if let Some(location) = info.location() {
        print!(
            "In file {:?} at line {:?}...\n",
            location.file(),
            location.line(),
        );
    } else {
        print!("Can't get panic location...\n");
    }
    if let Some(message) = info.message() {
        print!("{}\n", message);
    } else {
        print!("Can't get panic message...\n");
    }
    loop {}
}
