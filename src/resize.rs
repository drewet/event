use std::intrinsics::TypeId;

use GenericEvent;
use ptr::Ptr;

/// When the window is resized
pub trait ResizeEvent {
    /// Creates a resize event.
    fn from_width_height(w: u32, h: u32) -> Option<Self>;
    /// Calls closure if this is a resize event.
    fn resize<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(u32, u32) -> U;
    /// Returns resize arguments.
    fn resize_args(&self) -> Option<[u32; 2]> {
        self.resize(|x, y| [x, y])
    }
}

impl<T: GenericEvent> ResizeEvent for T {
    #[inline(always)]
    fn from_width_height(w: u32, h: u32) -> Option<T> {
        let id = TypeId::of::<Box<ResizeEvent>>();
        Ptr::with_ref::<(u32, u32), Option<T>, _>(&(w, h), |: ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }

    #[inline(always)]
    fn resize<U, F>(&self, mut f: F) -> Option<U>
        where
            F: FnMut(u32, u32) -> U
    {
        let id = TypeId::of::<Box<ResizeEvent>>();
        self.with_event(id, |&mut: ptr| {
            let &(w, h) = ptr.expect::<(u32, u32)>();
            f(w, h)
        })
    }
}

