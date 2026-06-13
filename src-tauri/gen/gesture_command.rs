// Hand-written prost struct — GestureCommand (expanded)
// Covers 3D animation, physics, and expressive character control.
// All fields are deterministically enforced by gesture_command.gbnf

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GestureCommand {
    /// Core action type
    #[prost(enumeration = "gesture_command::ActionType", tag = "1")]
    pub action: i32,

    /// Normalised intensity of the action (0.0 = minimal, 1.0 = maximal)
    #[prost(float, tag = "2")]
    pub intensity: f32,

    /// Duration of the action in milliseconds
    #[prost(int32, tag = "3")]
    pub duration_ms: i32,

    /// Directional vector in 3D world space
    #[prost(message, optional, tag = "4")]
    pub direction: ::core::option::Option<gesture_command::Vector3>,

    /// Absolute guarantee this packet was machine-validated
    #[prost(bool, tag = "5")]
    pub compiler_verified: bool,

    /// Playback speed multiplier (0.1 = slow-mo, 1.0 = normal, 2.0 = fast)
    #[prost(float, tag = "6")]
    pub speed_multiplier: f32,

    /// Number of times to loop the action (0 = infinite, 1 = once)
    #[prost(int32, tag = "7")]
    pub loop_count: i32,

    /// Animation blending mode
    #[prost(enumeration = "gesture_command::BlendMode", tag = "8")]
    pub blend_mode: i32,

    /// Priority in the animation queue (higher = overrides lower)
    #[prost(int32, tag = "9")]
    pub priority: i32,

    /// Easing curve for the animation
    #[prost(enumeration = "gesture_command::Easing", tag = "10")]
    pub easing: i32,

    /// Optional: constrain action to a specific skeleton bone or bone group
    #[prost(string, tag = "11")]
    pub target_bone: ::prost::alloc::string::String,

    /// Emotional context for facial/body expression layering
    #[prost(enumeration = "gesture_command::EmotionTag", tag = "12")]
    pub emotion_tag: i32,
}

pub mod gesture_command {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Vector3 {
        #[prost(float, tag = "1")]
        pub x: f32,
        #[prost(float, tag = "2")]
        pub y: f32,
        #[prost(float, tag = "3")]
        pub z: f32,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ActionType {
        Idle = 0,
        Jump = 1,
        Wave = 2,
        Run = 3,
        Attack = 4,
        Walk = 5,
        Crouch = 6,
        Roll = 7,
        Emote = 8,
        Interact = 9,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BlendMode {
        Override = 0,
        Additive = 1,
        Layer = 2,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Easing {
        Linear = 0,
        EaseIn = 1,
        EaseOut = 2,
        EaseInOut = 3,
        Spring = 4,
        Bounce = 5,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EmotionTag {
        Neutral = 0,
        Aggressive = 1,
        Fearful = 2,
        Joyful = 3,
        Exhausted = 4,
        Determined = 5,
        Surprised = 6,
    }
}
