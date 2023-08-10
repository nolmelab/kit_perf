//! https://github.com/actix/actix
//! 
//! actix-web이 아닌 actor 프레임워크이다. 8K 별을 받은 프로젝트이다. 
//! 러스트에 다른 액터 프레임워크들도 상당히 있다. ractor가 그 중 하나이다. 
//! actix actor를 공부하는 이유는 통신과 처리 구조를 tokio 하에서 
//! 잘 정리하고, 실제 구현도 이해하는데 있다. 만약 액터 틀로 MMORPG까지 
//! 포괄할 수 있다면 매우 좋은 일이다. MMORPG에서는 동시성을 올리는 일이 
//! 매우 중요하고 액터로 가능하다면 구현이나 실행 안정성 등이 좋아진다. 
//!
//! actix-web이 actix에 기반하여 만들어졌다. 성능이 좋고, 웹으로 
//! 많이 사용했을 터이니 더 좋다. 
//!
//! 학습 모듈들: 
//! - actix_start 
//! - 

mod actix_start;

// tokio::main으로 하면 task::LocalSet 바깥에서 spawn_local을 호출 했다는 오류가 나온다. 
// actix_rt를 사용해야 한다. tokio와 차이는 무엇이고 어떻게 사용할 수 있는가?
// https://docs.rs/actix-rt/latest/actix_rt/struct.Runtime.html
// - 모든 퓨처가 스폰한 쓰레드에서 실행된다. 따라서, Send가 불필요하지만 
// - 블럭킹 호출을 가질 수 없는 한계가 있다. tokio task와 섞어서 써야 한다. 
// - 확실히 멀티쓰레드 처리와 통신은 생각해야 할 분기가 많다.
#[actix_rt::main]
async fn main() {
    actix_start::main().await;
}

