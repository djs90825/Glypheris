// Hand-written prost struct — ExecutionPlan (full build-out)
// Covers autonomous agent DAG task orchestration.
// All fields deterministically enforced by execution_plan.gbnf

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionPlan {
    /// Unique plan identifier (UUID-style string)
    #[prost(string, tag = "1")]
    pub plan_id: ::prost::alloc::string::String,

    /// Natural language summary of the goal
    #[prost(string, tag = "2")]
    pub objective: ::prost::alloc::string::String,

    /// Execution priority level
    #[prost(enumeration = "execution_plan::PriorityLevel", tag = "3")]
    pub priority_level: i32,

    /// Max retry attempts on failure (0 = no retry)
    #[prost(int32, tag = "4")]
    pub max_retries: i32,

    /// Hard execution deadline in milliseconds (0 = no limit)
    #[prost(int32, tag = "5")]
    pub timeout_ms: i32,

    /// If true, a human must approve before execution begins
    #[prost(bool, tag = "6")]
    pub requires_confirmation: bool,

    /// If true, independent nodes may execute concurrently
    #[prost(bool, tag = "7")]
    pub parallel_allowed: bool,

    /// The ordered or parallel DAG of tasks
    #[prost(message, repeated, tag = "8")]
    pub nodes: ::prost::alloc::vec::Vec<execution_plan::TaskNode>,

    /// Estimated compute cost in abstract units
    #[prost(float, tag = "9")]
    pub estimated_cost_units: f32,

    /// Absolute guarantee this packet was machine-validated
    #[prost(bool, tag = "10")]
    pub compiler_verified: bool,
}

pub mod execution_plan {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TaskNode {
        #[prost(string, tag = "1")]
        pub task_id: ::prost::alloc::string::String,

        #[prost(string, tag = "2")]
        pub tool_name: ::prost::alloc::string::String,

        /// JSON-encoded parameter map for the tool
        #[prost(string, tag = "3")]
        pub parameters_json: ::prost::alloc::string::String,

        /// task_ids this node depends on completing first
        #[prost(string, repeated, tag = "4")]
        pub dependencies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PriorityLevel {
        Background = 0,
        Low = 1,
        Normal = 2,
        High = 3,
        Critical = 4,
    }
}
