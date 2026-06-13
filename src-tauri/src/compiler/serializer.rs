use prost::Message;
use serde_json::Value;

pub fn compile_to_binary(json_input: &str, schema: &str) -> Result<Vec<u8>, String> {
    let parsed: Value = serde_json::from_str(json_input)
        .map_err(|e| format!("Strict JSON violation: {}", e))?;

    let mut buf = Vec::new();

    match schema {
        "GestureCommand" => {
            use crate::gen::gesture_command::{gesture_command, GestureCommand};

            let mut cmd = GestureCommand::default();

            cmd.action = match parsed["action"].as_str().unwrap_or("IDLE") {
                "JUMP"     => gesture_command::ActionType::Jump as i32,
                "WAVE"     => gesture_command::ActionType::Wave as i32,
                "RUN"      => gesture_command::ActionType::Run as i32,
                "ATTACK"   => gesture_command::ActionType::Attack as i32,
                "WALK"     => gesture_command::ActionType::Walk as i32,
                "CROUCH"   => gesture_command::ActionType::Crouch as i32,
                "ROLL"     => gesture_command::ActionType::Roll as i32,
                "EMOTE"    => gesture_command::ActionType::Emote as i32,
                "INTERACT" => gesture_command::ActionType::Interact as i32,
                _          => gesture_command::ActionType::Idle as i32,
            };
            cmd.intensity        = parsed["intensity"].as_f64().unwrap_or(0.5) as f32;
            cmd.duration_ms      = parsed["duration_ms"].as_i64().unwrap_or(0) as i32;
            cmd.compiler_verified = parsed["compiler_verified"].as_bool().unwrap_or(true);
            cmd.speed_multiplier = parsed["speed_multiplier"].as_f64().unwrap_or(1.0) as f32;
            cmd.loop_count       = parsed["loop_count"].as_i64().unwrap_or(1) as i32;
            cmd.priority         = parsed["priority"].as_i64().unwrap_or(0) as i32;
            cmd.target_bone      = parsed["target_bone"].as_str().unwrap_or("").to_string();

            cmd.blend_mode = match parsed["blend_mode"].as_str().unwrap_or("OVERRIDE") {
                "ADDITIVE" => gesture_command::BlendMode::Additive as i32,
                "LAYER"    => gesture_command::BlendMode::Layer as i32,
                _          => gesture_command::BlendMode::Override as i32,
            };
            cmd.easing = match parsed["easing"].as_str().unwrap_or("LINEAR") {
                "EASE_IN"     => gesture_command::Easing::EaseIn as i32,
                "EASE_OUT"    => gesture_command::Easing::EaseOut as i32,
                "EASE_IN_OUT" => gesture_command::Easing::EaseInOut as i32,
                "SPRING"      => gesture_command::Easing::Spring as i32,
                "BOUNCE"      => gesture_command::Easing::Bounce as i32,
                _             => gesture_command::Easing::Linear as i32,
            };
            cmd.emotion_tag = match parsed["emotion_tag"].as_str().unwrap_or("NEUTRAL") {
                "AGGRESSIVE" => gesture_command::EmotionTag::Aggressive as i32,
                "FEARFUL"    => gesture_command::EmotionTag::Fearful as i32,
                "JOYFUL"     => gesture_command::EmotionTag::Joyful as i32,
                "EXHAUSTED"  => gesture_command::EmotionTag::Exhausted as i32,
                "DETERMINED" => gesture_command::EmotionTag::Determined as i32,
                "SURPRISED"  => gesture_command::EmotionTag::Surprised as i32,
                _            => gesture_command::EmotionTag::Neutral as i32,
            };

            if let Some(dir) = parsed["direction"].as_object() {
                let mut v = gesture_command::Vector3::default();
                v.x = dir.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                v.y = dir.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                v.z = dir.get("z").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                cmd.direction = Some(v);
            }

            cmd.encode(&mut buf).map_err(|e| e.to_string())?;
        }

        "ExecutionPlan" => {
            use crate::gen::execution_plan::{execution_plan, ExecutionPlan};

            let mut plan = ExecutionPlan::default();
            plan.plan_id              = parsed["plan_id"].as_str().unwrap_or("").to_string();
            plan.objective            = parsed["objective"].as_str().unwrap_or("").to_string();
            plan.max_retries          = parsed["max_retries"].as_i64().unwrap_or(0) as i32;
            plan.timeout_ms           = parsed["timeout_ms"].as_i64().unwrap_or(0) as i32;
            plan.requires_confirmation = parsed["requires_confirmation"].as_bool().unwrap_or(false);
            plan.parallel_allowed     = parsed["parallel_allowed"].as_bool().unwrap_or(false);
            plan.estimated_cost_units = parsed["estimated_cost_units"].as_f64().unwrap_or(0.0) as f32;
            plan.compiler_verified    = true;

            plan.priority_level = match parsed["priority_level"].as_str().unwrap_or("NORMAL") {
                "CRITICAL"   => execution_plan::PriorityLevel::Critical as i32,
                "HIGH"       => execution_plan::PriorityLevel::High as i32,
                "LOW"        => execution_plan::PriorityLevel::Low as i32,
                "BACKGROUND" => execution_plan::PriorityLevel::Background as i32,
                _            => execution_plan::PriorityLevel::Normal as i32,
            };

            if let Some(nodes_arr) = parsed["nodes"].as_array() {
                let mut nodes = Vec::new();
                for node_val in nodes_arr {
                    let mut node = execution_plan::TaskNode::default();
                    node.task_id = node_val["task_id"].as_str().unwrap_or("").to_string();
                    node.tool_name = node_val["tool_name"].as_str().unwrap_or("").to_string();
                    node.parameters_json = node_val["parameters_json"].as_str().unwrap_or("").to_string();
                    if let Some(deps_arr) = node_val["dependencies"].as_array() {
                        node.dependencies = deps_arr.iter()
                            .filter_map(|d| d.as_str().map(|s| s.to_string()))
                            .collect();
                    }
                    nodes.push(node);
                }
                plan.nodes = nodes;
            }

            plan.encode(&mut buf).map_err(|e| e.to_string())?;
        }

        "InferencePacket" => {
            use crate::gen::inference_packet::{inference_packet, InferencePacket};

            let mut packet = InferencePacket::default();
            packet.session_id           = parsed["session_id"].as_str().unwrap_or("").to_string();
            packet.confidence_threshold = parsed["confidence_threshold"].as_f64().unwrap_or(0.7) as f32;
            packet.final_answer         = parsed["final_answer"].as_str().unwrap_or("").to_string();
            packet.overall_confidence   = parsed["overall_confidence"].as_f64().unwrap_or(0.0) as f32;
            packet.requires_clarification = parsed["requires_clarification"].as_bool().unwrap_or(false);
            packet.context_window_used  = parsed["context_window_used"].as_i64().unwrap_or(0) as i32;
            packet.compiler_verified    = true;

            packet.query_type = match parsed["query_type"].as_str().unwrap_or("CONVERSATIONAL") {
                "FACTUAL"       => inference_packet::QueryType::Factual as i32,
                "ANALYTICAL"    => inference_packet::QueryType::Analytical as i32,
                "CREATIVE"      => inference_packet::QueryType::Creative as i32,
                "INSTRUCTIONAL" => inference_packet::QueryType::Instructional as i32,
                _               => inference_packet::QueryType::Conversational as i32,
            };

            if let Some(cot_arr) = parsed["chain_of_thought"].as_array() {
                let mut cot = Vec::new();
                for node_val in cot_arr {
                    let mut node = inference_packet::ThoughtNode::default();
                    node.step_id = node_val["step_id"].as_str().unwrap_or("").to_string();
                    node.reasoning = node_val["reasoning"].as_str().unwrap_or("").to_string();
                    node.confidence = node_val["confidence"].as_f64().unwrap_or(0.0) as f32;
                    if let Some(ev_arr) = node_val["evidence_refs"].as_array() {
                        node.evidence_refs = ev_arr.iter()
                            .filter_map(|e| e.as_str().map(|s| s.to_string()))
                            .collect();
                    }
                    cot.push(node);
                }
                packet.chain_of_thought = cot;
            }

            packet.encode(&mut buf).map_err(|e| e.to_string())?;
        }

        _ => return Err(format!("Unknown schema target: {}", schema)),
    }

    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gesture_command_serialization() {
        let json = r#"{
            "action": "ATTACK",
            "intensity": 0.95,
            "duration_ms": 500,
            "direction": {
                "x": 0.0,
                "y": 0.0,
                "z": 1.0
            },
            "compiler_verified": true,
            "speed_multiplier": 1.0,
            "loop_count": 1,
            "blend_mode": "OVERRIDE",
            "priority": 8,
            "easing": "LINEAR",
            "target_bone": "",
            "emotion_tag": "AGGRESSIVE"
        }"#;

        let res = compile_to_binary(json, "GestureCommand");
        assert!(res.is_ok());
        let bytes = res.unwrap();
        use crate::gen::gesture_command::GestureCommand;
        use prost::Message;
        let decoded = GestureCommand::decode(&bytes[..]).unwrap();
        assert_eq!(decoded.intensity, 0.95);
        assert_eq!(decoded.duration_ms, 500);
        assert_eq!(decoded.priority, 8);
        assert!(decoded.direction.is_some());
        assert_eq!(decoded.direction.unwrap().z, 1.0);
    }

    #[test]
    fn test_execution_plan_serialization() {
        let json = r#"{
            "plan_id": "plan-123",
            "objective": "Execute a safe local workflow",
            "priority_level": "HIGH",
            "max_retries": 3,
            "timeout_ms": 10000,
            "requires_confirmation": false,
            "parallel_allowed": true,
            "nodes": [
                {
                    "task_id": "task_1",
                    "tool_name": "http_get",
                    "parameters_json": "{\"url\":\"https://example.com\"}",
                    "dependencies": []
                },
                {
                    "task_id": "task_2",
                    "tool_name": "fs_write",
                    "parameters_json": "{\"filename\":\"output.txt\",\"content\":\"hello\"}",
                    "dependencies": ["task_1"]
                }
            ],
            "estimated_cost_units": 1.5,
            "compiler_verified": true
        }"#;

        let res = compile_to_binary(json, "ExecutionPlan");
        assert!(res.is_ok());
        let bytes = res.unwrap();
        use crate::gen::execution_plan::ExecutionPlan;
        use prost::Message;
        let decoded = ExecutionPlan::decode(&bytes[..]).unwrap();
        assert_eq!(decoded.plan_id, "plan-123");
        assert_eq!(decoded.max_retries, 3);
        assert_eq!(decoded.nodes.len(), 2);
        assert_eq!(decoded.nodes[0].task_id, "task_1");
        assert_eq!(decoded.nodes[0].tool_name, "http_get");
        assert_eq!(decoded.nodes[1].dependencies.len(), 1);
        assert_eq!(decoded.nodes[1].dependencies[0], "task_1");
    }

    #[test]
    fn test_inference_packet_serialization() {
        let json = r#"{
            "session_id": "session-456",
            "query_type": "ANALYTICAL",
            "confidence_threshold": 0.85,
            "chain_of_thought": [
                {
                    "step_id": "step_1",
                    "reasoning": "Analyze the black hole entropy equation.",
                    "confidence": 0.95,
                    "evidence_refs": ["ref_bekenstein", "ref_hawking"]
                }
            ],
            "final_answer": "Entropy is proportional to the area of the event horizon.",
            "overall_confidence": 0.9,
            "requires_clarification": false,
            "context_window_used": 2048,
            "compiler_verified": true
        }"#;

        let res = compile_to_binary(json, "InferencePacket");
        assert!(res.is_ok());
        let bytes = res.unwrap();
        use crate::gen::inference_packet::InferencePacket;
        use prost::Message;
        let decoded = InferencePacket::decode(&bytes[..]).unwrap();
        assert_eq!(decoded.session_id, "session-456");
        assert_eq!(decoded.confidence_threshold, 0.85);
        assert_eq!(decoded.chain_of_thought.len(), 1);
        assert_eq!(decoded.chain_of_thought[0].step_id, "step_1");
        assert_eq!(decoded.chain_of_thought[0].evidence_refs.len(), 2);
        assert_eq!(decoded.chain_of_thought[0].evidence_refs[0], "ref_bekenstein");
    }
}

