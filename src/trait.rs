pub trait Adapt {
    fn adapt<const N: usize>(&self) -> [u8; N]
    where
        Self: AsRef<[u8]>,
    {
        assert!(
            self.as_ref().len() >= N,
            "read_bytes N={} data_len={}",
            N,
            self.as_ref().len()
        );
        let mut buffer = [0u8; N];
        buffer.copy_from_slice(&self.as_ref()[..N]);
        buffer
    }
}

impl<T> Adapt for T where T: AsRef<[u8]> {}

pub trait Pagible {
    /*
    async fn next_page(self) -> Option<ApiResult<Self>>
        where Self: Debug + DeserializeOwned;
    async fn prev_page(self) -> Option<ApiResult<Self>>
        where Self: Debug + DeserializeOwned;
    */
    fn has_next(&self) -> bool;
    fn has_prev(&self) -> bool;
    fn total(&self) -> u64;
    fn current(&self) -> u64;

    fn next(&self) -> u64 {
        if self.has_next() {
            self.current() + 1
        } else {
            self.current()
        }
    }

    fn prev(&self) -> u64 {
        if self.has_prev() {
            self.current() - 1
        } else {
            self.current()
        }
    }
}
