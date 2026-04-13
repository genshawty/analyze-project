В работе старался каждый фикс в отдельный коммит выносить, чтобы было видно происходящее.

## `lib.rs::read_log`

Хочу отметить функцию `lib.rs::read_log`.

```rust
request_ids.is_empty() || request_ids.contains(&log.request_id)
    && match mode {}
```

Работает с багом, который не проверяется тестами (если `.is_empty() == true`, то всё остальное не проверяется). Кажется логичнее и правильнее:

```rust
(request_ids.is_empty() || request_ids.contains(&log.request_id))
    && match mode {}
```

Добавил тест на проверку этой логики — `test_errors_with_empty_request_ids`.

## Бенчмарк синглтона парсера

Добавил бенчмарк на проверку, нужен ли синглтон для парсера — `bench_parser_build`.

Результаты:

```
running 1 test
N = 1000000
cached: 817.474709ms total, 817.5 ns/iter
fresh : 833.571917ms total, 833.6 ns/iter
```

Создание нового парсера каждый раз незначительно медленнее, но зато удобнее в дальнейшей разработке — поэтому меняем.

## Парсеры: `String` → `&str`

Изменение парсеров со `String` на `&str`. Изменение выполнено, кстати наглядно на нашем бенчмарке видно отличие:

```
N = 1000000
cached: 139.700458ms total, 139.7 ns/iter
fresh : 175.077291ms total, 175.1 ns/iter
build-per-call overhead: ~35.4 ns, fresh is 1.25x cached
```
