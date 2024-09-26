pub trait SliceExt<T> {
    fn get_n<const N: usize>(&self, offset: usize) -> Option<&[T; N]>;
}

pub trait ByteSliceExt {
    fn read_u16_be(&self, offset: usize) -> Option<u16>;
}

impl<T> SliceExt<T> for [T] {
    fn get_n<const N: usize>(&self, offset: usize) -> Option<&[T; N]> {
        self.get(offset..).and_then(<[_]>::first_chunk::<N>)
    }
}

impl ByteSliceExt for [u8] {
    fn read_u16_be(&self, offset: usize) -> Option<u16> {
        self.get_n(offset).map(|arr| u16::from_be_bytes(*arr))
    }
}
