//! Process management syscalls

use crate::{
    config::MAX_SYSCALL_NUM, mm::{frame_alloc, translate_and_write_bytes, PTEFlags, PageTable, PhysPageNum, VirtPageNum},
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_current_task_id, get_task_first_run_time, get_task_syscall_times, suspend_current_and_run_next, TaskStatus
    },
    timer::{get_time_ms, get_time_us},
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
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
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let timeval_size = core::mem::size_of::<TimeVal>();
    let return_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let return_val_bytes = unsafe {
        core::slice::from_raw_parts(
            &return_val as *const TimeVal as *const u8,
            timeval_size,
        )
    };
    translate_and_write_bytes(current_user_token(), _ts as *const u8, timeval_size, return_val_bytes);
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let task_id = get_current_task_id();
    trace!("task_id: {}", task_id);
    let task_info = TaskInfo {
        status: TaskStatus::Running,
        syscall_times: get_task_syscall_times(task_id),
        time: get_time_ms() - get_task_first_run_time(task_id),
    };
    let task_info_size = core::mem::size_of::<TaskInfo>();
    let task_info_bytes = unsafe {
        core::slice::from_raw_parts(
            &task_info as *const TaskInfo as *const u8,
            task_info_size,
        )
    };
    translate_and_write_bytes(current_user_token(), _ti as *const u8, task_info_size, task_info_bytes);
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    // _start should be aligned to 4096, or return -1
    if _start % 4096 != 0 {
        error!("start should be aligned to 4096");
        return -1;
    }
    // the lower 3 bits of _port cannot be all zeros
    if _port & 0b111 == 0 {
        error!("the lower 3 bits of port cannot be all zeros");
        return -1;
    }
    // other bits of _port should be all zeros
    if _port & !0b111 == 0 {
        error!("other bits of port should be all zeros");
        return -1;
    }

    // real_length is the length of the memory that should be allocated, ceil to 4096
    let real_length = (_len + 4095) / 4096 * 4096;

    let mut pte_flags = PTEFlags::empty();
    pte_flags |= PTEFlags::U;
    if _port & 0b1 != 0 {
        pte_flags |= PTEFlags::R;
    }
    if _port & 0b10 != 0 {
        pte_flags |= PTEFlags::W;
    }
    if _port & 0b100 != 0 {
        pte_flags |= PTEFlags::X;
    }

    for i in 0..real_length / 4096 {
        // try to allocate a frame
        let result = frame_alloc();
        if result.is_none() {
            error!("frame_alloc failed");
            return -1;
        }

        let frame = result.unwrap();
        // map the frame to _start + i * 4096
        let mut page_table = PageTable::from_token(current_user_token());
        page_table.map(VirtPageNum::from(_start + i * 4096), PhysPageNum::from(frame.ppn), pte_flags);
    }
    trace!("kernel: sys_mmap success");
    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    // _start should be aligned to 4096, or return -1
    if _start % 4096 != 0 {
        error!("start should be aligned to 4096");
        return -1;
    }
    // real_length is the length of the memory that should be deallocated, ceil to 4096
    let real_length = (_len + 4095) / 4096 * 4096;

    for i in 0..real_length / 4096 {
        let mut page_table = PageTable::from_token(current_user_token());
        page_table.unmap(VirtPageNum::from(_start + i * 4096));
    }
    trace!("kernel: sys_munmap success");
    0
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
