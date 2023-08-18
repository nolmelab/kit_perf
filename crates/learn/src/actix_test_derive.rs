//! actix의 test들을 읽고 실행하면서 이해한다. 
//! 첫 테스트로 derive.rs의 코드를 옮겨서 살펴본다.  
//! actix의 actor 구현을 보면 구현이 복잡하다. 필수적인 요소들로 
//! 구성된 것이 맞는지 확인하고 tokio나 async 틀의 발전을 반영해서 
//! 다시 알맞게 구현할 수 있는 방법을 찾아야 하는 것으로 보인다. 
//! 
//! async와 channel이 함께 섞여서 한번에 모두 이해하기 어렵다. 
//! 

#[cfg(test)]
mod tests {

    // macros feature가 활성화된 경우만 빌드하므로 제외
    // #![cfg(feature = "macros")]

    use std::ops::Mul;
    
    // 꽤 많은 구조를 접근 가능하게 한다. 
    use actix::prelude::*;
    
    // 메세지 proc 매크로로 정의한다. 
    // 러스트 코드로 TokenStream 상에서 매크로 구현을 한다. 
    // 아직 공부를 제대로 하지 않은 부분이다. 
    #[derive(Message)]
    #[rtype(result = "()")]
    struct Empty;
    
    struct EmptyActor;
    
    impl Actor for EmptyActor {
        // Context는 ContextParts<A>와 Mailbox로 구성된다. 
        // ContextParts는 contextimpl.rs에 있고, addr, flags, wait, items, handles를 갖고 있다. 
        type Context = Context<Self>;
    }
    
    // Empty 메세지 타잎에 대한 핸들러 구현이다. 
    // handler.rs에는 많은 Response 구조들이 정의되어 있다. 
    impl Handler<Empty> for EmptyActor {
        type Result = ();
    
        fn handle(&mut self, _message: Empty, _context: &mut Context<Self>) {}
    }
    
    #[test]
    #[allow(clippy::unit_cmp)]
    fn response_derive_empty() {
        // Systgem::new()는 SystemRunner를 만든다. 내부에서 SystemCommand 처리를 위한 
        // 채널을 만들고 default_tokio_runtime()으로 생성한다. 
        // Arbiter도 만들고, SystemController도 생성하여 spawn()한다.
        System::new().block_on(async {
            let addr = EmptyActor.start();

            // addr에서 AddressSender가 중요한 역할을 한다. 
            // oneshot channel, AtomicWaker 등을 사용한다. 
            let res = addr.send(Empty);
    
            // tokio::task::spawn_local()로 현재 쓰레드에 태스크를 만든다. 
            actix_rt::spawn(async move {
                // res.await를 해야 실제 전달되고 응답을 받는다.
                match res.await {
                    Ok(result) => assert!(result == ()),
                    _ => panic!("Something went wrong"),
                }
    
                System::current().stop();
            });
        });
    }  
}