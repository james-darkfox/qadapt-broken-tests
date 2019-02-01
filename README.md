
# QADAPT's missing tests

* `assert_no_alloc_then_exit_protected`: The `assert_no_alloc` macro acts like
  `#[deny(allocations)]` but there is no way to ask to forbid allocations. Yes, a malicious
dependency could just hook the system allocator directly.

* `no_alloc`: Works as expected. Or does it?

* `dealloc_in_no_alloc_fn`: Moving a value into a function with the `#[no_alloc]` attribute
  incorrectly permits the deallocation of the moved value.

* `correct_dealloc_panic` and `correct_dealloc_panic2` demonstrate how that should be fixed.

* `enable_without_disable`: Entering protected mode and forgetting to exit will break the caller of
  the offending function. There is no enforced isolation.

* `panic_on_drop_er_deadlock`: If an allocation or deallocation occurs in a drop implementation,
  and a panic occurs in protected mode, a double panic would happen aborting the program. Except it
doesn't. Worse, the mutex hits a deadlock and the whole process hangs forever. I've disabled that
test.

````
running 7 tests
test assert_no_alloc_then_exit_protected ... FAILED
test no_alloc ... ok
test dealloc_in_no_alloc_fn ... FAILED
test correct_dealloc_panic ... ok
test correct_dealloc_panic2 ... ok
test enable_without_disable ... FAILED
test panic_on_drop_er_deadlock ... ignored

failures:

---- enable_without_disable stdout ----
thread 'enable_without_disable' panicked at 'Unexpected deallocation for size 8, protection level: 1', /vendor/qadapt/src/lib.rs:332:17


failures:
    assert_no_alloc_then_exit_protected
    dealloc_in_no_alloc_fn
    enable_without_disable

test result: FAILED. 3 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out
````
