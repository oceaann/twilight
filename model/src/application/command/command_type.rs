use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum CommandType {
    /// Slash command.
    ///
    /// Text-based command that appears when a user types `/`.
    ChatInput,
    /// UI-based command.
    ///
    /// Appears when a user right clicks or taps om a user.
    User,
    /// UI-based command.
    ///
    /// Appears when a user right clicks or taps on a message.
    Message,

    /// Not yet supported
    Unknown(u8)
}

impl CommandType {
    pub const fn kind(self) -> &'static str {
        match self {
            Self::ChatInput => "ChatInput",
            Self::User => "User",
            Self::Message => "Message",
            CommandType::Unknown(_) => "Unknown interaction type"
        }
    }
}

impl From<u8> for CommandType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::ChatInput,
            2 => Self::User,
            3 => Self::Message,
            unknown => Self::Unknown(unknown)
        }
    }
}

impl From<CommandType> for u8 {
    fn from(value: CommandType) -> Self {
        match value {
            CommandType::ChatInput => 1,
            CommandType::User => 2,
            CommandType::Message => 3,
            CommandType::Unknown(unknown) => unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CommandType;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        CommandType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync
    );

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&CommandType::ChatInput, &[Token::U8(1)]);
        serde_test::assert_tokens(&CommandType::User, &[Token::U8(2)]);
        serde_test::assert_tokens(&CommandType::Message, &[Token::U8(3)]);
    }

    #[test]
    fn test_kinds() {
        assert_eq!("ChatInput", CommandType::ChatInput.kind());
        assert_eq!("User", CommandType::User.kind());
        assert_eq!("Message", CommandType::Message.kind());
        assert_eq!("Unknown interaction type", CommandType::Unknown(99).kind());
    }
}
