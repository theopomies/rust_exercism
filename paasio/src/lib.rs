use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    data: R,
    bytes_through: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(data: R) -> ReadStats<R> {
        Self {
            data,
            bytes_through: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        let read = self.data.read(buf)?;
        self.bytes_through += read;
        Ok(read)
    }
}

pub struct WriteStats<W> {
    data: W,
    bytes_through: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(data: W) -> WriteStats<W> {
        Self {
            data,
            bytes_through: 0,
            writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        let written = self.data.write(buf)?;
        self.bytes_through += written;
        Ok(written)
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}
