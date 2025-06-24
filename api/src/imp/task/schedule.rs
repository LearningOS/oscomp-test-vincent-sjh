use crate::ptr::{PtrWrapper, UserConstPtr, UserPtr};
use arceos_posix_api as api;
use axerrno::{LinuxError, LinuxResult};
use axtask::{AxCpuMask, current};

pub fn sys_sched_yield() -> LinuxResult<isize> {
    Ok(api::sys_sched_yield() as _)
}

pub fn sys_nanosleep(
    req: UserConstPtr<api::ctypes::timespec>,
    rem: UserPtr<api::ctypes::timespec>,
) -> LinuxResult<isize> {
    unsafe { Ok(api::sys_nanosleep(req.get()?, rem.get()?) as _) }
}

pub fn sys_sched_getaffinity
(
     pid: i32,
     cpusetsize: usize,
     user_mask: UserPtr<u8>,
) -> LinuxResult<isize> {
    if cpusetsize * 8 < axconfig::SMP {
        return Err(LinuxError::EINVAL);
    }

    // TODO: support other threads
    if pid != 0 {
        return Err(LinuxError::EPERM);
    }

    let mask = current().cpumask();
    let mask_bytes = mask.as_bytes();
    user_mask
        .get_as_mut_slice(mask_bytes.len())?
        .copy_from_slice(mask_bytes);

    Ok(0)
}

pub fn sys_sched_setaffinity(
    pid: i32,
    cpusetsize: usize,
    user_mask: UserConstPtr<u8>,
) -> LinuxResult<isize> {
    let size = cpusetsize.min(axconfig::SMP.div_ceil(8));
    let user_mask = user_mask.get_as_slice(size)?;
    let mut cpu_mask = AxCpuMask::new();

    for i in 0..(size * 8).min(axconfig::SMP) {
        if user_mask[i / 8] & (1 << (i % 8)) != 0 {
            cpu_mask.set(i, true);
        }
    }

    // TODO: support other threads
    if pid != 0 {
        return Err(LinuxError::EPERM);
    }
    axtask::set_current_affinity(cpu_mask);

    Ok(0)
}