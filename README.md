# lemonadern's Rust Playground

## Contents

- 単方向連結リスト
  - 目的：参照カウンタと内部可変性パターンの練習
  - headのみを保持する縛りで実装（sizeやtailは持たない）
  - [ソースコード](./src/linked_list.rs)
  - [テスト](./tests/linked_list.rs)
  - 機能
    - `push_front`
    - `pop_front`
    - `push_back`
    - `pop_back`
    - `insert`
  - 今後実装するかもしれない機能
    - From, Into
    - `remove` by index
    - `len`
    - `is_empty`
    - `iter`
    - `append` (concatenation)
    - `clear` (removes all elements)
  - 機能命名の参考： [`std::collections::LinkedList`](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)
- **WIP**: other contents