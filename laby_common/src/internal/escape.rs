//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
// This is an amalgamation of files for SIMD-optimized HTML escaping code from sailfish.
// References to std were replaced with alloc and core. Support for Miri was removed.
// Additional documentation were added. The original source code can be found at:
//
//   https://github.com/Kogia-sima/sailfish/blob/master/sailfish/src/runtime/escape
//
// ===============================================================================
//
// The MIT License (MIT)
// Copyright (c) 2020 Ryohei Machida
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
// OR OTHER DEALINGS IN THE SOFTWARE.
//
static ESCAPE_LUT: [u8; 256] = [
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 0, 9, 9, 9, 1, 2, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 3, 9, 4, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
];

const ESCAPED: [&str; 5] = ["&quot;", "&amp;", "&#39;", "&lt;", "&gt;"];
const ESCAPED_LEN: usize = 5;

use super::buffer::Buffer;
use alloc::string::String;
use core::{
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};
use ptr::copy_nonoverlapping as memcpy_16;

/// Takes a string slice [`&str`] and writes the escaped form into the given [`Buffer`].
///
/// This is a port of [sailfish][5]'s `escape_to_buf` function
/// which is an [x86][4] [SIMD-optimized][1] HTML escaping function.
/// The characters `&"'<>` are replaced with the respective HTML entities.
///
/// If the platform supports [AVX2][2] or [SSE2][3], this function will delegate
/// to the fastest SIMD-optimized implementation automatically. Otherwise, it will gracefully
/// degrade to a fallback scalar implementation. This feature detection is performed at runtime
/// and is not determined by the target platform at compile time.
///
/// Non-[x86][4] platforms will always use the fallback scalar implementation regardless of
/// SIMD support.
///
/// To escape a string as a [`String`] instead of [`Buffer`] conveniently,
/// see [`escape_str`] function.
///
/// [1]: https://en.wikipedia.org/wiki/SIMD
/// [2]: https://en.wikipedia.org/wiki/Advanced_Vector_Extensions
/// [3]: https://en.wikipedia.org/wiki/SSE2
/// [4]: https://en.wikipedia.org/wiki/X86
/// [5]: https://docs.rs/sailfish/
///
/// # Example
///
/// ```
/// # use laby_common as laby;
/// # use laby::internal::*;
/// use laby::internal::Buffer;
///
/// let mut buffer = Buffer::new();
/// escape("a < b", &mut buffer);
///
/// assert_eq!(buffer.into_string(), "a &lt; b");
/// ```
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn escape(feed: &str, buf: &mut Buffer) {
    type FnRaw = *mut ();
    static FN: AtomicPtr<()> = AtomicPtr::new(detect as FnRaw);

    fn detect(feed: &str, buf: &mut Buffer) {
        debug_assert!(feed.len() >= 16);
        let features = raw_cpuid::CpuId::new().get_feature_info();
        let fun = if let Some(features) = features {
            if features.has_avx() {
                avx2::escape
            } else if features.has_sse2() {
                sse2::escape
            } else {
                fallback::escape
            }
        } else {
            fallback::escape
        };

        FN.store(fun as FnRaw, Ordering::Relaxed);
        unsafe { fun(feed, buf) };
    }

    unsafe {
        if feed.len() < 16 {
            buf.reserve_small(feed.len() * 6);
            let l = naive::escape_small(feed, buf.as_mut_ptr().add(buf.len()));
            buf.advance(l);
        } else {
            let fun = FN.load(Ordering::Relaxed);
            core::mem::transmute::<FnRaw, fn(&str, &mut Buffer)>(fun)(feed, buf);
        }
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn escape(feed: &str, buf: &mut Buffer) {
    unsafe {
        if feed.len() < 16 {
            buf.reserve_small(feed.len() * 6);
            let l = naive::escape_small(feed, buf.as_mut_ptr().add(buf.len()));
            buf.advance(l);
        } else {
            fallback::escape(feed, buf)
        }
    }
}

/// Takes a string slice [`&str`] and returns the escaped [`String`].
///
/// This is equivalent to constructing a new [`Buffer`], calling [`escape`] into that buffer
/// and then returning [`Buffer::into_string()`].
///
/// # Example
///
/// ```
/// # use laby_common::internal::*;
/// let s = "a < b";
/// let t = "a &lt; b";
///
/// assert_eq!(escape_str(s), t);
/// ```
#[inline]
pub fn escape_str(s: &str) -> String {
    let mut buffer = Buffer::new();
    escape(s, &mut buffer);
    buffer.into_string()
}

mod fallback {
    use super::*;

    #[cfg(target_pointer_width = "32")]
    const USIZE_BYTES: usize = 4;

    #[cfg(target_pointer_width = "64")]
    const USIZE_BYTES: usize = 8;

    const USIZE_ALIGN: usize = USIZE_BYTES - 1;

    #[inline]
    fn contains_zero_byte(x: usize) -> bool {
        const LO_U64: u64 = 0x0101_0101_0101_0101;
        const HI_U64: u64 = 0x8080_8080_8080_8080;
        const LO_USIZE: usize = LO_U64 as usize;
        const HI_USIZE: usize = HI_U64 as usize;

        x.wrapping_sub(LO_USIZE) & !x & HI_USIZE != 0
    }

    #[inline]
    fn contains_key(x: usize) -> bool {
        const INDEPENDENTS1: usize = 0x0505_0505_0505_0505_u64 as usize;
        const INDEPENDENTS2: usize = 0x0202_0202_0202_0202_u64 as usize;
        const KEY1: usize = 0x2727_2727_2727_2727_u64 as usize;
        const KEY2: usize = 0x3e3e_3e3e_3e3e_3e3e_u64 as usize;

        let y1 = x | INDEPENDENTS1;
        let y2 = x | INDEPENDENTS2;
        let z1 = y1 ^ KEY1;
        let z2 = y2 ^ KEY2;
        contains_zero_byte(z1) || contains_zero_byte(z2)
    }

    #[inline]
    pub unsafe fn escape(feed: &str, buffer: &mut Buffer) {
        debug_assert!(feed.len() >= 16);

        let len = feed.len();
        let mut start_ptr = feed.as_ptr();
        let end_ptr = feed[len..].as_ptr();

        let mut ptr = start_ptr;
        let aligned_ptr = ptr.add(USIZE_BYTES - (start_ptr as usize & USIZE_ALIGN));
        debug_assert_eq!(aligned_ptr as usize % USIZE_BYTES, 0);
        debug_assert!(aligned_ptr <= end_ptr);

        let chunk = (ptr as *const usize).read_unaligned();
        if contains_key(chunk) {
            start_ptr = naive::proceed(buffer, start_ptr, ptr, aligned_ptr);
        }

        ptr = aligned_ptr;

        while ptr <= end_ptr.sub(USIZE_BYTES) {
            debug_assert_eq!((ptr as usize) % USIZE_BYTES, 0);

            let chunk = *(ptr as *const usize);
            if contains_key(chunk) {
                start_ptr = naive::proceed(buffer, start_ptr, ptr, ptr.add(USIZE_BYTES))
            }
            ptr = ptr.add(USIZE_BYTES);
        }
        debug_assert!(ptr <= end_ptr);
        debug_assert!(start_ptr <= ptr);
        naive::escape(buffer, start_ptr, ptr, end_ptr);
    }
}

mod naive {
    use super::*;
    use core::{ptr, slice};

    #[inline]
    pub(super) unsafe fn escape(
        buffer: &mut Buffer,
        mut start_ptr: *const u8,
        ptr: *const u8,
        end_ptr: *const u8,
    ) {
        start_ptr = proceed(buffer, start_ptr, ptr, end_ptr);

        if end_ptr > start_ptr {
            let slc = slice::from_raw_parts(start_ptr, end_ptr as usize - start_ptr as usize);
            buffer.push_str(core::str::from_utf8_unchecked(slc));
        }
    }

    #[inline]
    pub(super) unsafe fn proceed(
        buffer: &mut Buffer,
        mut start_ptr: *const u8,
        mut ptr: *const u8,
        end_ptr: *const u8,
    ) -> *const u8 {
        while ptr < end_ptr {
            debug_assert!(start_ptr <= ptr);
            let idx = ESCAPE_LUT[*ptr as usize] as usize;
            debug_assert!(idx <= 9);
            if idx >= ESCAPED_LEN {
                ptr = ptr.add(1);
            } else {
                if ptr > start_ptr {
                    let slc = slice::from_raw_parts(start_ptr, ptr as usize - start_ptr as usize);
                    buffer.push_str(core::str::from_utf8_unchecked(slc));
                }
                push_escaped_str(*ESCAPED.get_unchecked(idx), buffer);
                start_ptr = ptr.add(1);
                ptr = ptr.add(1);
            }
        }

        debug_assert_eq!(ptr, end_ptr);
        debug_assert!(start_ptr <= ptr);
        start_ptr
    }

    pub(super) unsafe fn escape_small(feed: &str, mut buf: *mut u8) -> usize {
        let mut start_ptr = feed.as_ptr();
        let mut ptr = start_ptr;
        let end_ptr = start_ptr.add(feed.len());
        let buf_begin = buf;

        while ptr < end_ptr {
            debug_assert!(start_ptr <= ptr);
            let idx = *ESCAPE_LUT.get_unchecked(*ptr as usize) as usize;
            debug_assert!(idx <= 9);
            if idx >= ESCAPED_LEN {
                ptr = ptr.add(1);
            } else {
                let escaped = ESCAPED.get_unchecked(idx);
                if ptr > start_ptr {
                    let len = ptr as usize - start_ptr as usize;

                    memcpy_16(start_ptr, buf, len);
                    buf = buf.add(len);
                }
                memcpy_16(escaped.as_ptr(), buf, escaped.len());
                buf = buf.add(escaped.len());
                start_ptr = ptr.add(1);
                ptr = ptr.add(1);
            }
        }

        debug_assert_eq!(ptr, end_ptr);
        debug_assert!(start_ptr <= ptr);

        if end_ptr > start_ptr {
            let len = end_ptr as usize - start_ptr as usize;
            memcpy_16(start_ptr, buf, len);
            buf = buf.add(len);
        }

        buf as usize - buf_begin as usize
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64"))]
    #[inline]
    pub(super) unsafe fn push_escaped_str(value: &str, buffer: &mut Buffer) {
        buffer.reserve_small(value.len());

        let src = value.as_ptr();
        let dst = buffer.as_mut_ptr().add(buffer.len());

        // memcpy
        let offset = value.len() - 4;
        let t2 = ptr::read_unaligned(src.add(offset) as *const u32);
        let t1 = ptr::read_unaligned(src as *const u32);
        ptr::write_unaligned(dst.add(offset) as *mut u32, t2);
        ptr::write_unaligned(dst as *mut u32, t1);

        buffer._set_len(buffer.len() + value.len());
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
    #[inline]
    pub(super) unsafe fn push_escaped_str(value: &str, buffer: &mut Buffer) {
        buffer.push_str(value);
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod avx2 {
    use super::{naive::push_escaped_str, *};
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    use core::slice;

    const VECTOR_BYTES: usize = core::mem::size_of::<__m256i>();

    pub unsafe fn escape(feed: &str, buffer: &mut Buffer) {
        debug_assert!(feed.len() >= 16);

        let len = feed.len();
        if len < VECTOR_BYTES {
            escape_small(feed, buffer);
            return;
        }

        let mut start_ptr = feed.as_ptr();
        let mut ptr = start_ptr;
        let end_ptr = feed[len..].as_ptr();

        let v_independent1 = _mm256_set1_epi8(5);
        let v_independent2 = _mm256_set1_epi8(2);
        let v_key1 = _mm256_set1_epi8(0x27);
        let v_key2 = _mm256_set1_epi8(0x3e);

        let maskgen = |x: __m256i| -> u32 {
            _mm256_movemask_epi8(_mm256_or_si256(
                _mm256_cmpeq_epi8(_mm256_or_si256(x, v_independent1), v_key1),
                _mm256_cmpeq_epi8(_mm256_or_si256(x, v_independent2), v_key2),
            )) as u32
        };

        while ptr <= end_ptr.sub(VECTOR_BYTES) {
            let mut mask = maskgen(_mm256_loadu_si256(ptr as *const __m256i));
            while mask != 0 {
                let trailing_zeros = mask.trailing_zeros() as usize;
                mask ^= 1 << trailing_zeros;
                let ptr2 = ptr.add(trailing_zeros);
                let c = ESCAPE_LUT[*ptr2 as usize] as usize;
                if c < ESCAPED_LEN {
                    if start_ptr < ptr2 {
                        let slc =
                            slice::from_raw_parts(start_ptr, ptr2 as usize - start_ptr as usize);
                        buffer.push_str(core::str::from_utf8_unchecked(slc));
                    }
                    push_escaped_str(*ESCAPED.get_unchecked(c), buffer);
                    start_ptr = ptr2.add(1);
                }
            }

            ptr = ptr.add(VECTOR_BYTES);
        }

        debug_assert!(ptr.add(VECTOR_BYTES) > end_ptr);

        if ptr < end_ptr {
            debug_assert!((end_ptr as usize - ptr as usize) < VECTOR_BYTES);
            let backs = VECTOR_BYTES - (end_ptr as usize - ptr as usize);

            let mut mask = maskgen(_mm256_loadu_si256(ptr.sub(backs) as *const __m256i)) >> backs;
            while mask != 0 {
                let trailing_zeros = mask.trailing_zeros() as usize;
                mask ^= 1 << trailing_zeros;
                let ptr2 = ptr.add(trailing_zeros);
                let c = ESCAPE_LUT[*ptr2 as usize] as usize;
                if c < ESCAPED_LEN {
                    if start_ptr < ptr2 {
                        let slc =
                            slice::from_raw_parts(start_ptr, ptr2 as usize - start_ptr as usize);
                        buffer.push_str(core::str::from_utf8_unchecked(slc));
                    }
                    push_escaped_str(*ESCAPED.get_unchecked(c), buffer);
                    start_ptr = ptr2.add(1);
                }
            }
        }

        if end_ptr > start_ptr {
            let slc = slice::from_raw_parts(start_ptr, end_ptr as usize - start_ptr as usize);
            buffer.push_str(core::str::from_utf8_unchecked(slc));
        }
    }

    #[inline]
    unsafe fn escape_small(feed: &str, buffer: &mut Buffer) {
        debug_assert!(feed.len() >= 16);
        debug_assert!(feed.len() < VECTOR_BYTES);

        let len = feed.len();
        let mut start_ptr = feed.as_ptr();
        let mut ptr = start_ptr;
        let end_ptr = start_ptr.add(len);

        let v_independent1 = _mm_set1_epi8(5);
        let v_independent2 = _mm_set1_epi8(2);
        let v_key1 = _mm_set1_epi8(0x27);
        let v_key2 = _mm_set1_epi8(0x3e);

        let maskgen = |x: __m128i| -> u32 {
            _mm_movemask_epi8(_mm_or_si128(
                _mm_cmpeq_epi8(_mm_or_si128(x, v_independent1), v_key1),
                _mm_cmpeq_epi8(_mm_or_si128(x, v_independent2), v_key2),
            )) as u32
        };

        let mut mask = maskgen(_mm_loadu_si128(ptr as *const __m128i));
        while mask != 0 {
            let trailing_zeros = mask.trailing_zeros() as usize;
            mask ^= 1 << trailing_zeros;
            let ptr2 = ptr.add(trailing_zeros);
            let c = ESCAPE_LUT[*ptr2 as usize] as usize;
            if c < ESCAPED_LEN {
                if start_ptr < ptr2 {
                    let slc = slice::from_raw_parts(start_ptr, ptr2 as usize - start_ptr as usize);
                    buffer.push_str(core::str::from_utf8_unchecked(slc));
                }
                push_escaped_str(*ESCAPED.get_unchecked(c), buffer);
                start_ptr = ptr2.add(1);
            }
        }

        if len != 16 {
            ptr = ptr.add(16);
            let read_ptr = end_ptr.sub(16);
            let backs = 32 - len;
            let mut mask = maskgen(_mm_loadu_si128(read_ptr as *const __m128i)) >> backs;

            while mask != 0 {
                let trailing_zeros = mask.trailing_zeros() as usize;
                mask ^= 1 << trailing_zeros;
                let ptr2 = ptr.add(trailing_zeros);
                let c = ESCAPE_LUT[*ptr2 as usize] as usize;
                if c < ESCAPED_LEN {
                    if start_ptr < ptr2 {
                        let slc =
                            slice::from_raw_parts(start_ptr, ptr2 as usize - start_ptr as usize);
                        buffer.push_str(core::str::from_utf8_unchecked(slc));
                    }
                    push_escaped_str(*ESCAPED.get_unchecked(c), buffer);
                    start_ptr = ptr2.add(1);
                }
            }
        }

        if end_ptr > start_ptr {
            let slc = slice::from_raw_parts(start_ptr, end_ptr as usize - start_ptr as usize);
            buffer.push_str(core::str::from_utf8_unchecked(slc));
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod sse2 {
    use super::{naive::push_escaped_str, *};
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    use core::slice;

    const VECTOR_BYTES: usize = core::mem::size_of::<__m128i>();

    pub unsafe fn escape(feed: &str, buffer: &mut Buffer) {
        let len = feed.len();
        let mut start_ptr = feed.as_ptr();
        let mut ptr = start_ptr;
        let end_ptr = feed[len..].as_ptr();

        let v_independent1 = _mm_set1_epi8(5);
        let v_independent2 = _mm_set1_epi8(2);
        let v_key1 = _mm_set1_epi8(0x27);
        let v_key2 = _mm_set1_epi8(0x3e);

        let maskgen = |x: __m128i| -> u32 {
            _mm_movemask_epi8(_mm_or_si128(
                _mm_cmpeq_epi8(_mm_or_si128(x, v_independent1), v_key1),
                _mm_cmpeq_epi8(_mm_or_si128(x, v_independent2), v_key2),
            )) as u32
        };

        while ptr <= end_ptr.sub(VECTOR_BYTES) {
            let mut mask = maskgen(_mm_loadu_si128(ptr as *const __m128i));
            while mask != 0 {
                let trailing_zeros = mask.trailing_zeros() as usize;
                mask ^= 1 << trailing_zeros;
                let ptr2 = ptr.add(trailing_zeros);
                let c = ESCAPE_LUT[*ptr2 as usize] as usize;
                if c < ESCAPED_LEN {
                    if start_ptr < ptr2 {
                        let slc =
                            slice::from_raw_parts(start_ptr, ptr2 as usize - start_ptr as usize);
                        buffer.push_str(core::str::from_utf8_unchecked(slc));
                    }
                    push_escaped_str(*ESCAPED.get_unchecked(c), buffer);
                    start_ptr = ptr2.add(1);
                }
            }

            ptr = ptr.add(VECTOR_BYTES);
        }

        debug_assert!(ptr.add(VECTOR_BYTES) > end_ptr);

        if ptr < end_ptr {
            debug_assert!((end_ptr as usize - ptr as usize) < VECTOR_BYTES);
            let backs = VECTOR_BYTES - (end_ptr as usize - ptr as usize);
            let read_ptr = ptr.sub(backs);

            let mut mask = maskgen(_mm_loadu_si128(read_ptr as *const __m128i)) >> backs;
            while mask != 0 {
                let trailing_zeros = mask.trailing_zeros() as usize;
                mask ^= 1 << trailing_zeros;
                let ptr2 = ptr.add(trailing_zeros);
                let c = ESCAPE_LUT[*ptr2 as usize] as usize;
                if c < ESCAPED_LEN {
                    if start_ptr < ptr2 {
                        let slc =
                            slice::from_raw_parts(start_ptr, ptr2 as usize - start_ptr as usize);
                        buffer.push_str(core::str::from_utf8_unchecked(slc));
                    }
                    push_escaped_str(*ESCAPED.get_unchecked(c), buffer);
                    start_ptr = ptr2.add(1);
                }
            }
        }

        if end_ptr > start_ptr {
            let slc = slice::from_raw_parts(start_ptr, end_ptr as usize - start_ptr as usize);
            buffer.push_str(core::str::from_utf8_unchecked(slc));
        }
    }
}
