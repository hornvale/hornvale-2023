use std::any::type_name;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem::size_of;

use crate::scripting_language::garbage_collection::object_header::ObjectHeader;
use crate::scripting_language::garbage_collection::reference::Reference;
use crate::scripting_language::garbage_collection::trace::Trace;
use crate::scripting_language::table::Table;
use crate::scripting_language::value::Value;

/// The `Collector` type.
#[derive(Debug)]
pub struct Collector {
  /// Total bytes allocated.
  pub bytes_allocated: usize,
  /// Next allocation for garbage collection.
  pub next_threshold: usize,
  /// Number of free slots.
  pub free_slots: Vec<usize>,
  /// Object header data.
  pub objects: Vec<Option<ObjectHeader>>,
  /// Strings.
  pub strings: HashMap<String, Reference<String>>,
  /// Grey stack.
  pub gray_stack: VecDeque<usize>,
}

impl Collector {
  /// Constructor.
  #[named]
  pub fn new() -> Self {
    trace_enter!();
    let bytes_allocated = 0;
    trace_var!(bytes_allocated);
    let next_threshold = 1024; // * 1024;
    trace_var!(next_threshold);
    let free_slots = Vec::new();
    trace_var!(free_slots);
    let objects = Vec::new();
    trace_var!(objects);
    let strings = HashMap::new();
    trace_var!(strings);
    let gray_stack = VecDeque::new();
    trace_var!(gray_stack);
    let result = Collector {
      bytes_allocated,
      next_threshold,
      free_slots,
      objects,
      strings,
      gray_stack,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Allocate a chunk of memory.
  #[named]
  pub fn alloc<T: Trace + 'static + Debug>(&mut self, object: T) -> Reference<T> {
    trace_enter!();
    trace_var!(object);
    let repr = format!("{:?}", object).chars().into_iter().take(32).collect::<String>();
    trace_var!(repr);
    let size = object.get_size() + size_of::<ObjectHeader>();
    trace_var!(size);
    self.bytes_allocated += size;
    let is_marked = false;
    trace_var!(is_marked);
    let object = Box::new(object);
    trace_var!(object);
    let entry = ObjectHeader {
      is_marked,
      size,
      object,
    };
    trace_var!(entry);
    let index = match self.free_slots.pop() {
      Some(i) => {
        self.objects[i] = Some(entry);
        i
      },
      None => {
        self.objects.push(Some(entry));
        self.objects.len() - 1
      },
    };
    trace_var!(index);
    debug!(
      "alloc(id: {}, type: {}: repr: {}, bytes: {}, next: {})",
      index,
      type_name::<T>(),
      repr,
      self.bytes_allocated,
      self.next_threshold,
    );
    let result = Reference {
      index,
      marker: PhantomData,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Eliminate duplicate string objects.
  #[named]
  pub fn intern(&mut self, name: String) -> Reference<String> {
    trace_enter!();
    let result = if let Some(&value) = self.strings.get(&name) {
      value
    } else {
      let reference = self.alloc(name.clone());
      trace_var!(reference);
      self.strings.insert(name, reference);
      reference
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Dereference.
  #[named]
  pub fn deref<T: Trace + 'static>(&self, reference: Reference<T>) -> &T {
    trace_enter!();
    trace_var!(reference);
    let result = self.objects[reference.index]
      .as_ref()
      .unwrap_or_else(|| panic!("Reference {} not found", reference.index))
      .object
      .as_any()
      .downcast_ref()
      .unwrap_or_else(|| panic!("Reference {} not found", reference.index));
    trace_exit!();
    result
  }

  /// Dereference mutably.
  #[named]
  pub fn deref_mut<T: Trace + 'static>(&mut self, reference: Reference<T>) -> &mut T {
    trace_enter!();
    trace_var!(reference);
    let result = self.objects[reference.index]
      .as_mut()
      .unwrap_or_else(|| panic!("Reference {} not found", reference.index))
      .object
      .as_any_mut()
      .downcast_mut()
      .unwrap_or_else(|| panic!("Reference {} not found", reference.index));
    trace_exit!();
    result
  }

  /// Free up memory that was allocated for a newly-released object.
  #[named]
  pub fn free(&mut self, index: usize) {
    trace_enter!();
    trace_var!(index);
    if let Some(old) = self.objects[index].take() {
      debug!("free (id: {}, size: {})", index, old.size);
      self.bytes_allocated -= old.size;
      self.free_slots.push(index);
    } else {
      panic!("Double free on {}", index);
    }
    trace_exit!();
  }

  /// Collect the garbage.
  ///
  /// This model of garbage collection follows a tricolor abstraction:
  ///
  /// White: At the beginning of a garbage collection, every object is white.
  /// This color means we have not reached or processed the object at all.
  ///
  /// Gray: During marking, when we first reach an object, we darken it gray.
  /// This color means we know the object itself is reachable and should not be
  /// collected. But we have not yet traced through it to see what other
  /// objects it references. In graph algorithm terms, this is the worklist
  /// — the set of objects we know about but haven’t processed yet.
  ///
  /// Black: When we take a gray object and mark all of the objects it
  /// references, we then turn the gray object black. This color means the mark
  /// phase is done processing that object.
  ///
  /// In terms of that abstraction, the marking process now looks like this:
  ///
  /// 1. Start off with all objects white.
  /// 2. Find all the roots and mark them gray.
  /// 3. Repeat as long as there are still gray objects:
  ///   A. Pick a gray object. Turn any white objects that the object mentions
  ///      to gray.
  ///   B. Mark the original gray object black.
  ///
  /// Note that at every step of this process no black node ever points to a
  /// white node. This property is called the **tricolor invariant**. The
  /// traversal process maintains this invariant to ensure that no reachable
  /// object is ever collected.
  ///
  /// @see https://craftinginterpreters.com/garbage-collection.html
  #[named]
  pub fn collect_garbage(&mut self) {
    trace_enter!();
    let before = self.bytes_allocated;
    trace_var!(before);
    self.trace_references();
    // Strings are handled a bit differently.
    self.remove_white_strings();
    self.sweep();
    self.next_threshold = self.bytes_allocated * 2;
    debug!(
      "collected {} bytes (from {} to {}) next at {}",
      before - self.bytes_allocated,
      before,
      self.bytes_allocated,
      self.next_threshold,
    );
    trace_exit!();
  }

  /// Trace references for gray objects.
  ///
  /// Until the stack empties, we keep pulling out gray objects, traversing
  /// their references, and then marking them black. Traversing an object’s
  /// references may turn up new white objects that get marked gray and added
  /// to the stack. So this function swings back and forth between turning
  /// white objects gray and gray objects black, gradually advancing the
  /// entire wavefront forward.
  ///
  /// @see https://craftinginterpreters.com/garbage-collection.html
  #[named]
  pub fn trace_references(&mut self) {
    trace_enter!();
    debug!("tracing references");
    while let Some(index) = self.gray_stack.pop_front() {
      self.blacken_object(index);
    }
    trace_exit!();
  }

  /// "Mark" an object as black.
  ///
  /// Note that we don’t set any state in the traversed object itself. There
  /// is no direct encoding of “black” in the object’s state. A black object
  /// is any object whose isMarked field is set and that is no longer in the
  /// gray stack.
  ///
  /// @see https://craftinginterpreters.com/garbage-collection.html
  #[named]
  pub fn blacken_object(&mut self, index: usize) {
    trace_enter!();
    debug!("blacken(id: {})", index);
    // Hack to trick the borrow checker to be able to call trace on an element.
    let object = self.objects[index].take();
    object.as_ref().unwrap().object.trace(self);
    self.objects[index] = object;
    trace_exit!();
  }

  /// Mark an object value as gray.
  ///
  /// See ::mark_object() for details, i.e. why we mark as gray and not black
  /// in this step.
  ///
  /// The implementation of `trace()` on Value will determine whether the value
  /// is heap-allocated and thus might need to be collected.  Stack-allocated
  /// values (boolean, nil, numbers) do not, of course.
  ///
  /// For heap-allocated values, that will forward to a type-specific implemen-
  /// tation of `trace()`.
  #[named]
  pub fn mark_value(&mut self, value: Value) {
    trace_enter!();
    trace_var!(value);
    value.trace(self);
    trace_exit!();
  }

  /// Mark an object as gray and add it to the stack.
  ///
  /// This indicates that the object is reachable and should not be collected,
  /// but also that we have not yet traced through it to determine which other
  /// objects it references.
  #[named]
  pub fn mark_object<T: Trace>(&mut self, reference: Reference<T>) {
    trace_enter!();
    trace_var!(reference);
    if let Some(object) = self.objects[reference.index].as_mut() {
      // If the object is already marked, we don’t mark it again and thus don’t
      // add it to the gray stack. This ensures that an already-gray object is
      // not redundantly added and that a black object is not inadvertently
      // turned back to gray. In other words, it keeps the wavefront moving
      // forward through only the white objects.
      if object.is_marked {
        return;
      }
      debug!(
        "mark(id:{}, type:{}, val:{:?})",
        reference.index,
        type_name::<T>(),
        reference
      );
      object.is_marked = true;
      self.gray_stack.push_back(reference.index);
    } else {
      panic!("marking already disposed object {}", reference.index)
    }
    trace_exit!();
  }

  /// Mark the various values that are referenced by a table.
  ///
  /// Don't forget to mark the key strings as well, since they too are managed
  /// by our garbage collection.
  #[named]
  pub fn mark_table(&mut self, table: &Table) {
    trace_enter!();
    trace_var!(table);
    for (&key, &value) in table {
      self.mark_object(key);
      self.mark_value(value);
    }
    trace_exit!();
  }

  /// Should we collect?
  ///
  /// We could use various criteria for this, but we are currently rather
  /// conservative in order to reduce our impact on performance.
  ///
  /// The idea is that the collector frequency automatically adjusts based
  /// on the live size of the heap. We track the total number of bytes of
  /// managed memory that the VM has allocated. When it goes above some
  /// threshold, we trigger a GC. After that, we note how many bytes of memory
  /// remain—how many were not freed. Then we adjust the threshold to some
  /// value larger than that.
  ///
  /// The result is that as the amount of live memory increases, we collect
  /// less frequently in order to avoid sacrificing throughput by re-traversing
  /// the growing pile of live objects. As the amount of live memory goes down,
  /// we collect more frequently so that we don’t lose too much latency by
  /// waiting too long.
  ///
  /// @see https://craftinginterpreters.com/garbage-collection.html
  #[named]
  pub fn should_collect(&self) -> bool {
    trace_enter!();
    let result = self.bytes_allocated > self.next_threshold;
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Sweep.
  ///
  /// This step in the garbage collection process frees memory allocated for
  /// any object that has not been marked as reachable during the mark step.
  ///
  /// When the loop in trace_references() exits, we have processed all the
  /// objects we could get our hands on. The gray stack is empty, and every
  /// object in the heap is either black or white. The black objects are
  /// reachable, and we want to hang on to them. Anything still white never
  /// got touched by the trace and is thus garbage. All that’s left is to
  /// reclaim them.
  ///
  /// @see https://craftinginterpreters.com/garbage-collection.html
  #[named]
  pub fn sweep(&mut self) {
    trace_enter!();
    debug!("sweeping");
    for i in 0..self.objects.len() {
      if let Some(mut object) = self.objects[i].as_mut() {
        if object.is_marked {
          object.is_marked = false;
        } else {
          self.free(i);
        }
      }
    }
    trace_exit!();
  }

  /// Remove strings marked white.
  ///
  /// The VM has a hash table containing a pointer to every single string in the
  /// heap. The VM uses this to de-duplicate strings.
  ///
  /// During the mark phase, we deliberately did not treat the VM’s string table
  /// as a source of roots. If we had, no string would ever be collected. The
  /// string table would grow and grow and never yield a single byte of memory
  /// back to the operating system. That would be bad.
  ///
  /// At the same time, if we do let the GC free strings, then the VM’s string
  /// table will be left with dangling pointers to freed memory. That would be
  /// even worse.
  ///
  /// The string table is special and we need special support for it. In
  /// particular, it needs a special kind of reference. The table should be able
  /// to refer to a string, but that link should not be considered a root when
  /// determining reachability. That implies that the referenced object can be
  /// freed. When that happens, the dangling reference must be fixed too, sort
  /// of like a magic, self-clearing pointer. This particular set of semantics
  /// comes up frequently enough that it has a name: a weak reference.
  ///
  /// We have already implicitly implemented half of the string table’s unique
  /// behavior by virtue of the fact that we don’t traverse it during marking.
  /// That means it doesn’t force strings to be reachable. The remaining piece
  /// is clearing out any dangling pointers for strings that are freed.
  ///
  /// @see https://craftinginterpreters.com/garbage-collection.html
  #[named]
  pub fn remove_white_strings(&mut self) {
    trace_enter!();
    debug!("removing white strings");
    let strings = &mut self.strings;
    let objects = &self.objects;
    strings.retain(|_k, v| objects[v.index].as_ref().unwrap().is_marked);
    trace_exit!();
  }
}

impl Default for Collector {
  #[named]
  fn default() -> Self {
    trace_enter!();
    let result = Self::new();
    trace_var!(result);
    trace_exit!();
    result
  }
}
