//! chaque task ds un module
//! , root a la priorité la plus basse

use crate::{
    constants::{SYS_CLK, SYS_TICK_FREQ},
    thr,
    thr::ThrsInit,
    Regs,
};

use drone_core::{fib, log};
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_stm32_map::{
    periph::{
        gpio::{periph_gpio_b, GpioB, GpioPortPeriph},
        sys_tick::{periph_sys_tick, SysTickPeriph},
    },
    reg,
};

#[derive(Debug)]
pub struct TickOverflow;

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);
    let gpio_b = periph_gpio_b!(reg);
    let sys_tick = periph_sys_tick!(reg);


    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    
    beacon(gpio_b, sys_tick, thr.sys_tick)
    .root_wait()
    .expect("beacon fail");
   

    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}
async fn beacon(
    
    gpio_b: GpioPortPeriph<GpioB>,
    sys_tick: SysTickPeriph,
    thr_sys_tick: thr::SysTick,
) -> Result<(), TickOverflow> {
    
    gpio_b.rcc_busenr_gpioen.set_bit(); // GPIO port B clock enable
    gpio_b.gpio_moder.modify(|r| r.write_moder0(0b01).write_moder7(0b01).write_moder14(0b01)); // Output mode, max speed 2 MHz
    

    loop {

        gpio_b.gpio_bsrr.bs0.set_bit();  
        gpio_b.gpio_bsrr.bs7.set_bit();
        gpio_b.gpio_bsrr.bs14.set_bit();
    }
}
