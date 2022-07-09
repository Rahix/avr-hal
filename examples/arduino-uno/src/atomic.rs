// Copyright (c) 2020 Dario Nieuwenhuis
// Code based of off atomic_polyfill.

pub use core::sync::atomic::{compiler_fence, fence, Ordering};

#[repr(transparent)]
pub struct AtomicUsize {
    inner: core::cell::UnsafeCell<usize>,
}

impl From<usize> for AtomicUsize {
    #[inline]
    fn from(v: usize) -> Self {
        Self::new(v)
    }
}

impl core::fmt::Debug for AtomicUsize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.load(Ordering::SeqCst), f)
    }
}

impl AtomicUsize {
    pub const fn new(v: usize) -> Self {
        Self {
            inner: core::cell::UnsafeCell::new(v),
        }
    }

    pub fn into_inner(self) -> usize {
        self.inner.into_inner()
    }

    pub fn get_mut(&mut self) -> &mut usize {
        self.inner.get_mut()
    }

    pub fn load(&self, _order: Ordering) -> usize {
        return critical_section(|| unsafe { *self.inner.get() });
    }

    pub fn store(&self, val: usize, _order: Ordering) {
        return critical_section(|| unsafe { *self.inner.get() = val });
    }

    fn load_nocs(&self, _order: Ordering) -> usize {
        return unsafe { *self.inner.get() };
    }

    fn store_nocs(&self, val: usize, _order: Ordering) {
        return unsafe { *self.inner.get() = val };
    }

    pub fn swap(&self, val: usize, order: Ordering) -> usize {
        self.op(order, |_| val)
    }

    pub fn compare_exchange(
        &self,
        current: usize,
        new: usize,
        success: Ordering,
        failure: Ordering,
    ) -> Result<usize, usize> {
        self.compare_exchange_weak(current, new, success, failure)
    }

    pub fn compare_exchange_weak(
        &self,
        current: usize,
        new: usize,
        success: Ordering,
        _failure: Ordering,
    ) -> Result<usize, usize> {
        critical_section(|| {
            let old = self.load_nocs(load_ordering(success));
            if old == current {
                self.store_nocs(new, store_ordering(success));
                Ok(old)
            } else {
                Err(old)
            }
        })
    }

    pub fn fetch_add(&self, val: usize, order: Ordering) -> usize {
        self.op(order, |old| old.wrapping_add(val))
    }

    pub fn fetch_sub(&self, val: usize, order: Ordering) -> usize {
        self.op(order, |old| old.wrapping_sub(val))
    }

    pub fn fetch_and(&self, val: usize, order: Ordering) -> usize {
        self.op(order, |old| old & val)
    }

    pub fn fetch_nand(&self, val: usize, order: Ordering) -> usize {
        self.op(order, |old| !(old & val))
    }

    pub fn fetch_or(&self, val: usize, order: Ordering) -> usize {
        self.op(order, |old| old | val)
    }

    pub fn fetch_xor(&self, val: usize, order: Ordering) -> usize {
        self.op(order, |old| old ^ val)
    }

    pub fn fetch_update<F>(
        &self,
        set_order: Ordering,
        _fetch_order: Ordering,
        mut f: F,
    ) -> Result<usize, usize>
    where
        F: FnMut(usize) -> Option<usize>,
    {
        critical_section(|| {
            let old = self.load_nocs(load_ordering(set_order));
            if let Some(new) = f(old) {
                self.store_nocs(new, store_ordering(set_order));
                Ok(old)
            } else {
                Err(old)
            }
        })
    }

    pub fn fetch_max(&self, val: usize, order: Ordering) -> usize {
        self.op(order, |old| old.max(val))
    }

    pub fn fetch_min(&self, val: usize, order: Ordering) -> usize {
        self.op(order, |old| old.min(val))
    }

    fn op(&self, order: Ordering, f: impl FnOnce(usize) -> usize) -> usize {
        critical_section(|| {
            let old = self.load_nocs(load_ordering(order));
            let new = f(old);
            self.store_nocs(new, store_ordering(order));
            old
        })
    }
}

#[allow(unused)]
fn load_ordering(order: Ordering) -> Ordering {
    match order {
        Ordering::Release => Ordering::Relaxed,
        Ordering::Relaxed => Ordering::Relaxed,
        Ordering::SeqCst => Ordering::SeqCst,
        Ordering::Acquire => Ordering::Acquire,
        Ordering::AcqRel => Ordering::Acquire,
        x => x,
    }
}

#[allow(unused)]
fn store_ordering(order: Ordering) -> Ordering {
    match order {
        Ordering::Release => Ordering::Release,
        Ordering::Relaxed => Ordering::Relaxed,
        Ordering::SeqCst => Ordering::SeqCst,
        Ordering::Acquire => Ordering::Relaxed,
        Ordering::AcqRel => Ordering::Release,
        x => x,
    }
}

#[allow(unused)]
fn critical_section<R>(f: impl FnOnce() -> R) -> R {
    avr_device::interrupt::free(move |_| f())
}
