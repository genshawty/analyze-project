В работе старался каждый фикс в отдельный коммит выносить, чтобы было видно происходящее. 
Хочу отметить lib.rs::read_log функцию. 
```request_ids.is_empty() || request_ids.contains(&log.request_id)
        // подсказка: лучше match
        && match mode {}```
Работает с багом, который не проверяется тестами (если .is_empty() == true, то все остальное не проверяется), кажется логичнее и правильнее
```(request_ids.is_empty() || request_ids.contains(&log.request_id))
        // подсказка: лучше match
        && match mode {}```
Добавил тест на проверку этой логики. ```test_errors_with_empty_request_ids```.