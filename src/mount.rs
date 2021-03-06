use libc::{c_ulong, c_int, c_void};
use {NixResult, NixPath, from_ffi};

bitflags!(
    flags MsFlags: c_ulong {
        const MS_RDONLY      = 1 << 0,  // Mount read-only
        const MS_NOSUID      = 1 << 1,  // Ignore suid and sgid bits
        const MS_NODEV       = 1 << 2,  // Disallow access to device special files
        const MS_NOEXEC      = 1 << 3,  // Disallow program execution
        const MS_SYNCHRONOUS = 1 << 4,  // Writes are synced at once
        const MS_REMOUNT     = 1 << 5,  // Alter flags of a mounted FS
        const MS_MANDLOCK    = 1 << 6,  // Allow mandatory locks on a FS
        const MS_DIRSYNC     = 1 << 7,  // Directory modifications are synchronous
        const MS_NOATIME     = 1 << 10, // Do not update access times
        const MS_NODIRATIME  = 1 << 11, // Do not update directory access times
        const MS_BIND        = 1 << 12, // Linux 2.4.0 - Bind directory at different place
        const MS_MOVE        = 1 << 13,
        const MS_REC         = 1 << 14,
        const MS_VERBOSE     = 1 << 15, // Deprecated
        const MS_SILENT      = 1 << 15,
        const MS_POSIXACL    = 1 << 16,
        const MS_UNBINDABLE  = 1 << 17,
        const MS_PRIVATE     = 1 << 18,
        const MS_SLAVE       = 1 << 19,
        const MS_SHARED      = 1 << 20,
        const MS_RELATIME    = 1 << 21,
        const MS_KERNMOUNT   = 1 << 22,
        const MS_I_VERSION   = 1 << 23,
        const MS_STRICTATIME = 1 << 24,
        const MS_NOSEC       = 1 << 28,
        const MS_BORN        = 1 << 29,
        const MS_ACTIVE      = 1 << 30,
        const MS_NOUSER      = 1 << 31,
        const MS_RMT_MASK    = MS_RDONLY.bits
                              | MS_SYNCHRONOUS.bits
                              | MS_MANDLOCK.bits
                              | MS_I_VERSION.bits,
        const MS_MGC_VAL     = 0xC0ED0000,
        const MS_MGC_MSK     = 0xffff0000
    }
);

bitflags!(
    flags MntFlags: c_int {
        const MNT_FORCE   = 1 << 0,
        const MNT_DETATCH = 1 << 1,
        const MNT_EXPIRE  = 1 << 2
    }
);

mod ffi {
    use libc::{c_char, c_int, c_void, c_ulong};

    extern {
        pub fn mount(
                source: *const c_char,
                target: *const c_char,
                fstype: *const c_char,
                flags: c_ulong,
                data: *const c_void) -> c_int;

        pub fn umount(target: *const c_char) -> c_int;

        pub fn umount2(target: *const c_char, flags: c_int) -> c_int;
    }
}

// XXX: Should `data` be a `NixPath` here?
pub fn mount<P1: NixPath, P2: NixPath, P3: NixPath, P4: NixPath>(
        source: Option<P1>,
        target: P2,
        fstype: Option<P3>,
        flags: MsFlags,
        data: Option<P4>) -> NixResult<()> {
    use libc;

    let res = try!(try!(try!(try!(
        source.with_nix_path(|source| {
            target.with_nix_path(|target| {
                fstype.with_nix_path(|fstype| {
                    data.with_nix_path(|data| {
                        unsafe {
                            ffi::mount(source,
                                       target,
                                       fstype,
                                       flags.bits,
                                       data as *const libc::c_void)
                        }
                    })
                })
            })
        })))));

    return from_ffi(res);
}

pub fn umount<P: NixPath>(target: P) -> NixResult<()> {
    let res = try!(target.with_nix_path(|ptr| {
        unsafe { ffi::umount(ptr) }
    }));

    from_ffi(res)
}

pub fn umount2<P: NixPath>(target: P, flags: MntFlags) -> NixResult<()> {
    let res = try!(target.with_nix_path(|ptr| {
        unsafe { ffi::umount2(ptr, flags.bits) }
    }));

    from_ffi(res)
}
