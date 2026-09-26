use thiserror::Error;

/// Errors for route of http
#[derive(Debug, Error)]
pub enum RouteError {
    #[error("ошибка ввода-вывода: {0}")]
    Io(#[from] std::io::Error),

    #[error("некорректная статус-строка ответа")]
    InvalidStatusLine,

    #[error("некорректный заголовок ответа: {0}")]
    InvalidHeader(String),

    #[error("соединение закрыто раньше, чем получен полный ответ")]
    UnexpectedEof,

    #[error("невалидная UTF-8 последовательность: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}
