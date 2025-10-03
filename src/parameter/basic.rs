/*
MIT License

Copyright (c) 2022 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
/// Describes the necessary parameters for the [`crate::crack<T: CrackTarget>()`]-function
/// without the generic part that is outsourced to [`crate::TargetHashAndHashFunction`].
#[derive(Debug)]
pub struct BasicCrackParameter {
    /// all symbols (letters, digits, ...)
    alphabet: Box<[char]>,
    /// maximum crack length (to limit possible combinations)
    max_length: u32,
    /// minimum crack length (to limit possible combinations)
    min_length: u32,
    /// use n-1 threads to save system resources
    fair_mode: bool,
}

impl BasicCrackParameter {
    /// Constructor.
    #[must_use]
    pub const fn new(
        alphabet: Box<[char]>,
        max_length: u32,
        min_length: u32,
        fair_mode: bool,
    ) -> Self {
        Self {
            alphabet,
            max_length,
            min_length,
            fair_mode,
        }
    }

    #[must_use]
    pub const fn alphabet(&self) -> &[char] {
        &self.alphabet
    }
    #[must_use]
    pub const fn max_length(&self) -> u32 {
        self.max_length
    }
    #[must_use]
    pub const fn min_length(&self) -> u32 {
        self.min_length
    }
    #[must_use]
    pub const fn fair_mode(&self) -> bool {
        self.fair_mode
    }
}
