// Пусть есть логи:
// System(requestid):
// - trace
// - error
// App(requestid):
// - trace
// - error
// - journal (человекочитаемая сводка)

// Есть прототип штуки, которая умеет:
// - парсить логи
// - фильтровать
//  -- по requestid
//  -- по ошибкам
//  -- по изменению счёта (купить/продать)

use std::path::PathBuf;

use analysis::parse::Announcements;
use analysis::{ReadMode, read_log};

// Модель данных:
// - Пользователь (userid, имя)
// - Вещи
//  -- Предмет (assetid, название)
//  -- Набор (assetid, количество)
//      comment{-- Собственность (assetid, userid владельца, количество)}
//  -- Таблица предложения (assetid на assetid, userid продавца)
//  -- Таблица спроса (assetid на assetid, userid покупателя)
// - Операция App
//  -- Journal
//   --- Создать пользователя userid с уставным капиталом от 10usd и выше
//   --- Удалить пользователя
//   --- Зарегистрировать assetid с ликвидностью от 50usd
//   --- Удалить assetid (весь asset должен принадлежать пользователю)
//   --- Внести usd для userid (usd (aka доллар сша) - это тип asset)
//   --- Вывести usd для userid
//   --- Купить asset
//   --- Продать asset
//  -- Trace
//   --- Соединить с биржей
//   --- Получить данные с биржи
//   --- Локальная проверка корректности (упреждение ошибок в ответе)
//   --- Отправить запрос в биржу
//   --- Получить ответ от биржи
//  -- Error
//   --- нет asset
//   --- системная ошибка
// - Операция System
//  -- Trace
//   --- Отправить запрос
//   --- Получить ответ
//  -- Error
//   --- нет сети
//   --- отказано в доступе
const PARSING_DEMO: &str =
    r#"[UserBackets{"user_id":"Bob","backets":[Backet{"asset_id":"milk","count":3,},],},]"#;

fn main() -> std::process::ExitCode {
    println!("Placeholder для экспериментов с cli");

    let announcements = match analysis::parse::just_parse::<Announcements>(PARSING_DEMO) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("demo parse failed: {e:?}");
            return std::process::ExitCode::FAILURE;
        }
    };
    println!("demo-parsed: {announcements:?}");

    let Some(filename) = std::env::args_os().nth(1).map(PathBuf::from) else {
        eprintln!("usage: analysis <logfile>");
        return std::process::ExitCode::FAILURE;
    };

    let cwd = std::env::current_dir().expect("invalid cwd");
    println!(
        "Trying opening file '{}' from directory '{}'",
        filename.display(),
        cwd.display()
    );

    let file = match std::fs::File::open(&filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("failed to open {}: {e}", filename.display());
            return std::process::ExitCode::FAILURE;
        }
    };

    println!("got logs:");
    for log in read_log(file, ReadMode::ReadModeAll, vec![]) {
        println!("  {log:?}");
    }

    std::process::ExitCode::SUCCESS
}
