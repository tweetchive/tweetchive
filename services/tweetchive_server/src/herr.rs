use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use color_eyre::Report;

pub type HResult<T> = Result<T, HErr>;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum HErr {
    StatusCode(StatusCode),
    Report(Report),
}

impl From<StatusCode> for HErr {
    fn from(sc: StatusCode) -> Self {
        HErr::StatusCode(sc)
    }
}

impl From<Report> for HErr {
    fn from(rep: Report) -> Self {
        HErr::Report(rep)
    }
}

impl IntoResponse for HErr {
    fn into_response(self) -> Response {
        match self {
            HErr::StatusCode(sc) => sc.into_response(),
            HErr::Report(report) => {
                (StatusCode::INTERNAL_SERVER_ERROR, report.to_string()).into_response()
            }
        }
    }
}
