//! Render target abstraction that can write either into an owned buffer or any
//! `std::io::Write` sink. This lets the renderer stream directly to stdout,
//! tmux pipes, files, etc. without forcing an intermediate `String`
//! allocation.

use std::fmt;
use std::io::{self, Write};

/// Internal storage backing the render target.
enum RenderSink<'a> {
    /// Growable in-memory buffer used for `render_to_string` and tests.
    Buffer(String),
    /// Borrowed writer (stdout lock, network stream, etc.).
    Borrowed(&'a mut dyn Write),
    /// Owned writer boxed as a trait object.
    #[allow(dead_code)]
    Owned(Box<dyn Write + 'a>),
}

impl<'a> RenderSink<'a> {
    fn write_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        match self {
            RenderSink::Buffer(buf) => {
                // SAFETY: input is valid UTF-8 since `bytes` originate from &str.
                if let Ok(text) = std::str::from_utf8(bytes) {
                    buf.push_str(text);
                    Ok(())
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "render target received non UTF-8 data",
                    ))
                }
            }
            RenderSink::Borrowed(writer) => writer.write_all(bytes),
            RenderSink::Owned(writer) => writer.write_all(bytes),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            RenderSink::Buffer(_) => Ok(()),
            RenderSink::Borrowed(writer) => writer.flush(),
            RenderSink::Owned(writer) => writer.flush(),
        }
    }
}

/// Streaming render target used throughout the visual module.
pub struct RenderTarget<'a> {
    sink: RenderSink<'a>,
    error: Option<io::Error>,
}

impl<'a> RenderTarget<'a> {
    /// Create a new in-memory render target without pre-allocated capacity.
    pub fn new() -> Self {
        Self::from_buffer(String::new())
    }

    /// Create a new in-memory render target with reserved capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_buffer(String::with_capacity(capacity))
    }

    fn from_buffer(buffer: String) -> Self {
        Self {
            sink: RenderSink::Buffer(buffer),
            error: None,
        }
    }

    /// Create a render target that borrows an external writer. Useful for
    /// streaming directly to stdout.
    pub fn from_writer<W>(writer: &'a mut W) -> Self
    where
        W: Write + 'a,
    {
        Self {
            sink: RenderSink::Borrowed(writer as &mut dyn Write),
            error: None,
        }
    }

    /// Create a render target that owns the provided writer by boxing it.
    #[allow(dead_code)]
    pub fn owned_writer<W>(writer: W) -> Self
    where
        W: Write + 'a,
    {
        Self {
            sink: RenderSink::Owned(Box::new(writer)),
            error: None,
        }
    }

    /// Append a full line (without newline) to the sink, then adds a newline.
    pub fn push_line(&mut self, line: &str) {
        if self.error.is_some() {
            return;
        }
        if let Err(err) = self.write_internal(line) {
            self.error = Some(err);
            return;
        }
        if let Err(err) = self.write_internal("\n") {
            self.error = Some(err);
        }
    }

    /// Append raw text without automatically adding a trailing newline.
    #[allow(dead_code)] // Temporary Vec adapter support; see TASKS [SP-2].
    pub fn push_raw(&mut self, text: &str) {
        if self.error.is_some() {
            return;
        }
        if let Err(err) = self.write_internal(text) {
            self.error = Some(err);
        }
    }

    /// The number of bytes currently stored in the underlying buffer.
    ///
    /// # Panics
    /// Panics if the render target is backed by an external writer.
    #[allow(dead_code)] // Temporary Vec adapter support; see TASKS [SP-2].
    pub fn len(&self) -> usize {
        match &self.sink {
            RenderSink::Buffer(buf) => buf.len(),
            _ => panic!("render target does not own an in-memory buffer"),
        }
    }

    /// Return a read-only view of the current buffer contents.
    ///
    /// # Panics
    /// Panics if the render target is backed by an external writer.
    #[allow(dead_code)] // Temporary Vec adapter support; see TASKS [SP-2].
    pub fn as_str(&self) -> &str {
        match &self.sink {
            RenderSink::Buffer(buf) => buf.as_str(),
            _ => panic!("render target does not own an in-memory buffer"),
        }
    }

    /// Consume the target and return the accumulated string.
    ///
    /// # Panics
    /// Panics if the render target is backed by an external writer or if an
    /// earlier write operation failed.
    pub fn into_string(self) -> String {
        if let Some(err) = self.error {
            panic!("render target recorded write error: {}", err);
        }
        match self.sink {
            RenderSink::Buffer(buf) => buf,
            _ => panic!("render target does not own an in-memory buffer"),
        }
    }

    /// Convenience helper to convert the buffer into individual lines.
    /// Primarily for compatibility with existing APIs that still expect
    /// `Vec<String>` responses.
    pub fn into_lines(self) -> Vec<String> {
        self.into_string()
            .lines()
            .map(|line| line.to_string())
            .collect()
    }

    /// Flush the underlying writer if necessary.
    #[allow(dead_code)]
    pub fn flush(&mut self) -> io::Result<()> {
        if let Some(err) = &self.error {
            return Err(io::Error::new(err.kind(), err.to_string()));
        }
        self.sink.flush()
    }

    /// Finalise the render target ensuring any deferred IO errors are surfaced.
    pub fn finish(mut self) -> io::Result<()> {
        if let Some(err) = self.error {
            return Err(err);
        }
        self.sink.flush()
    }

    fn write_internal(&mut self, text: &str) -> io::Result<()> {
        self.sink.write_bytes(text.as_bytes())
    }
}

impl<'a> fmt::Write for RenderTarget<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_internal(s).map_err(|_| fmt::Error)
    }
}
