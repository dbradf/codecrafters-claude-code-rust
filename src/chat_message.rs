use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum ChatMessage {
    System {
        content: SystemContent,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    User {
        content: UserContent,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    Developer {
        content: DeveloperContent,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    Assistant {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<AssistantContent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        refusal: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reasoning: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reasoning_details: Option<ReasoningDetails>,
        #[serde(skip_serializing_if = "Option::is_none")]
        images: Option<ImageUrl>,
    },
    Tool {
        content: String,
        tool_call_id: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolContent {
    String(String),
    ToolResponseMessageContent(ToolResponseMessageContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum ToolResponseMessageContent {
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
    #[serde(rename = "input_audio")]
    InputAudio { input_audio: InputAudio },
    #[serde(rename = "input_video")]
    InputVideo { video_url: VideoUrl },
    #[serde(rename = "video_url")]
    VideoUrl { video_url: VideoUrl },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    id: String,
    #[serde(rename = "type")]
    kind: String,
    function: ToolCallFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ToolCallFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ReasoningDetails {
    #[serde(rename = "reasoning.summary")]
    ReasoningSummary {
        summary: String,
        id: Option<String>,
        format: Option<ReasoningFormat>,
        index: Option<f64>,
    },
    #[serde(rename = "reasoning.encrypted")]
    ReasoningEncrypted {
        data: String,
        id: Option<String>,
        format: Option<ReasoningFormat>,
        index: Option<f64>,
    },
    #[serde(rename = "reasoning.text")]
    ReasoningText {
        text: Option<String>,
        signature: Option<String>,
        id: Option<String>,
        format: Option<ReasoningFormat>,
        index: Option<f64>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ReasoningFormat {
    Unknown,
    OpenAiResponsesV1,
    AzureOpenaiResonsesV1,
    XaiResponsesV1,
    AnthropicClaudeV1,
    GoogleGeminiV1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum AssistantContent {
    String(String),
    AssistantMessageContent(AssistantMessageContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum AssistantMessageContent {
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
    #[serde(rename = "input_audio")]
    InputAudio { input_audio: InputAudio },
    #[serde(rename = "input_video")]
    InputVideo { video_url: VideoUrl },
    #[serde(rename = "video_url")]
    VideoUrl { video_url: VideoUrl },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum DeveloperContent {
    String(String),
    DeveloperMessageContent {
        #[serde(rename = "type")]
        message_type: String, //always "text"
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserContent {
    String(String),
    UserMessageContent1(UserMessageContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum UserMessageContent {
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
    #[serde(rename = "input_audio")]
    InputAudio { input_audio: InputAudio },
    #[serde(rename = "input_video")]
    InputVideo { video_url: VideoUrl },
    #[serde(rename = "video_url")]
    VideoUrl { video_url: VideoUrl },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VideoUrl {
    url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InputAudio {
    data: String,
    format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ImageUrl {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<ImageUrlDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ImageUrlDetail {
    Auto,
    Low,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserContentText {
    #[serde(rename = "type")]
    message_type: String, //always "text"
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_control: Option<CacheControl>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SystemContent {
    String(String),
    SystemMessageContent1 {
        #[serde(rename = "type")]
        message_type: String, //always "text"
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheControl {
    #[serde(rename = "type")]
    cache_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<TtlOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum TtlOptions {
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "1h")]
    OneHour,
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(ChatMessage::System {
        content: SystemContent::String("hello world".to_string()), name: None
    }, "{\"role\":\"system\",\"content\":\"hello world\"}")]
    #[case(ChatMessage::System {
        content: SystemContent::SystemMessageContent1 {
            message_type: "text".to_string(),
            text: "hello world".to_string(),
            cache_control: Some(CacheControl {
                cache_type: "ephemeral".to_string(),
                ttl: Some(TtlOptions::FiveMinutes)
            })
        },
        name: Some("hello".to_string())
    }, "{\"role\":\"system\",\"content\":{\"type\":\"text\",\"text\":\"hello world\",\"cache_control\":{\"type\":\"ephemeral\",\"ttl\":\"5m\"}},\"name\":\"hello\"}")]
    fn test_system_messages(#[case] message: ChatMessage, #[case] expected: &str) {
        assert_eq!(expected, serde_json::to_string(&message).unwrap());
    }
}
