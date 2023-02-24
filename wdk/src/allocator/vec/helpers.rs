use super::Header;

use core::alloc::Layout;

pub const fn next_aligned(n: usize, alignment: usize) -> usize {
    (n + (alignment - 1)) & !(alignment - 1)
}

pub const fn next_capacity<T>(capacity: usize) -> usize {
    let elem_size = core::mem::size_of::<T>();

    if capacity == 0 {
        return match elem_size {
            1 => 8,
            2..=1024 => 4,
            _ => 1,
        };
    }

    capacity.saturating_mul(2)
}

pub const fn max_align<T>() -> usize {
    let align_t = core::mem::align_of::<T>();
    let header_align = core::mem::align_of::<Header>();

    if align_t > header_align {
        align_t
    } else {
        header_align
    }
}

pub const fn make_layout<T>(capacity: usize) -> Layout {
    let alignment = max_align::<T>();
    let header_size = core::mem::size_of::<Header>();

    let num_bytes = next_aligned(header_size, alignment)
        + next_aligned(capacity * core::mem::size_of::<T>(), alignment);

    unsafe { Layout::from_size_align_unchecked(num_bytes, alignment) }
}

pub const fn max_elems<T>() -> usize {
    let alignment = max_align::<T>();
    let header_bytes = next_aligned(core::mem::size_of::<Header>(), alignment);
    let max = usize::MAX;
    let m = max - (max % alignment) - header_bytes;

    m / core::mem::size_of::<T>()
}
