# Rust의 Arc<T>와 약한 참조 패턴 가이드

## 1. Arc<T> (Atomic Reference Counting)

### 기본 개념
```rust
use std::sync::Arc;

let data = Arc::new(vec![1, 2, 3]);
let clone = Arc::clone(&data);  // 참조 카운트 증가
```

- 스레드 간 안전한 공유를 위한 원자적 참조 카운팅
- `Rc<T>`의 스레드 안전 버전
- 참조 카운트가 0이 되면 자동으로 메모리 해제

## 2. 약한 참조 패턴들

### Weak<T>
```rust
use std::sync::{Arc, Weak};

let strong = Arc::new(String::from("Hello"));
let weak = Arc::downgrade(&strong);  // 약한 참조 생성

// 약한 참조 사용
if let Some(upgraded) = weak.upgrade() {
    println!("Value still exists: {}", upgraded);
}
```

### NonNull<T>
```rust
use std::ptr::NonNull;

let ptr = NonNull::new(Box::into_raw(Box::new(42)))
    .expect("Failed to create NonNull pointer");
```

### ManuallyDrop<T>
```rust
use std::mem::ManuallyDrop;

let data = ManuallyDrop::new(String::from("Don't drop me yet"));
// 명시적으로 drop 호출 필요
```

## 3. 순환 참조 해결

### 약한 참조 사용
```rust
struct Node {
    next: Option<Arc<Node>>,
    prev: Option<Weak<Node>>,  // 순환 참조 방지
}
```

## 4. 메모리 관리 패턴

### 강한 참조와 약한 참조 조합
```rust
struct Cache {
    data: Arc<String>,
    weak_ref: Weak<String>,
}

impl Cache {
    fn new(data: String) -> Self {
        let arc = Arc::new(data);
        let weak = Arc::downgrade(&arc);
        Cache {
            data: arc,
            weak_ref: weak,
        }
    }
}
```

## 핵심 정리

1. **Arc<T> 특징**
   - 스레드 안전한 참조 카운팅
   - 원자적 연산 사용
   - 스레드 간 데이터 공유에 적합

2. **약한 참조 사용 시나리오**
   - 순환 참조 방지
   - 캐시 구현
   - 임시 참조 관리

3. **메모리 안전성**
   - NonNull을 통한 null 안전성 보장
   - ManuallyDrop으로 명시적 메모리 관리
   - 순환 참조 방지를 통한 메모리 누수 예방

4. **모범 사례**
   - 적절한 참조 타입 선택
   - 순환 참조 주의
   - 메모리 누수 방지를 위한 설계

이러한 패턴들을 적절히 활용하면 안전하고 효율적인 메모리 관리가 가능합니다.