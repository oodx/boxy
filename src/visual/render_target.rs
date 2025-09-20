//! Render target buffer for Boxy output.
//!
//! Renders stream directly into a single growable buffer, avoiding the need to
//! accumulate temporary `Vec<String>` allocations while still allowing callers
//! to collect lines when necessary.

#[derive(Default, Debug)]
pub struct RenderTarget {
    buffer: String,
}

impl RenderTarget {
    /// Create a new render target with no pre-allocated capacity.
    #[allow(dead_code)] // Temporary Vec adapter support; see TASKS [SP-2].
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new render target with an estimated capacity to limit
    /// reallocation as lines are appended.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
        }
    }

    /// Append a full line (without newline) to the buffer.
    pub fn push_line(&mut self, line: &str) {
        self.buffer.push_str(line);
        self.buffer.push('\n');
    }

    /// Append raw text without automatically adding a trailing newline.
    #[allow(dead_code)] // Temporary Vec adapter support; see TASKS [SP-2].
    pub fn push_raw(&mut self, text: &str) {
        self.buffer.push_str(text);
    }

    /// The number of bytes currently stored in the buffer.
    #[allow(dead_code)] // Temporary Vec adapter support; see TASKS [SP-2].
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Return a read-only view of the current buffer contents.
    #[allow(dead_code)] // Temporary Vec adapter support; see TASKS [SP-2].
    pub fn as_str(&self) -> &str {
        &self.buffer
    }

    /// Consume the target and return the accumulated string.
    pub fn into_string(self) -> String {
        self.buffer
    }

    /// Convenience helper to convert the buffer into individual lines.
    ///
    /// This is primarily for compatibility with existing APIs that still
    /// expect `Vec<String>` responses.
    #[allow(dead_code)] // Temporary Vec adapter support; see TASKS [SP-2].
    pub fn into_lines(self) -> Vec<String> {
        self.buffer.lines().map(|line| line.to_string()).collect()
    }
}

impl std::fmt::Write for RenderTarget {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.push_str(s);
        Ok(())
    }
}
