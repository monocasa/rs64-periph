pub const DCACHE_LINE_SIZE: usize = 16;
pub const DCACHE_LINE_MASK: usize = !(DCACHE_LINE_SIZE - 1);

pub unsafe fn write_cp0_count(new_count: u32) {
    asm!("mtc0 $0, $$9"
        :
        : "r"(new_count));
}

pub unsafe fn read_cp0_count() -> u32 {
    let cp0_count: u32;
    asm!("mfc0 $0, $$9"
        : "=r"(cp0_count));
    cp0_count
}

pub unsafe fn write_cp0_compare(new_compare: u32) {
    asm!("mtc0 $0, $$11"
        :
        : "r"(new_compare));
}

pub unsafe fn read_cp0_compare() -> u32 {
    let cp0_compare: u32;
    asm!("mfc0 $0, $$11"
        : "=r"(cp0_compare));
    cp0_compare
}

pub unsafe fn read_cp0_status() -> u32 {
    let cp0_status: u32;
    asm!("mfc0 $0, $$12"
        : "=r"(cp0_status));
    cp0_status
}

pub unsafe fn read_cp0_cause() -> u32 {
    let cp0_cause: u32;
    asm!("mfc0 $0, $$13"
        : "=r"(cp0_cause));
    cp0_cause
}

pub fn ticks_to_usecs(ticks: u64) -> u64 {
    (ticks * 1000) / 46875
}

pub fn dcache_hit_writeback_invalidate(addr: usize, length: usize) {
    let aligned_base = addr & DCACHE_LINE_MASK;
    let end = addr + length;
    let mut current_base = aligned_base;
    while current_base < end {
        unsafe {
            asm!("cache 0x15,($0)"
                :
                :"r"(current_base));
        }
        current_base += DCACHE_LINE_SIZE;
    }
}
