use std::{io::{Read, Write}};

use lc_crypto::digest::Digest;


pub struct HashRead<R,D>{
    digest: D,
    inner: R
}

impl<R,D> HashRead<R,D>{
    pub const fn new(reader: R, digest: D) -> Self{
        Self { digest, inner: reader }
    }

    pub fn into_inner(self) -> R{
        self.inner
    }
}

impl<R: Read,D: Digest> Read for HashRead<R,D>{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        
        let bytes = self.inner.read(buf)?;

        self.digest.update(&buf[..bytes]);

        Ok(bytes)
    }
}

impl<R,D: Digest> HashRead<R,D>{
    pub fn hash(&mut self) -> Vec<u8>{
        let mut buf = vec![0u8;D::OUTPUT_SIZE];

        self.digest.do_final(&[], &mut buf);

        buf
    }
}

pub struct HashWrite<W,D>{
    inner: W,
    digest: D
}

impl<W,D> HashWrite<W,D>{
    pub const fn new(writer: W, digest: D) -> Self{
        Self{
            inner: writer,
            digest
        }
    }

    pub fn into_inner(self) -> W{
        self.inner
    }
}

impl<W: Write, D: Digest> Write for HashWrite<W,D>{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let bytes = self.inner.write(buf)?;

        self.digest.update(&buf[..bytes]);

        Ok(bytes)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl<W,D: Digest> HashWrite<W,D>{
    pub fn hash(&mut self) -> Vec<u8>{
        let mut buf = vec![0u8;D::OUTPUT_SIZE];

        self.digest.do_final(&[], &mut buf);

        buf
    }
}




