# learn egui 

egui는 rust의 imgui이다. 간단하고 쉽게 툴에 사용할 수 있다. imgui가 unity에 쓰이면서 
유명해졌는데 egui는 동일한 immediate mode gui로 접근한다. 

기능과 통합된 gui를 만들기 편리하다. 

## 흐름 

eframe::App을 impl 하는 MyApp을 만든다. 
eframe::App 트레이트는 update를 갖는다. 

update에서 gui 판넬 등 컨테이너에 대해 ui 항목을 FnOnce로 지정한다. 

Frame, Context, Ui가 중요한 클래스이다. show()에서 여러 가지를 점검해서 
계층적인 Ui를 구성한다. 여기가 Immediate 한 부분이다. 

### Context

context.rs의 내용을 훑어 본다. 

- WrappedTextureManager 
  - 폰트 텍스처를 만들고, 다른 텍스처들도 처리하는 듯 
  - Arc<RwLock<epaint::TextureManager>> 이다. 
    - thread-safety를 고려한 UI 프레임워크라 게임 개발에도 도움이 될 듯 

- Repaint 
  - Option<Box<dyn Fn(RequestRepaintInfo) + Send + Sync>>
    - Fn trait object를 여러 곳에서 사용한다 
    - Rust에서 실행시간 다형성을 제공하는 방법 중 하나이다 (다른 하나는 vtable을 
      직접 구성하는 방법이 있다)

  - Default 트레이트 구현은 생성을 편하게 하고, 따라서 사용도 편해진다. 

- ContextImpl 
  - fonts, memory, animation_manager, tex_manager, os, 
  - input, frame_state, graphics, output, paint_stats
  - repaint, layer_rects_this_frame, layer_rects_prev_frame
  - impl: 
    - begin_frame_mut() 
    - update_fonts_mut() 

- Context 
  - Arc<RwLock<ContextImpl>>
  - impl: 
    - read<R>(&self, reader: impl FnOnce(&ContextImpl) -> R) -> R 
    - write<R>(&self, writer: impl FnOnce(&mut ContextImpl) -> R) -> R
    - run(&self, new_input: RawInput, run_ui: impl FnOnce(&Context)) -> FullOutput 
    - begin_frame()
    - input<R>()
    - input_mut<R>()
    - memory<R>()
    - memory_mut<R>()
    - data<R>()
    - data_mut<R>()
    - graphics_mut<R>()
    - output<R>()
    - output_mut<R>()
    - from_state<R>()
    - from_state_mut<R>()
    - fonts<R>()
    - ...

Fn / FnOnce / FnMut 트레이트를 매우 많이 사용하고 있다. 내부의 제어를 외부에서 제공하는 
측면에서 Dependency Injection과 유사하지만 매우 명시적이다 (동적인 타잎 계산에 의존하지 않음)

Context는 Ui 들을 그리고 입력을 처리하기 위한 모든 상태를 갖고 있다. 물론 Ui들도 
여러 가지 상태를 갖지만 특정 플래폼에서 그리고 입력을 처리하고 상태를 조회하고 
지정할 수 있는 기능을 제공한다. 

tessellator.rs를 살펴보면 대수적 타잎인 enum과 Composition으로 많은 일들을 명시적으로 
할 수 있다는 걸 알 수 있다. OOP의 virtual이 능사가 아니다. 더 이해하기 어려울 때도 
많다. 

- begin_frame()
- end_frame()
- run()
- tesselate() 

위 외에도 중요한 함수들이 많다. 결국 지원하는 모양으로 Ui를 잘 구성하면 된다. 

### Ui

소중한 Ui이다. 

- Ui
  - Id
  - Painter 
  - Arc<Style> 
    - Copy on Write를 직접 구현한다. 
  - Placer 
  - menu_state: Option<Arc<RwLock<MenuState>>>

  - impl:
    - new(ctx: Context, ... )
    - child_ui()
    - ...
    - 

Ui는 개별 Ui의 배치, 그리기 등을 처리한다. 

checkbox(), button() 등 위젯 함수들도 포함하고 있다. 좋은 선택인가? 
아마도 쉽고 일관되게 쓸 수 있게 하려는 의도로 보인다. 

Checkbox::new(checked, text).ui(&mut parent_ui)로 항상 외부에서 별도로 
만들수도 있다. 

러스트 코드는 일관성이 있고 이해 못할 부분이 거의 없어져서 매우 좋다. 
좀만 더 가면 된다. 



## 연습 

  - Menu 
  - Window 
  - Plot 
  - Inputs 
  - Button


