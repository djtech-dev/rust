// revisions: CHECK-BASE CHECK-OPT
// compile-flags: -C no-prepopulate-passes -C opt-level=0 -Z mir-opt-level=1
//[CHECK-BASE] compile-flags: -Z opt-switch-monomorphizing=off

#![crate_type = "lib"]
#![feature(core_intrinsics)]
#![feature(never_type)]

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

// CHECK-LABEL: @match_either_never
#[no_mangle]
pub fn match_either_never(e: Either<!, !>) -> u8 {
    // CHECK-BASE-NOT: unreachable
    // CHECK-BASE: switch i[[TY:[0-9]+]] undef, label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT:    i[[TY]] 0, label %{{[a-zA-Z0-9_]+}}
    // CHECK-BASE-NEXT:    i[[TY]] 1, label %{{[a-zA-Z0-9_]+}}
    // CHECK-BASE-NEXT:    i[[TY]] 2, label %{{[a-zA-Z0-9_]+}}
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NOT: switch
    // CHECK-OPT: unreachable
    match std::intrinsics::discriminant_value(&e) {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => 3,
    }
}
