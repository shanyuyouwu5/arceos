pub mod mem;
pub mod pl061;
#[cfg(feature = "smp")]
pub mod mp;

#[cfg(feature = "irq")]
pub mod irq {
    pub use crate::platform::aarch64_common::gic::*;
}

pub mod console {
    pub use crate::platform::aarch64_common::pl011::*;
}

pub mod time {
    pub use crate::platform::aarch64_common::generic_timer::*;
}

pub mod misc {
    pub use crate::platform::aarch64_common::psci::system_off as terminate;
}

unsafe extern "C" {
    fn rust_main(cpu_id: usize, dtb: usize);
    #[cfg(feature = "smp")]
    fn rust_main_secondary(cpu_id: usize);
}

pub(crate) unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    crate::mem::clear_bss();
    axcpu::init::init_trap();
    crate::cpu::init_primary(cpu_id);
    super::aarch64_common::pl011::init_early();
    super::aarch64_common::generic_timer::init_early();
    rust_main(cpu_id, dtb);
}
fn gpio_init() {
    info!("rust entry");
    use crate::platform::aarch64_common::gic;
    use crate::platform::aarch64_qemu_virt::pl061::pl061_init;
    pl061_init(); 
    crate::irq::set_enable(gic::GPIO_IRQ_NUM,true);
    gic::register_handler(gic::GPIO_IRQ_NUM,handler_gpio_irq);
}
fn handler_gpio_irq() {
    use tock_registers::interfaces::{Writeable,Readable};
    use crate::platform::aarch64_qemu_virt::pl061::*;
    // use crate::platform::aarch64_common::psci::system_off;
    let pl061_reg:&Pl061reg = unsafe{&*PL061_BASE};
    pl061_reg.ic.set(pl061_reg.ie.get());
    // system_off();
    use core::arch::asm;
    unsafe{
        asm!("mov w0, #0x018");
        asm!("hlt #0x0F000");
    }
}
#[cfg(feature = "smp")]
pub(crate) unsafe extern "C" fn rust_entry_secondary(cpu_id: usize) {
    axcpu::init::init_trap();
    crate::cpu::init_secondary(cpu_id);
    rust_main_secondary(cpu_id);
}

/// Initializes the platform devices for the primary CPU.
///
/// For example, the interrupt controller and the timer.
pub fn platform_init() {
    #[cfg(feature = "irq")]
    super::aarch64_common::gic::init_primary();
    super::aarch64_common::generic_timer::init_percpu();
    super::aarch64_common::pl011::init();
    gpio_init();
}

/// Initializes the platform devices for secondary CPUs.
#[cfg(feature = "smp")]
pub fn platform_init_secondary() {
    #[cfg(feature = "irq")]
    super::aarch64_common::gic::init_secondary();
    super::aarch64_common::generic_timer::init_percpu();
}
