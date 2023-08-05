//! buf를 다루는 방법에 대해 자세하게 살펴본다.
//! bytes::{Buf, BufMut, Bytes, BytesMut}가 기본이 된다.
//! Buf, BufMut는 트레이트이고, Bytes, BytesMut는 struct이다.
//! 
//! Bytes, BytesMut가 struct이지만 vtable을 통해 구현을 변경할 수 있게 한다.

#[cfg(test)]
mod tests {
    use bytes::{Buf, BufMut, Bytes, BytesMut};

    #[test]
    fn bytes_module_usage() {
        // 싸게 복제 가능하고 조각으로 사용 가능한 연속적인 메모리 
        // 여러 vtable 구현이 있다. static 메모리, Arc<[u8]> 등 
        // ptr: *const u8을 갖는다. 
        
        let b1 = Bytes::new();
        assert_eq!(b1.len(), 0);

        // STATIC_VTABLE 버전의 Bytes
        let mut b2 = Bytes::from_static(b"hello");
        assert_eq!(b2.len(), 5);
        assert!(!b2.is_empty());

        let b3 = Bytes::copy_from_slice(&b"hello"[..]);
        assert_eq!(b3.len(), 5);

        // 내부 구현을 아직 모르는 부분이 있으나 대략 살피고 지나간다. 
        // 모든 코드를 다 이해할 수 있는 사람은 없다. 사용의 의미를 정확하게 파악한다. 

        // slice()는 clone()을 사용하고, RangeBound<usize> trait로 범위를 받는다.
        let sb1 = b2.slice(1..3);        
        assert_eq!(sb1.len(), 2);

        // slice_ref()는 &[u8]이 같은 Bytes에서 온 것으로 가정하고 
        // slice()로 얻은 것처럼 동일한 Bytes를 만든다. 
        // 포인터 주소를 사용하므로 빠르나 위험할 수도 있다. 

        let s2 = b2.split_off(3);
        assert_eq!(b2.len(), 3);
        assert_eq!(s2.len(), 2);
        assert_eq!(b2, Bytes::from_static(b"hel"));

        // split_at()는 남는 부분과 돌려주는 값이 split_off()와 거꾸로이다.
    }

    #[test]
    fn bytes_more() {
        let mut b = Bytes::from_static(b"hello world");
        b.truncate(5);
        assert_eq!(&b[..], b"hello");

        // with_vtable()은 unsafe 함수이다. 왜 그런가?  
        // drop() 등을 UB 없이 구현해야 하기때문이다. 

        // as_slice()는 unstable 한 상태이다. 
        // assert_eq!(b.as_slice(), b"hello");
    }

    #[test]
    fn buf_for_bytes() {
        // Buf 트레이트 구현을 확인한다. 

        let mut b1 = Bytes::from_static(b"hello world");
    }

    #[test]
    fn new_from_vec() {
        // new from Vec<u8> or &[u8]
        // copy_from_slice()로 할 수 있다. 
        let v = vec![0_u8, 1, 2, 3];
        let b1 = Bytes::copy_from_slice(&v);

        // From<Vec<u8>>의 구현은 러스트의 다양한 기본 기법들을 갖고 구현한다. 
        // mem::forget(), Box::into_raw() 두 가지. 
        // 그리고, Bytes에 고유한 SHARED_VTABLE 
        let b2 = Bytes::from(v);

        assert_eq!(b1, b2);

        // 지금은 정신 건강을 위해 VTable 구현들은 안 보도록 한다. 
        // 이런 기법을 써야 하는 경우들이 있을 수 있다. 성능에 매우 민감한 경우나 
        // trait로 구조가 잘 안 나오는 경우.
    }

    #[test]
    fn bytes_mut_usage() {
        // 연속된 메모리에 대한 고유한 참조를 갖는다. 어떻게 보장하는가?
        let mut b = BytesMut::with_capacity(1024);

        // 대부분의 유용한 기능은 BufMut 구현이다.
        let vs = vec![0_u8, 1, 2];
        b.put(vs.as_slice());
        b.put_u8(3);
    }

    #[test]
    fn bytes_mut_split_off() {
        // Bytes와 같으나 변경 가능한 두 개의 BytesMut로 만들고 
        // 쓸 수 있게 되면 문제가 될 것 같은데... 

        // split도 같다. 
        // Cow처럼 쓸 경우 분리되는 구현을 갖는 걸로 보인다. 하지만 어떻게?
        // extend_from_slice() 같은 함수에서 항상 reserve()로 체크한다. 
        // reserve() 함수에서 뭔가를 하지 않을까? 
        // self.data : Shared로 is_unique()를 체크한다. 

    }

    #[test]
    fn bytes_mut_reserve() {
        // 메모리 조작의 파티이다. 다채로운 메모리 기법들을 볼 수 있다. 
        // 정확하게 이해하자. 

        let mut buf = BytesMut::with_capacity(128);
        buf.put(&[0; 64][..]);
        
        let ptr = buf.as_ptr();
        let other = buf.split(); // shared now

        assert!(buf.is_empty());
        assert_eq!(buf.capacity(), 64); // shrunken, but same ptr

        drop(other); // shared. hence, memory is kept
        buf.reserve(128); // the whole original buffer is reused
        assert_eq!(buf.capacity(), 128);
        assert_eq!(ptr, buf.as_ptr()); // hence, the same memory

        // 
    }

    #[test]
    fn bytes_mut_more() {
        // extend_from_slice()는 데이터를 추가할 때 매우 좋다. 
    }
}
