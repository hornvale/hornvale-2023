/// The limit we'll apply to the call frame stack.
pub const CALL_FRAMES_MAX: usize = 64;

/// The limit we'll apply to the stack.
/// 64 * 256 = 16,384 bytes.
pub const STACK_SIZE_MAX: usize = CALL_FRAMES_MAX * 256;
