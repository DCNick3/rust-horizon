use horizon_sync::mutex::Mutex as HosMutex;

use crate::alloc::{GlobalAlloc, Layout, System};

static DLMALLOC: HosMutex<dlmalloc::Dlmalloc<DlmallocBackend>> = HosMutex::new(dlmalloc::Dlmalloc::new_with_allocator(DlmallocBackend {}));

struct DlmallocBackend {}

unsafe impl dlmalloc::Allocator for DlmallocBackend {
    fn alloc(&self, size: usize) -> (*mut u8, usize, u32) {

        // TODO: round up to nearest power of two
        let size = size.next_power_of_two();

        let res = horizon_global::heap::allocate(Layout::from_size_align(size, Self::page_size(&self)).unwrap());

        let ptr = match res {
            Ok(ptr) => ptr,
            Err(()) => return (core::ptr::null_mut(), 0, 0),
        };
        
        (ptr, size, 0)
    }

    fn remap(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize, _can_move: bool) -> *mut u8 {
        core::ptr::null_mut()
    }

    fn free_part(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize) -> bool {
        false
    }

    fn free(&self, _ptr: *mut u8, _size: usize) -> bool {
        todo!()
    }

    fn can_release_part(&self, _flags: u32) -> bool {
        false
    }

    fn allocates_zeros(&self) -> bool {
        false
    }

    fn page_size(&self) -> usize {
        4096
    }
}

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: Calling malloc() is safe because preconditions on this function match the trait method preconditions.
        let mut dlmalloc = DLMALLOC.lock();
        unsafe { dlmalloc.malloc(layout.size(), layout.align()) }
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: Calling calloc() is safe because preconditions on this function match the trait method preconditions.
        let mut dlmalloc = DLMALLOC.lock();
        unsafe { dlmalloc.calloc(layout.size(), layout.align()) }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: Calling free() is safe because preconditions on this function match the trait method preconditions.
        let mut dlmalloc = DLMALLOC.lock();
        unsafe { dlmalloc.free(ptr, layout.size(), layout.align()) }
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: Calling realloc() is safe because preconditions on this function match the trait method preconditions.
        let mut dlmalloc = DLMALLOC.lock();
        unsafe { dlmalloc.realloc(ptr, layout.size(), layout.align(), new_size) }
    }
}
