use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Success,
    Fail,
    Error,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse<D>
where
    D: Serialize,
{
    status: ResponseStatus,
    data: D,
}

impl<D> SuccessResponse<D>
where
    D: Serialize,
{
    pub fn new(data: D) -> Self
    where
        D: Serialize,
    {
        SuccessResponse {
            status: ResponseStatus::Success,
            data,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FailResponse<D>
where
    D: Serialize,
{
    status: ResponseStatus,
    data: D,
}

impl<D> FailResponse<D>
where
    D: Serialize,
{
    pub fn new(data: D) -> Self
    where
        D: Serialize,
    {
        FailResponse {
            status: ResponseStatus::Fail,
            data,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse<D>
where
    D: Serialize,
{
    status: ResponseStatus,
    message: String,
    data: Option<D>,
}

impl<D> ErrorResponse<D>
where
    D: Serialize,
{
    pub fn new(message: String, data: Option<D>) -> Self
    where
        D: Serialize,
    {
        ErrorResponse {
            status: ResponseStatus::Error,
            message,
            data,
        }
    }
}
