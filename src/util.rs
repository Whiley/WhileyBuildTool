use std::str::Chars;

// ===================================================================
// Line Abstraction
// ===================================================================
#[derive(Debug)]
pub struct Line<'a> {
    /// String being iterated
    contents: &'a str,
    /// Starting offset of current line
    pub start: usize,
    /// Offset of newline terminator for current line, or
    /// (if none exists) matches length of contents
    pub end: usize
}

impl<'a> Line<'a> {
    pub fn new(contents: &'a str, start: usize, end: usize) -> Self {
	Line{contents,start,end}
    }
    /// Extract a slice representing the contents of the current line.
    /// Observe that this does not include line termators (e.g. `LR`
    /// or `CRLF`).
    pub fn as_str(&self) -> &'a str {
	// Extract slice
	&self.contents[self.start..self.end]
    }

    /// Check whether a given offset in the enclosing string is within
    /// this line or not.
    pub fn contains(&self,offset: usize) -> bool {
	offset >= self.start && offset < self.end
    }
}

// ===================================================================
// Line Iterator
// ===================================================================
pub struct LineIter<'a> {
    /// String being iterated
    contents: &'a str,
    /// Underlying character iterator
    iter: Chars<'a>,
    /// starting offset of current line
    offset: usize
}

impl<'a> LineIter<'a> {
    // Construct a new line iterator from
    pub fn new(contents: &'a str) -> Self {
	let iter = contents.chars();
	LineIter{contents,iter,offset:0}
    }
}

impl<'a> Iterator for LineIter<'a> {
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
	let start = self.offset;
	let mut cr;
	// Perform initial match such that, if the underlying iterator
	// returns None straight away, we return None.
	match self.iter.next() {
	    Some('\n') => {
		// Update offset position
		self.offset = self.offset + 1;
		// Done
		return Some(Line::new(self.contents,start,start));
	    }
	    Some(x) => { cr = x == '\r'; }
	    None => { return None; }
	}
	// Skip over character just parsed
	self.offset = self.offset + 1;
	let mut end = start + 1;
	// Advance until eiher a LF is found, or we reach the end of
	// the iterator.
	loop {
	    // Skip over character about to parse
	    self.offset = self.offset + 1;
	    // Parse it
	    match self.iter.next() {
		Some('\n') => {
		    if cr { end = end - 1; }
		    break;
		}
		Some(x) => { cr = x == '\r'; }
		None => break,
	    }
	    end = end + 1;
	}
	// Done
	Some(Line::new(self.contents,start,end))
    }
}

/// Construct a line offset iterator from a given string slice.
/// Something tells me could do this more nicely using IntoIterator or
/// something.
pub fn line_offsets<'a>(contents: &'a str) -> LineIter<'a> {
    LineIter::new(contents)
}
