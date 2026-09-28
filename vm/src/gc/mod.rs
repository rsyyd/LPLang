use std::cell::UnsafeCell;
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Gc {
    objects: Vec<GcObject>,
    roots: HashSet<*mut GcObject>,
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

        let data = vec![0u8; size];
        let obj = GcObject {
            data: UnsafeCell::new(data),
            marked: false,
            size,
        };
        self.objects.push(obj);
        self.bytes_allocated.fetch_add(size, Ordering::Relaxed);
        // Return the pointer to the data
        let obj_ref = self.objects.last_mut().unwrap();
        obj_ref.data.get() as *mut u8
    }

    pub fn root(&mut self, ptr: *const u8) {
        for (i, obj) in self.objects.iter().enumerate() {
            let obj_ptr = obj.data.get() as *const u8;
            let end = unsafe { obj_ptr.add(obj.size) };
            if ptr >= obj_ptr && ptr < end {
                let obj_ptr = &mut self.objects[i] as *mut GcObject;
                self.roots.insert(obj_ptr);
                break;
            }
        }
    }

    pub fn unroot(&mut self, ptr: *const u8) {
        for (i, obj) in self.objects.iter().enumerate() {
            let obj_ptr = obj.data.get() as *const u8;
            let end = unsafe { obj_ptr.add(obj.size) };
            if ptr >= obj_ptr && ptr < end {
                let obj_ptr = &mut self.objects[i] as *mut GcObject;
                self.roots.remove(&obj_ptr);
                break;
            }
        }
    }

    fn collect(&mut self) {
        // Mark phase - collect roots first to avoid borrow issues
        let roots: Vec<*mut GcObject> = self.roots.iter().copied().collect();
        
        // Use a stack for iterative marking instead of recursion
        let mut mark_stack = roots;
        
        while let Some(obj) = mark_stack.pop() {
            self.mark_object(obj, &mut mark_stack);
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

    fn mark_object(&mut self, obj: *mut GcObject, mark_stack: &mut Vec<*mut GcObject>) {
        unsafe {
            if (*obj).marked {
                return;
            }
            (*obj).marked = true;

            // Scan for pointers (simplified - real impl would use precise GC)
            let data = (*obj).data.get();
            let size = (*obj).size;
            let bytes = std::slice::from_raw_parts(data as *const u8, size);

            // Collect indices of objects that might be referenced
            let mut to_add = Vec::new();
            
            for chunk in bytes.chunks_exact(std::mem::size_of::<usize>()) {
                let ptr = usize::from_ne_bytes(chunk.try_into().unwrap()) as *const u8;
                if ptr.is_null() { continue; }
                for (idx, _candidate) in self.objects.iter().enumerate() {
                    let candidate = &self.objects[idx];
                    let cptr = candidate.data.get() as *const u8;
                    let cend = cptr.add(candidate.size);
                    if ptr >= cptr && ptr < cend {
                        to_add.push(idx);
                    }
                }
            }
            
            // Add to mark stack
            for idx in to_add {
                mark_stack.push(&mut self.objects[idx] as *mut GcObject);
            }
        }
    }
}