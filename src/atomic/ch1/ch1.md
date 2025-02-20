# 동시성

## 1. 스레드 타입과 범위

### 범위 스레드
- 특정 범위에서만 존재하는 스레드를 의미
- static이 아닌 라이프타임을 가진 값을 캡처 가능
- 범위를 벗어나면 자동으로 종료

### static 관련
- **static 변수**
  - 프로그램 시작부터 종료까지 존재
  - 모든 스레드에서 접근 가능
  - 프로그램이 직접 소유권 보유
  - 어떤 스레드보다 더 오래 존재

- **메모리 leak**
  ```rust
  // Box::leak 사용 예시
  let leaked = Box::leak(Box::new(42));
  // leaked는 프로그램 종료까지 존재
  ```

## 2. 소유권과 참조

### 참조 카운팅
- **Rc<T>**
  ```rust
  use std::rc::Rc;
  let data = Rc::new(vec![1, 2, 3]);
  let clone = Rc::clone(&data); // 참조 카운트만 증가
  ```
- **Arc<T>** 
  - 스레드 안전한 원자적 참조 카운팅
  - 멀티스레드 환경에서 데이터 공유에 사용

### 대여 타입
```rust
let mut value = 42;
let ref1 = &value;      // 불변 대여
let ref2 = &mut value;  // 가변 대여
```

## 3. 내부 가변성 타입들

### Cell
```rust
use std::cell::Cell;

let cell = Cell::new(1);
cell.set(2); // 불변 참조로도 값 변경 가능
```

### RefCell
```rust
use std::cell::RefCell;

let data = RefCell::new(vec![1, 2, 3]);
data.borrow_mut().push(4); // 런타임에 대여 규칙 검사
```

### RwLock (읽기-쓰기 락)
```rust
use std::sync::RwLock;

let lock = RwLock::new(5);
// 읽기 락
let r1 = lock.read().unwrap();
let r2 = lock.read().unwrap(); // 동시에 여러 읽기 가능

// 쓰기 락
let mut w = lock.write().unwrap(); // 독점적 접근
*w += 1;
```

### Mutex (상호 배제)
```rust
use std::sync::Mutex;

let mutex = Mutex::new(0);
{
    let mut num = mutex.lock().unwrap();
    *num += 1;
} // MutexGuard 드롭되면서 자동으로 잠금 해제
```

### Atomic (원자적 연산)
```rust
use std::sync::atomic::{AtomicI32, Ordering};

let counter = AtomicI32::new(0);
counter.fetch_add(1, Ordering::SeqCst);
```

## 4. 동기화 메커니즘

### 스레드 파킹
```rust
use std::thread;

thread::park(); // 현재 스레드 대기
thread::Thread::unpark(&thread); // 스레드 깨우기
```

### 조건 변수
```rust
use std::sync::{Mutex, Condvar};

let pair = (Mutex::new(false), Condvar::new());
let (lock, cvar) = &pair;

let mut started = lock.lock().unwrap();
while !*started {
    started = cvar.wait(started).unwrap();
}
```

## 5. 트레이트 구현

### Send와 Sync
```rust
// Send 예시
// Vec<T>는 T가 Send일 때 Send
impl<T: Send> Send for Vec<T> {}

// Sync 예시
// &T는 T가 Sync일 때 Send
impl<T: Sync> Send for &T {}
```

## 6. 스마트 포인터와 Deref

### Box<T>
```rust
let x = Box::new(42);
println!("{}", *x); // Deref 통해 자동 역참조
```

### Deref 구현
```rust
use std::ops::Deref;

impl<T> Deref for Box<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
```

## 주요 정리:

1. **동시성 처리 선택 가이드**:
   - 단순 카운터 → Atomic
   - 복잡한 데이터 공유 → Arc + Mutex/RwLock
   - 조건 기반 동기화 → Condvar + Mutex

2. **메모리 관리 패턴**:
   - 단일 소유권 → Box
   - 다중 소유권 (단일 스레드) → Rc
   - 다중 소유권 (멀티 스레드) → Arc

3. **내부 가변성 사용**:
   - 단일 스레드 → Cell/RefCell
   - 멀티 스레드 → Mutex/RwLock

4. **성능 최적화**:
   - 읽기 중심 → RwLock
   - 쓰기 중심 → Mutex
   - 단순 연산 → Atomic

