// revisions: CHECK-BASE CHECK-OPT
// compile-flags: -C no-prepopulate-passes -Z mir-opt-level=0
//[CHECK-BASE] compile-flags: -Z opt-switch-monomorphizing=off

#![crate_type = "lib"]
#![feature(never_type)]

use std::num::NonZeroUsize;

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

// CHECK-LABEL: @match_left_empty
#[no_mangle]
pub fn match_left_empty(e: Either<!, bool>) -> u8 {
    // CHECK-BASE-NOT: br label
    // CHECK-BASE: switch i[[TY:[0-9]+]] 1, label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[TY]] 0, label %{{[a-zA-Z0-9_]+}}
    // CHECK-BASE-NEXT: i[[TY]] 1, label %[[R:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NOT: switch
    // CHECK-OPT: br label %[[R:[a-zA-Z0-9_]+]]
    // CHECK: [[R]]:
    // CHECK-NEXT: store i8 1, i8* %1, align 1
    match e {
        Either::Left(_) => 0,
        Either::Right(_) => 1,
    }
}

// CHECK-LABEL: @match_right_empty
#[no_mangle]
pub fn match_right_empty(e: Either<bool, !>) -> u8 {
    // CHECK-BASE-NOT: br label
    // CHECK-BASE: switch i[[TY:[0-9]+]] 0, label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[TY]] 0, label %[[L:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: i[[TY]] 1, label %{{[a-zA-Z0-9_]+}}
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NOT: switch
    // CHECK-OPT: br label %[[L:[a-zA-Z0-9_]+]]
    // CHECK: [[L]]:
    // CHECK-NEXT: store i8 0, i8* %1, align 1
    match e {
        Either::Left(_) => 0,
        Either::Right(_) => 1,
    }
}

pub struct Uninhabited<T>(T, !);

// CHECK-LABEL: @match_left_large_empty
#[no_mangle]
pub fn match_left_large_empty(e: Either<Uninhabited<usize>, bool>) -> u8 {
    // CHECK-BASE-NOT: br i1
    // CHECK-BASE: switch i[[TY:[0-9]+]] %_{{[0-9]+}}, label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[TY]] 0, label %{{[a-zA-Z0-9_]+}}
    // CHECK-BASE-NEXT: i[[TY]] 1, label %[[R:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NOT: switch
    // CHECK-OPT: %[[D:[0-9]+]] = icmp eq i64 %_{{[0-9]+}}, 1
    // CHECK-OPT-NEXT: br i1 %[[D]], label %[[R:[a-zA-Z0-9_]+]], label %{{[a-zA-Z0-9_]+}}
    // CHECK: [[R]]:
    // CHECK-NEXT: store i8 1, i8* %1, align 1
    match e {
        Either::Left(_) => 0,
        Either::Right(_) => 1,
    }
}

// CHECK-LABEL: @match_large_empty
#[no_mangle]
pub fn match_large_empty(e: Either<bool, Uninhabited<usize>>) -> u8 {
    // CHECK-BASE-NOT: br i1
    // CHECK-BASE: switch i[[TY:[0-9]+]] %_{{[0-9]+}}, label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[TY]] 0, label %[[L:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: i[[TY]] 1, label %{{[a-zA-Z0-9_]+}}
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NOT: switch
    // CHECK-OPT: %[[D:[0-9]+]] = icmp eq i64 %_{{[0-9]+}}, 0
    // CHECK-OPT-NEXT: br i1 %[[D]], label %[[L:[a-zA-Z0-9_]+]], label %{{[a-zA-Z0-9_]+}}
    // CHECK: [[L]]:
    // CHECK-NEXT: store i8 0, i8* %1, align 1
    match e {
        Either::Left(_) => 0,
        Either::Right(_) => 1,
    }
}

// CHECK-LABEL: @match_left_large_empty_nonzero
#[no_mangle]
pub fn match_left_large_empty_nonzero(e: Either<Uninhabited<NonZeroUsize>, ()>) -> u8 {
    // CHECK-BASE-NOT: br i1
    // CHECK-BASE: switch i[[TY:[0-9]+]] %_{{[0-9]+}}, label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[TY]] 0, label %{{[a-zA-Z0-9_]+}}
    // CHECK-BASE-NEXT: i[[TY]] 1, label %[[R:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NOT: switch
    // CHECK-OPT: %[[D:[0-9]+]] = icmp eq i64 %_{{[0-9]+}}, 1
    // CHECK-OPT-NEXT: br i1 %[[D]], label %[[R:[a-zA-Z0-9_]+]], label %{{[a-zA-Z0-9_]+}}
    // CHECK: [[R]]:
    // CHECK-NEXT: store i8 1, i8* %1, align 1
    match e {
        Either::Left(_) => 0,
        Either::Right(_) => 1,
    }
}

// CHECK-LABEL: @match_large_empty_nonzero
#[no_mangle]
pub fn match_large_empty_nonzero(e: Either<(), Uninhabited<NonZeroUsize>>) -> u8 {
    // CHECK-BASE-NOT: br i1
    // CHECK-BASE: switch i[[TY:[0-9]+]] %_{{[0-9]+}}, label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[TY]] 0, label %[[L:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: i[[TY]] 1, label %{{[a-zA-Z0-9_]+}}
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NOT: switch
    // CHECK-OPT: %[[D:[0-9]+]] = icmp eq i64 %_{{[0-9]+}}, 0
    // CHECK-OPT-NEXT: br i1 %[[D]], label %[[L:[a-zA-Z0-9_]+]], label %{{[a-zA-Z0-9_]+}}
    // CHECK: [[L]]:
    // CHECK-NEXT: store i8 0, i8* %1, align 1
    match e {
        Either::Left(_) => 0,
        Either::Right(_) => 1,
    }
}
