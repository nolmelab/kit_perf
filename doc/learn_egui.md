# learn egui 

egui는 rust의 imgui이다. 간단하고 쉽게 툴에 사용할 수 있다. imgui가 unity에 쓰이면서 
유명해졌는데 egui는 동일한 immediate mode gui로 접근한다. 

기능과 통합된 gui를 만들기 편리하다. 

## 흐름 

eframe::App을 impl 하는 MyApp을 만든다. 
eframe::App 트레이트는 update를 갖는다. 

update에서 gui 판넬 등 컨테이너에 대해 ui 항목을 FnOnce로 지정한다. 

## 연습 

- Tabbed Control 
  - Plot 
  - Inputs 
  - Button


