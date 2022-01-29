/// Describes the necessary parameters for the [`crate::crack`]-function
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
    pub fn new(alphabet: Box<[char]>, max_length: u32, min_length: u32, fair_mode: bool) -> Self {
        Self {
            alphabet,
            max_length,
            min_length,
            fair_mode,
        }
    }

    pub fn alphabet(&self) -> &[char] {
        &self.alphabet
    }
    pub fn max_length(&self) -> u32 {
        self.max_length
    }
    pub fn min_length(&self) -> u32 {
        self.min_length
    }
    pub fn fair_mode(&self) -> bool {
        self.fair_mode
    }
}
