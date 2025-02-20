# 러스트의 Atomic 연산과 메모리 순서

## 1. Atomic 기본 개념
```rust
use std::sync::atomic::{AtomicI32, Ordering};

let counter = AtomicI32::new(0);
```

### 핵심 특징
- 연산이 원자적으로 실행됨 (중간 상태 없음)
- 멀티스레드 환경에서 안전하게 공유 가능
- 락 없이도 스레드 간 데이터 공유 가능

## 2. 메모리 순서 (Memory Ordering)

### Relaxed
```rust
counter.store(1, Ordering::Relaxed);
// 최소한의 보장만 제공
// 단순 실행만 보장
```

### Acquire
```rust
let value = counter.load(Ordering::Acquire);
// 다른 스레드의 변경사항 확인 후 실행
```

### Release
```rust
counter.store(2, Ordering::Release);
// 현재 변경사항이 다른 스레드에 보일 때까지 대기
```

### AcqRel (Acquire + Release)
```rust
counter.fetch_add(1, Ordering::AcqRel);
// 양방향 동기화 보장
```

### SeqCst (Sequential Consistency)
```rust
counter.fetch_add(1, Ordering::SeqCst);
// 가장 엄격한 순서 보장
// 안정성이 중요할 때 권장
```

## 3. 주요 Atomic 연산

### Load 연산
```rust
let current = counter.load(Ordering::SeqCst);
// 현재 값을 원자적으로 읽기
```

### Store 연산
```rust
counter.store(5, Ordering::SeqCst);
// 새 값을 원자적으로 저장
```

### Compare and Exchange
```rust
let result = counter.compare_exchange(
    expected,    // 예상되는 현재 값
    new_value,   // 설정하려는 새 값
    Ordering::SeqCst,
    Ordering::SeqCst
);
// 값이 expected와 같을 때만 new_value로 업데이트
```

## 4. 사용 시나리오

### 적합한 경우
- 단순 카운터 구현
- 플래그 변수 관리
- 스레드 간 간단한 신호 전달

### 부적합한 경우
- 복잡한 데이터 구조
- 여러 필드의 동시 업데이트
- 트랜잭션이 필요한 연산

## 5. 구현 예시

### 카운터 구현
```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

let counter = AtomicUsize::new(0);
let counter_ref = &counter;

// 여러 스레드에서 안전하게 카운터 증가
let handles: Vec<_> = (0..10).map(|_| {
    thread::spawn(move || {
        counter_ref.fetch_add(1, Ordering::SeqCst);
    })
}).collect();

// 모든 스레드 완료 대기
for handle in handles {
    handle.join().unwrap();
}
```

## 핵심 정리

1. **Atomic 특성**
   - 연산은 분할 불가능
   - 완료 또는 미실행 상태만 존재
   - 스레드 간 안전한 데이터 공유

2. **Ordering 선택**
   - 일반적으로 SeqCst 사용 권장
   - 성능이 중요한 경우 더 약한 순서 고려

3. **최적 사용 사례**
   - 단순 카운터
   - 플래그
   - 간단한 공유 변수

4. **주의사항**
   - 복잡한 데이터는 Mutex 사용 권장
   - 적절한 메모리 순서 선택 중요

이렇게 정리해보았습니다. Atomic 연산의 핵심 개념과 실제 사용 예시를 포함했습니다.