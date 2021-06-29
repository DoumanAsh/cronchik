use core::{ptr, mem};

pub trait IteratorExt: Iterator {
    ///Returns `None` if iterator doesn't contain exactly `S` element
    fn collect_exact<const S: usize>(&mut self) -> Option<[Self::Item; S]> {
        let mut result = mem::MaybeUninit::<[Self::Item; S]>::uninit();

        let result_item_ptr = result.as_mut_ptr() as *mut Self::Item;
        for idx in 0..S {
            let next = self.next()?;
            unsafe {
                ptr::write(result_item_ptr.add(idx), next);
            }
        }

        let result = unsafe {
            result.assume_init()
        };

        if self.next().is_some() {
            None
        } else {
            Some(result)
        }
    }
}

impl<T: Iterator> IteratorExt for T {
}
