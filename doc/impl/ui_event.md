# ui & event impl note 

- std::sync::mpsc 사용할 때 try_recv()로 non-blocking 호출 

- echo 표시가 매우 느린 문제 
  - client 연결 종료 한참 후까지 표시되는 문제 
  - sleep를 제거해도 그런 문제가 있다. 

- Terminated() 표시 
  - 라따뚜이 ui가 종료하면서 나온 메세지이다. 
  - draw() 호출할 때 에러가 나오는 경우가 있다. 
    - 왜 그런지는 에러 메세지를 보면 자세히 알 수 있을 것이다. 

- 러스트의 에러 처리 
  - Result의 경우 에러 처리가 가능하거나 표시할 수 있으면 해당 지정에서 처리한다. 
    - ?로 리턴하면 편하지만 처리를 해야 견고해진다. 

실시간 gui로 표시하는 건 좋은 선택이 아닌 듯 하다. 로그나 tracing 형태로 깔끔하게 
표시되도록 하고, 결과 리포트를 잘 수집해서 보여주는 방향이 맞을 듯 하다. 
