//! Process management syscalls
use crate::{
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, current_user_token, get_task_info, TaskInfo, insert_area, unmap_area,
    }, mm::{translated_byte_buffer, VirtAddr, MapPermission}, timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}


/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let tmp = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    let tmp_slice = unsafe {
        core::slice::from_raw_parts(&tmp as *const TimeVal as *const u8, core::mem::size_of::<TimeVal>())
    };

    let mut buffer = translated_byte_buffer(current_user_token(), ts as *const u8, core::mem::size_of::<TimeVal>());
    buffer[0].copy_from_slice(tmp_slice);

    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let task_info = get_task_info();
    let task_info_slice = unsafe {
        core::slice::from_raw_parts(&task_info as *const TaskInfo as *const u8, core::mem::size_of::<TaskInfo>())
    };

    let mut buffer = translated_byte_buffer(current_user_token(), ti as *const u8, core::mem::size_of::<TaskInfo>());
    buffer[0].copy_from_slice(task_info_slice);

    0
}

// YOUR JOB: Implement mmap.
// creates a new mapping in the virtual address space of the
// calling process
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let start_va: VirtAddr = start.into();
    if start_va.page_offset() == 0 && prot & (!0x7) == 0 && prot & (0x7) != 0 {
        let end_va: VirtAddr = (start + len).into();
        let permission = MapPermission::from_bits((prot << 1) as u8).unwrap() | MapPermission::U;
        insert_area(start_va, end_va, permission)
    } else {
        -1
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    let start_va: VirtAddr = start.into();
    let end_va: VirtAddr = (start + len).into();
    if start_va.page_offset() == 0 {
        unmap_area(start_va, end_va)
    } else {
        -1
    }
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
