use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub struct LogBytes<T: AsRef<[u8]>>(pub T);
impl<T: AsRef<[u8]>> std::fmt::Display for LogBytes<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", STANDARD.encode(&self.0))
    }
}
