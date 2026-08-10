// 1. Подключаем модуль aluzin.rs и импортируем макросы
#[macro_use]
mod aluzin;

// 2. Выводим функцию в корень, чтобы макрос увидел её через $crate::extract_block
pub use aluzin::extract_block;

// 3. Вызываем макрос напрямую
artm!("template.alzn");

fn main() {
    // 4. Получаем сгенерированный HTML
    let html_content = render();

    // 5. Запускаем сервер
    aluzin::start_server(html_content);
}
