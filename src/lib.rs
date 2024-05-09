use libc::{c_int};

#[link(name = "dd_profiling", kind = "static")]
extern {
    pub fn ddprof_start_profiling() -> c_int;
}

