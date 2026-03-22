// Trinity Protocol - Brain/Body Bridge
// Unified request/response types for all Bevy nodes

use crate::id_contract::IdContract;
use crate::stream::StreamEvent;
use crate::types::ModelInfo;
use serde::{Deserialize, Serialize};

#[cfg(feature = "bevy")]
use bevy::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrainRequest {
    Think {
        prompt: String,
        history: Vec<crate::types::ChatMessage>,
        context: Option<String>,
        temperature: Option<f32>,
        max_tokens: Option<usize>,
    },
    Ping,
    GetModelInfo,
    SubmitTask {
        task_type: String,
        payload: String,
    },
    GenerateIdContract {
        persona_id: String,
        goals: Vec<String>,
    },
    /// Request specialized analysis for ADDIE phase
    RequestAddieAnalysis {
        phase: String,
        context: Option<String>,
    },
    GetQueueStatus,
    ListPendingTasks,
    GetHardwareStats,
    PollEvents {
        since_id: u64,
    },
    PerformAddieAnalysis {
        project_data: String,
        phase: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrainResponse {
    Connected {
        model_info: Option<ModelInfo>,
    },
    Disconnected,
    ConnectionFailed {
        error: String,
    },
    ThinkResult {
        result: Result<String, String>,
    },
    /// Voice synthesis result with audio data
    VoiceResult {
        text: String,
        audio_data: Option<Vec<u8>>,
        sample_rate: u32,
    },
    PingResult {
        alive: bool,
    },
    ModelInfo {
        info: Option<ModelInfo>,
    },
    TaskSubmitted {
        task_id: uuid::Uuid,
    },
    QueueStatus {
        status: crate::task::QueueStatus,
    },
    PendingTasks {
        tasks: Vec<crate::task::TaskInfo>,
    },
    HardwareStats {
        stats: crate::types::HardwareStats,
    },
    StreamEvents {
        events: Vec<StreamEvent>,
    },
    /// Generated ID Contract response
    IdContractResult {
        contract: Result<IdContract, String>,
    },
}

#[cfg(feature = "bevy")]
#[derive(Resource)]
pub struct BrainConnection {
    pub connected: bool,
    pub brain_addr: String,
    pub model_info: Option<ModelInfo>,
    pub request_tx: crossbeam_channel::Sender<BrainRequest>,
    pub response_rx: crossbeam_channel::Receiver<BrainResponse>,
}

#[cfg(feature = "bevy")]
impl BrainConnection {
    /// Send a request to the brain
    pub fn send_request(&self, request: BrainRequest) -> Result<(), String> {
        self.request_tx
            .send(request)
            .map_err(|e| format!("Failed to send request: {}", e))
    }

    /// Try to receive a response from the brain
    pub fn try_recv_response(&self) -> Result<BrainResponse, crossbeam_channel::TryRecvError> {
        self.response_rx.try_recv()
    }
}
