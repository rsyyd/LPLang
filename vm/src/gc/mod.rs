use std::cell::UnsafeCell;
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Gc {
    objects: Vec<GcObject>,
    roots: HashSet<*const GcObject>,
    bytes_allocated: AtomicUsize,
    next_gc_threshold: usize,
}

struct GcObject {
    data: UnsafeCell<Vec<u8>>,
    marked: bool,
    size: usize,
}

impl Gc {
    pub fn new(threshold: usize) -> Self {
        Self {
            objects: Vec::new(),
            roots: HashSet::new(),
            bytes_allocated: AtomicUsize::new(0),
            next_gc_threshold: threshold,
        }
    }

    pub fn alloc(&mut self, size: usize) -> *mut u8 {
        if self.bytes_allocated.load(Ordering::Relaxed) > self.next_gc_threshold {
            self.collect();
        }

        let mut data = vec![0u8; size];
        let ptr = data.as_mut_ptr();
        let obj = GcObject {
            data: UnsafeCell::new(data),
            marked: false,
            size,
        };
        self.objects.push(obj);
        self.bytes_allocated.fetch_add(size, Ordering::Relaxed);
        ptr
    }

    pub fn root(&mut self, ptr: *const u8) {
        // Find object containing ptr and add to roots
        for obj in &self.objects {
            let obj_ptr = obj.data.get() as *const u8;
            let end = unsafe { obj_ptr.add(obj.size) };
            if ptr >= obj_ptr && ptr < end {
                self.roots.insert(obj as *const GcObject);
                break;
            }
        }
    }

    pub fn unroot(&mut self, ptr: *const u8) {
        for obj in &self.objects {
            let obj_ptr = obj.data.get() as *const u8;
            let end = unsafe { obj_ptr.add(obj.size) };
            if ptr >= obj_ptr && ptr < end {
                self.roots.remove(&(obj as *const GcObject));
                break;
            }
        }
    }

    fn collect(&mut self) {
        // Mark phase
        for root in &self.roots {
            self.mark(**root);
        }

        // Sweep phase
        let mut new_objects = Vec::new();
        let mut freed = 0;

        for obj in self.objects.drain(..) {
            if obj.marked {
                new_objects.push(obj);
            } else {
                freed += obj.size;
            }
        }

        self.objects = new_objects;
        self.bytes_allocated.fetch_sub(freed, Ordering::Relaxed);
        self.next_gc_threshold = self.bytes_allocated.load(Ordering::Relaxed) * 2;
    }

    fn mark(&mut self, obj: *const GcObject) {
        unsafe {
            if (*obj).marked {
                return;
            }
            (*obj).marked = true;

            // Scan for pointers (simplified - real impl would use precise GC)
            let data = (*obj).data.get();
            let size = (*obj).size;
            let bytes = std::slice::from_raw_parts(data as *const u8, size);

            for chunk in bytes.chunks_exact(std::mem::size_of::<usize>()) {
                let ptr = usize::from_ne_bytes(chunk.try_into().unwrap()) as *const u8;
                if ptr.is_null() { continue; }
                for candidate in &self.objects {
                    let cptr = candidate.data.get() as *const u8;
                    let cend = unsafe { cptr.add(candidate.size) };
                    if ptr >= cptr && ptr < cend {
                        self.mark(candidate);
                    }
                }
            }
        }
    }
}