#![cfg(test)]

use qadapt::{QADAPT, assert_no_alloc, enter_protected, exit_protected, no_alloc};

#[global_allocator]
static Q: QADAPT = QADAPT;

#[test]
fn enable_without_disable() {
    enter_protected();
    // whoops. just broke the caller of this function because it wants to do allocations
    // okay, okay. I broke the contract qadapt gave me.
}

#[test]
#[should_panic]
fn assert_no_alloc_then_exit_protected() {
    // do not allocate!
    assert_no_alloc!({
        // but something I called knows about qadapt, and may exit protected mode
        exit_protected();
        // oh no! an allocation and immediate-deallocation!
        Box::new(0);
        // and return to protected mode to make assert_no_alloc happy.
        enter_protected();
    });
    // no panic!
}

#[test]
// panic_on_drop triggers a double-panic on panic, thus aborts the tests - or does it?
#[ignore]
fn panic_on_drop() {
    struct AllocOnDrop;

    impl Drop for AllocOnDrop {
        fn drop(&mut self) {
            Box::new(0);
            // no. it would if it didn't hit a mutex deadlock first
        }
    }

    enter_protected();
    let alloc_drop = AllocOnDrop;
    panic!("this panic will be promoted to a double-panic and abort!^W^W^W^Wdeadlock!");
    // exit_protected();
}

#[test]
#[no_alloc]
#[should_panic]
fn no_alloc() {
    Box::new(0);
    // good, this panics. but what about..
}

#[test]
#[should_panic]
fn dealloc_in_no_alloc_fn() {
    #[no_alloc]
    fn no_alloc_but_does_dealloc(f: Box<i32>) {}

    let b = Box::new(0);
    no_alloc_but_does_dealloc(b);
    // the box was moved into the function but it did not panic!
}

#[test]
#[should_panic]
fn correct_dealloc_panic() {
    // lets fix that
    fn correct_panic_on_dealloc(f: Box<i32>) {
        assert_no_alloc!({ f; });
    }

    let b = Box::new(0);
    correct_panic_on_dealloc(b);
    // panics as expected.
}

#[test]
#[should_panic]
fn correct_dealloc_panic2() {
    // lets fix that a proc-macro friendly way
    fn correct_panic_on_dealloc2(f: Box<i32>) {
        fn _inner(f: Box<i32>) {}
        assert_no_alloc!(_inner(f))
    }

    let b = Box::new(0);
    correct_panic_on_dealloc2(b);
    // panics as expected.
}
