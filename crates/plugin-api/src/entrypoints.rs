use crate::error::Result;
use crate::io::{read_json_from_stdin, write_json_to_stdout};
use crate::protocol::{
    AnimationFrame, InfoPluginRequest, InfoPluginResponse, LogoAnimationRequest,
    LogoAnimationResponse,
};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, PartialEq)]
pub struct InfoPluginInput<T> {
    pub request: InfoPluginRequest,
    pub args: Option<T>,
}

impl<T> InfoPluginInput<T> {
    pub fn into_args(self) -> Option<T> {
        self.args
    }

    pub fn args_ref(&self) -> Option<&T> {
        self.args.as_ref()
    }

    pub fn into_parts(self) -> (InfoPluginRequest, Option<T>) {
        (self.request, self.args)
    }

    pub fn args_or_default(self) -> T
    where
        T: Default,
    {
        self.args.unwrap_or_default()
    }
}

pub fn read_logo_animation_request() -> Result<LogoAnimationRequest> {
    let request: LogoAnimationRequest = read_json_from_stdin()?;
    request.validate()?;
    Ok(request)
}

pub fn read_info_plugin_request<T: DeserializeOwned>() -> Result<InfoPluginInput<T>> {
    let request: InfoPluginRequest = read_json_from_stdin()?;
    request.validate()?;
    let args = request.parse_args()?;

    Ok(InfoPluginInput {
        request,
        args,
    })
}

pub fn read_info_plugin_args_or_default<T>() -> Result<T>
where
    T: DeserializeOwned + Default,
{
    Ok(read_info_plugin_request::<T>()?.args_or_default())
}

pub fn write_logo_animation_frames(frames: Vec<AnimationFrame>) -> Result<()> {
    let response = LogoAnimationResponse::new(frames);
    response.validate()?;
    write_json_to_stdout(&response)
}

pub fn write_info_lines(lines: Vec<String>) -> Result<()> {
    write_json_to_stdout(&InfoPluginResponse::new(lines))
}

#[cfg(test)]
mod tests {
    use super::InfoPluginInput;
    use crate::protocol::InfoPluginRequest;

    #[test]
    fn info_input_returns_default_args() {
        let input = InfoPluginInput::<Vec<String>> {
            request: InfoPluginRequest::new(None),
            args: None,
        };

        assert!(input.args_or_default().is_empty());
    }

    #[test]
    fn info_input_exposes_request_and_args() {
        let request = InfoPluginRequest::new(None);
        let input = InfoPluginInput {
            request: request.clone(),
            args: Some("value".to_string()),
        };

        assert_eq!(input.args_ref(), Some(&"value".to_string()));

        let (raw_request, args) = input.into_parts();
        assert_eq!(raw_request, request);
        assert_eq!(args.as_deref(), Some("value"));
    }
}
