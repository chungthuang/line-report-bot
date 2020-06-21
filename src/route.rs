pub enum Route {
    CalendarStart,
    CalendarEnd,
    Webhooks,
    Submit,
    Unhandled,
}

impl From<&str> for Route {
    fn from(s: &str) -> Self {
        match s {
            "/calendar_start" => Route::CalendarStart,
            "/calendar_end" => Route::CalendarEnd,
            "/webhooks" => Route::Webhooks,
            "/submit" => Route::Submit,
            _ => Route::Unhandled,
        }
    }
}
