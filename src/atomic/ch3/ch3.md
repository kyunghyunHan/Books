# 러스트의 메모리 순서와 동기화

## 1. 메모리 순서 기본 개념

### happens-before 관계
- 코드의 실행 순서를 보장하는 관계
```rust
fn example() {
    f();  // 이 함수가 먼저 실행됨이 보장
    g();  // 그 다음 실행
}
```

### 주요 특징
- 스레드 간에는 특정 상황에서만 happens-before 관계 적용
- 스레드 생성, 조인, 뮤텍스 작업, 아토믹 연산 등에서 적용
- 일반적인 코드 순서가 실제 실행 순서를 보장하지 않음

## 2. 메모리 순서 타입들

### Relaxed Ordering
```rust
use std::sync::atomic::{AtomicI32, Ordering};

let atomic = AtomicI32::new(0);
atomic.store(1, Ordering::Relaxed);  // 최소한의 보장
```
- 가장 기본적인 메모리 순서
- 최고의 성능 제공
- 순서 보장이 최소화됨

### Release-Acquire Ordering
```rust
// Thread 1
atomic.store(1, Ordering::Release);  // 저장 연산

// Thread 2
let val = atomic.load(Ordering::Acquire);  // 읽기 연산
```
특징:
- Release: 저장 작업에 사용
- Acquire: 읽기 작업에 사용
- happens-before 관계 형성

### SeqCst (Sequential Consistency)
```rust
atomic.store(1, Ordering::SeqCst);  // 가장 엄격한 순서 보장
```
- 모든 스레드에서 동일한 순서 보장
- 가장 직관적이지만 성능 비용 발생

## 3. 동기화 패턴

### Store-Load 패턴
```rust
// Thread 1
atomic.store(42, Ordering::Release);

// Thread 2
if atomic.load(Ordering::Acquire) == 42 {
    // Thread 1의 변경사항이 보장됨
}
```

### Consume-Load 패턴
```rust
// acquire-load보다 약한 보장
let value = atomic.load(Ordering::Consume);
```

## 4. 실제 사용 예시

### 데이터 동기화
```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

let ready = AtomicBool::new(false);

// 생산자 스레드
thread::spawn(move || {
    // 데이터 준비
    ready.store(true, Ordering::Release);
});

// 소비자 스레드
while !ready.load(Ordering::Acquire) {
    thread::yield_now();
}
// 데이터 사용 가능
```

## 5. 주요 고려사항

### 성능과 안전성 트레이드오프
1. Relaxed: 최고 성능, 최소 보장
2. Release-Acquire: 중간 수준의 보장과 성능
3. SeqCst: 최고 안전성, 성능 비용 발생

### 최적화 고려사항
- 컴파일러와 CPU는 성능 최적화를 위해 코드 재배치 가능
- 명시적인 메모리 순서가 필요한 경우에만 강한 순서 사용

## 핵심 요약

1. **기본 원칙**
   - 스레드 간 동기화는 명시적으로 지정 필요
   - happens-before 관계가 중요
   - 메모리 순서는 성능과 안전성의 균형

2. **선택 가이드**
   - 단순 연산: Relaxed
   - 데이터 동기화: Release-Acquire
   - 완벽한 순서 필요: SeqCst

3. **주의사항**
   - 과도한 제약은 성능 저하 초래
   - 너무 느슨한 순서는 버그 유발 가능
   - 상황에 맞는 적절한 순서 선택 중요