# 메모리 순서

## 메모리 순서 재정렬 최적화


- 느슨한 순서:Relaxed
- 순서의 해제와 획득:Release,Acquire,AcqRel
- SeqCst


## happens-before관계

- f(); g();가 있다면 f가 실행된 다음 g가 실행
- 스레드 사이에서는 특정상황에서만 적용
- Relaxed에서는 메모리 순서가 아닌 스레드 생성,조인 ,뮤텍스 잠금과 해제,아토믹 연산과 같으 ㄴ경우
- Relaxed메모리 순서는 가장 기본적이고 가장 성능이 뛰어난 순서

## 해제 순서와 획득 순서
- happens-before관계를 만들기 위해서는 해제 순서와 획득순서를 사용할수 있다.
- Release메모리 순서는 저장연산
- Acquire메모리 순서는 읽기 연산
- happens-before관계는 acquire-load연산이 release-store연산의 결과를 관찰할떄 만들어진다.