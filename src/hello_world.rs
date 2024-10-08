// https://www.rust-lang.org/
// https://www.rust-lang.org/learn
// https://doc.rust-lang.org/std/index.html
// https://github.com/rust-unofficial/awesome-rust
// https://www.youtube.com/@RustVideos/videos

// https://en.wikipedia.org/wiki/Rust_(programming_language)
// https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/

use std::ops::Deref;
use std::sync::atomic::AtomicUsize;
use std::mem::ManuallyDrop;
use std::cell::UnsafeCell;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::fence;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::ptr::NonNull;
use libc::c_char;
use std::ffi::CStr;

fn main() {
    println!("Hello, world!");
}