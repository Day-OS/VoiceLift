use crate::PIPERTTS_MANAGER;
use busrt::rpc::{RpcEvent, RpcResult};
use events::client::{RequestStopTTS, ResponseStopTTS};
use vl_linux_backend::events;

fn _evt_stop_tts(event: RpcEvent) -> Result<(), String> {
    let manager = PIPERTTS_MANAGER
        .get()
        .ok_or("PIPERTTS_MANAGER not set")?
        .read()
        .map_err(|_| "Failed to lock PIPERTTS_MANAGER")?;

    // Verify if the event payload is of type RequestDevices
    let _: RequestStopTTS = rmp_serde::from_slice(event.payload())
        .map_err(|err| {
            format!("Failed to deserialize request: {err}")
        })?;

    manager.stop_speak().map_err(|e| format!("{e}"))?;
    drop(manager);

    Ok(())
}

pub fn evt_stop_tts(event: RpcEvent) -> RpcResult {
    let result = _evt_stop_tts(event);
    if let Err(e) = result.clone() {
        log::error!(
            "Failed to send Speak request to PipeWireTTS manager: {e}",
        );
    }
    let response = rmp_serde::to_vec(&ResponseStopTTS { result })?;

    Ok(Some(response))
}
