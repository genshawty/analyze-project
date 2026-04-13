В работе старался каждый фикс в отдельный коммит выносить, чтобы было видно происходящее. 
Хочу отметить lib.rs::read_log функцию. 

```
request_ids.is_empty() || request_ids.contains(&log.request_id)
        // подсказка: лучше match
        && match mode {}
```
Работает с багом, который не проверяется тестами (если .is_empty() == true, то все остальное не проверяется), кажется логичнее и правильнее
```
(request_ids.is_empty() || request_ids.contains(&log.request_id))
        // подсказка: лучше match
        && match mode {}
```
Добавил тест на проверку этой логики. ```test_errors_with_empty_request_ids```.

Добавил бенчмарк на проверку, нужен ли синглетон для парсера ```bench_parser_build```.
Результаты: 
```
running 1 test
N = 1000000
cached: 817.474709ms total, 817.5 ns/iter
fresh : 833.571917ms total, 833.6 ns/iter```
Создание нового парсера каждый раз незначительно медленнее, но зато удобнее в дальнейшей разработке - поэтому меняем.
```

Изменение парсеров со String -> &str.
Изменение выполнено, кстати наглядно на нашем бенчмарке видно отличие:
```
N = 100000
cached: 15.555208ms total, 155.6 ns/iter
fresh : 19.049041ms total, 190.5 ns/iter
build-per-call overhead: ~34.9 ns, fresh is 1.22x cached
```