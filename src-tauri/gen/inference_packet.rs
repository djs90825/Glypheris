// Hand-written prost struct — InferencePacket (full build-out)
// Covers general AI chat, analytical, and factual inference pipelines.
// All fields deterministically enforced by inference_packet.gbnf

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferencePacket {
    /// Conversation or session tracking identifier
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,

    /// Classification of the query type
    #[prost(enumeration = "inference_packet::QueryType", tag = "2")]
    pub query_type: i32,

    /// Minimum confidence threshold to accept the answer (0.0–1.0)
    #[prost(float, tag = "3")]
    pub confidence_threshold: f32,

    /// Step-by-step reasoning chain
    #[prost(message, repeated, tag = "4")]
    pub chain_of_thought: ::prost::alloc::vec::Vec<inference_packet::ThoughtNode>,

    /// The final concluded answer or response
    #[prost(string, tag = "5")]
    pub final_answer: ::prost::alloc::string::String,

    /// Overall confidence score of the final answer (0.0–1.0)
    #[prost(float, tag = "6")]
    pub overall_confidence: f32,

    /// True if the original query was ambiguous and needs clarification
    #[prost(bool, tag = "7")]
    pub requires_clarification: bool,

    /// Number of context window tokens consumed during inference
    #[prost(int32, tag = "8")]
    pub context_window_used: i32,

    /// Absolute guarantee this packet was machine-validated
    #[prost(bool, tag = "9")]
    pub compiler_verified: bool,
}

pub mod inference_packet {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ThoughtNode {
        #[prost(string, tag = "1")]
        pub step_id: ::prost::alloc::string::String,

        #[prost(string, tag = "2")]
        pub reasoning: ::prost::alloc::string::String,

        #[prost(float, tag = "3")]
        pub confidence: f32,

        /// References to supporting evidence or context
        #[prost(string, repeated, tag = "4")]
        pub evidence_refs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum QueryType {
        Conversational = 0,
        Factual = 1,
        Analytical = 2,
        Creative = 3,
        Instructional = 4,
    }
}
