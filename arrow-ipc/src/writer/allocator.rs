use std::convert::Infallible;
use std::ops::{Deref, DerefMut};

use flatbuffers::Allocator;

pub struct IpcAllocator<'a> {
    buf: &'a mut Vec<u8>,
    pos: usize,
    end: usize,
}

impl<'a> IpcAllocator<'a> {
    fn new(buf: &'a mut Vec<u8>, pos: usize, end: usize) -> Self {
        Self { buf, pos, end }
    }

    #[inline]
    fn len(&self) -> usize {
        self.end - self.pos
    }
}

impl<'a> Deref for IpcAllocator<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.buf[self.pos..self.end]
    }
}

impl<'a> DerefMut for IpcAllocator<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buf[self.pos..self.end]
    }
}

unsafe impl<'a> Allocator for IpcAllocator<'a> {
    type Error = Infallible;

    fn grow_downwards(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flatbuffers::FlatBufferBuilder;

    #[test]
    fn test_fb_create() {
        let mut buf = Vec::new();
        FlatBufferBuilder::new_in(IpcAllocator::new(&mut buf, 0, 0));
    }
}
