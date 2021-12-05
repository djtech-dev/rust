// revisions: CHECK-BASE CHECK-OPT
// compile-flags: -C no-prepopulate-passes -Z mir-opt-level=0
//[CHECK-BASE] compile-flags: -Z opt-switch-monomorphizing=off

#![crate_type = "lib"]
#![feature(never_type)]

// CHECK-LABEL: @match_never
#[no_mangle]
pub fn match_never(e: !) -> u8 {
    // CHECK-NOT: switch
    // CHECK: unreachable
    match e {}
}

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

// CHECK-LABEL: @match_either_never
#[no_mangle]
pub fn match_either_never(e: Either<!, !>) -> u8 {
    // CHECK: ; call uninhabited::match_either_generic
    match_either_generic(e)
}

// CHECK-LABEL: ; uninhabited::match_either_generic
fn match_either_generic<A, B>(e: Either<A, B>) -> u8 {
    // CHECK-BASE-NOT: unreachable
    // CHECK-BASE: switch i[[TY:[0-9]+]] undef, label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT:    i[[TY]] 0, label %{{[a-zA-Z0-9_]+}}
    // CHECK-BASE-NEXT:    i[[TY]] 1, label %{{[a-zA-Z0-9_]+}}
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NOT: switch
    // CHECK-OPT: unreachable
    match e {
        Either::Left(_) => 0,
        Either::Right(_) => 1,
    }
}
