//! Screen

use super::{super::cursor::Color, Cursor, font, frame_buffer};

pub const MARGIN: usize = 8;

static mut WIDTH: usize = 0;
static mut HEIGHT: usize = 0;
static mut STRIDE: usize = 0;

pub fn set_config(width: usize, height: usize, stride: usize) {
    unsafe {
        WIDTH = width;
        HEIGHT = height;
        STRIDE = stride;
    }
}

pub fn width() -> usize {
    unsafe { WIDTH }
}

pub fn height() -> usize {
    unsafe { HEIGHT }
}

pub fn stride() -> usize {
    unsafe { STRIDE }
}

pub fn clear() {
    let mut ptr = frame_buffer::base() as *mut u8;
    let size = frame_buffer::size();
    for _ in 0..size {
        unsafe {
            ptr.write_volatile(0x00);
            ptr = ptr.add(1);
        }
    }
}

pub fn last_is_black() -> bool {
    unsafe {
        let mut ptr = (frame_buffer::base() as *mut u32)
            .add(MARGIN * STRIDE)
            .add(MARGIN)
            .add(((Cursor::max_y() - 1) * font::HEIGHT) * STRIDE)
            .add((Cursor::max_x() - 1) * font::WIDTH);
        for _ in 0..font::HEIGHT {
            for _ in 0..font::WIDTH {
                if ptr.read_volatile() != Color::Black as u32 {
                    return false;
                }
                ptr = ptr.add(1);
            }
            ptr = ptr.add(STRIDE).sub(font::WIDTH);
        }
    }
    true
}

pub fn up() {
    unsafe {
        let mut ptr = (frame_buffer::base() as *mut u32)
            .add(MARGIN * STRIDE)
            .add(MARGIN);
        for _ in MARGIN..(HEIGHT - MARGIN) {
            for _ in MARGIN..(WIDTH - MARGIN) {
                ptr.write_volatile(ptr.add(font::HEIGHT * STRIDE).read_volatile());
                ptr = ptr.add(1);
            }
            ptr = ptr.add(MARGIN).add(STRIDE - WIDTH).add(MARGIN);
        }
    }
}

pub fn left(x: usize, y: usize, mut ptr: *mut u32) {
    for i in y..Cursor::max_y() {
        for j in 0..font::HEIGHT {
            unsafe {
                for _ in (if i == y { x * font::WIDTH } else { 0 })
                    ..((Cursor::max_x() - 1) * font::WIDTH)
                {
                    ptr.write_volatile(ptr.add(font::WIDTH).read_volatile());
                    ptr = ptr.add(1);
                }
                for _ in 0..font::WIDTH {
                    ptr.write_volatile(if i + 1 == Cursor::max_y() {
                        Color::Black as u32
                    } else {
                        ptr.sub((Cursor::max_x() - 1) * font::WIDTH)
                            .add(font::HEIGHT * STRIDE)
                            .read_volatile()
                    });
                    ptr = ptr.add(1);
                }
                ptr = ptr.add(MARGIN).add(STRIDE - WIDTH).add(MARGIN);
                if i == y && j + 1 != font::HEIGHT {
                    ptr = ptr.add(x * font::WIDTH);
                }
            }
        }
    }
}

pub fn right(x: usize, y: usize, ptr: *mut u32) {
    unsafe {
        let mut tail = ptr
            .add(((Cursor::max_y() - y) * font::HEIGHT - 1) * STRIDE)
            .add((Cursor::max_x() - x) * font::WIDTH - 1);
        for i in (y..Cursor::max_y()).rev() {
            for _ in 0..font::HEIGHT {
                for _ in (((if i == y { x } else { 0 }) + 1) * font::WIDTH)
                    ..(Cursor::max_x() * font::WIDTH)
                {
                    tail.write_volatile(tail.sub(font::WIDTH).read_volatile());
                    tail = tail.sub(1);
                }
                for _ in 0..font::WIDTH {
                    tail.write_volatile(if i == y {
                        Color::Black as u32
                    } else {
                        tail.sub(font::HEIGHT * STRIDE)
                            .add((Cursor::max_x() - 1) * font::WIDTH)
                            .read_volatile()
                    });
                    tail = tail.sub(1);
                }
                if i == y {
                    tail = tail.sub(x * font::WIDTH);
                }
                tail = tail.sub(MARGIN).sub(STRIDE - WIDTH).sub(MARGIN);
            }
        }
    }
}
