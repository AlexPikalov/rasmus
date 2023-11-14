pub trait BytesGetter<B> {
    fn get_bytes(data: &Vec<u8>) -> B;
}

pub struct MemoryBytesGetter;

impl BytesGetter<[u8; 1]> for MemoryBytesGetter {
    fn get_bytes(data: &Vec<u8>) -> [u8; 1] {
        [data[0]; 1]
    }
}

impl BytesGetter<[u8; 2]> for MemoryBytesGetter {
    fn get_bytes(data: &Vec<u8>) -> [u8; 2] {
        let mut arr = [0u8; 2];
        for b in 0..2 {
            arr[b] = data[b];
        }
        arr
    }
}

impl BytesGetter<[u8; 4]> for MemoryBytesGetter {
    fn get_bytes(data: &Vec<u8>) -> [u8; 4] {
        let mut arr = [0u8; 4];
        for b in 0..4 {
            arr[b] = data[b];
        }
        arr
    }
}

impl BytesGetter<[u8; 8]> for MemoryBytesGetter {
    fn get_bytes(data: &Vec<u8>) -> [u8; 8] {
        let mut arr = [0u8; 8];
        for b in 0..8 {
            arr[b] = data[b];
        }
        arr
    }
}

impl BytesGetter<[u8; 16]> for MemoryBytesGetter {
    fn get_bytes(data: &Vec<u8>) -> [u8; 16] {
        let mut arr = [0u8; 16];
        for b in 0..16 {
            arr[b] = data[b];
        }
        arr
    }
}

impl BytesGetter<Vec<u8>> for MemoryBytesGetter {
  fn get_bytes(data: &Vec<u8>) -> Vec<u8> {
      let mut arr = [0u8; 16];
      for b in 0..16 {
          arr[b] = data[b];
      }
      arr.to_vec()
  }
}
