pub enum Route {
    CalendarStart,
    CalendarEnd,
    Submit,
    Unhandled,
}

impl From<&str> for Route {
    fn from(s: &str) -> Self {
        match s {
            "/calendar_start" => Route::CalendarStart,
            "/calendar_end" => Route::CalendarEnd,
            "/submit" => Route::Submit,
            _ => Route::Unhandled,
        }
    }
}
