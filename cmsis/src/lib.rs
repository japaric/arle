//! A CMSIS implementation on top of ARLE
//!
//! Just to show that is possible -- well almost.

#![allow(non_snake_case)]
#![no_std]

extern crate arle;

use arle::*;

/* Core Register Access */
#[inline(always)]
pub unsafe fn __get_CONTROL() -> u32 {
    __rsr(CONTROL)
}

#[inline(always)]
pub unsafe fn __set_CONTROL(control: u32) {
    __wsr(CONTROL, control)
}

#[inline(always)]
pub unsafe fn __get_IPSR() -> u32 {
    __rsr(IPSR)
}

#[inline(always)]
pub unsafe fn __get_APSR() -> u32 {
    __rsr(APSR)
}

#[inline(always)]
pub unsafe fn __get_xPSR() -> u32 {
    __rsr(xPSR)
}

#[inline(always)]
pub unsafe fn __get_PSP() -> u32 {
    __rsrp(PSP) as u32
}

#[inline(always)]
pub unsafe fn __set_PSP(top_of_proc_stack: u32) {
    __wsrp(PSP, top_of_proc_stack as *const u8)
}

#[inline(always)]
pub unsafe fn __get_MSP() -> u32 {
    __rsrp(MSP) as u32
}

#[inline(always)]
pub unsafe fn __set_MSP(top_of_main_stack: u32) {
    __wsrp(MSP, top_of_main_stack as *const u8)
}

#[inline(always)]
pub unsafe fn __get_PRIMASK() -> u32 {
    __rsr(PRIMASK)
}

#[inline(always)]
pub unsafe fn __set_PRIMASK(primask: u32) {
    __wsr(PRIMASK, primask)
}

#[inline(always)]
pub unsafe fn __get_BASEPRI() -> u32 {
    __rsr(BASEPRI)
}

#[inline(always)]
pub unsafe fn __set_BASEPRI(base_pri: u32) {
    __wsr(BASEPRI, base_pri)
}

#[inline(always)]
pub unsafe fn __set_BASEPRI_MAX(base_pri: u32) {
    __wsr(BASEPRI_MAX, base_pri)
}

#[inline(always)]
pub unsafe fn __get_FAULTMASK() -> u32 {
    __rsr(FAULTMASK)
}

#[inline(always)]
pub unsafe fn __set_FAULTMASK(fault_mask: u32) {
    __wsr(FAULTMASK, fault_mask)
}

#[inline(always)]
pub unsafe fn __enable_irq() {
    // XXX Should be `CPSIE i` but there's no ACLE API for that. Is this alternative semantically
    // equivalent?
    __wsr(PRIMASK, 0)
}


#[inline(always)]
pub unsafe fn __disable_irq() {
    // XXX Should be `CPSID i` but there's no ACLE API for that. Is this alternative semantically
    // equivalent?
    __wsr(PRIMASK, 1)
}

#[inline(always)]
pub unsafe fn __enable_fault_irq() {
    // XXX Should be `CPSIE f` but there's no ACLE API for that. Is this alternative semantically
    // equivalent?
    __wsr(FAULTMASK, 0)
}


#[inline(always)]
pub unsafe fn __disable_fault_irq() {
    // XXX Should be `CPSID f` but there's no ACLE API for that. Is this alternative semantically
    // equivalent?
    __wsr(FAULTMASK, 1)
}

/* CPU instructions */
#[inline(always)]
pub unsafe fn __NOP() {
    __nop()
}

#[inline(always)]
pub unsafe fn __WFI() {
    __wfi()
}

#[inline(always)]
pub unsafe fn __WFE() {
    __wfe()
}

#[inline(always)]
pub unsafe fn __SEV() {
    __sev()
}

#[inline(always)]
pub unsafe fn __BKPT() {
    // XXX Impossible to implement using ACLE
    unimplemented!()
}

#[inline(always)]
pub unsafe fn __ISB() {
    __isb(SY)
}

#[inline(always)]
pub unsafe fn __DSB() {
    __dsb(SY)
}

#[inline(always)]
pub unsafe fn __DMB() {
    __dmb(SY)
}
