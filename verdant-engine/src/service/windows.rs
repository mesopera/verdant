use std::ffi::OsString;
use std::time::Duration;
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher, Result as ServiceResult,
};
use tracing::{info, error};

const SERVICE_NAME: &str = "VerdantEngine";
const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

pub fn run_service() -> anyhow::Result<()> {
    // Register the service entry point
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)
        .map_err(|e| anyhow::anyhow!("Failed to start service dispatcher: {}", e))?;
    Ok(())
}

define_windows_service!(ffi_service_main, service_main);

fn service_main(arguments: Vec<OsString>) {
    if let Err(e) = run_service_impl(arguments) {
        error!("Service error: {}", e);
    }
}

fn run_service_impl(_arguments: Vec<OsString>) -> ServiceResult<()> {
    let (shutdown_tx, shutdown_rx) = std::sync::mpsc::channel();

    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop | ServiceControl::Interrogate => {
                shutdown_tx.send(()).ok();
                ServiceControlHandlerResult::NoError
            }
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    info!("Verdant Engine service started");

    // Run the main service loop in a separate thread
    let _service_thread = std::thread::spawn(|| {
        if let Err(e) = run_engine_loop() {
            error!("Engine loop failed: {}", e);
        }
    });

    // Wait for shutdown signal
    shutdown_rx.recv().ok();
    info!("Received shutdown signal");

    // Stop the service
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    Ok(())
}

fn run_engine_loop() -> anyhow::Result<()> {
    // This will be implemented in main.rs
    // For now, just a placeholder
    info!("Engine loop would run here");
    
    // Keep the service running
    loop {
        std::thread::sleep(Duration::from_secs(60));
    }
}

pub fn install_service() -> anyhow::Result<()> {
    use std::process::Command;
    
    let exe_path = std::env::current_exe()?;
    
    println!("Installing Verdant Engine as Windows service...");
    
    let output = Command::new("sc")
        .args(&[
            "create",
            SERVICE_NAME,
            &format!("binPath= \"{}\" service", exe_path.display()),
            "start= auto",
            "DisplayName= Verdant™ Contribution Optimization Engine",
        ])
        .output()?;
    
    if output.status.success() {
        println!("✓ Service installed successfully");
        println!("\nTo start the service, run:");
        println!("  verdant-engine.exe start");
        println!("\nOr use Windows Services Manager (services.msc)");
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to install service: {}", error)
    }
}

pub fn uninstall_service() -> anyhow::Result<()> {
    use std::process::Command;
    
    println!("Uninstalling Verdant Engine service...");
    
    // Stop the service first if it's running
    let _ = Command::new("sc")
        .args(&["stop", SERVICE_NAME])
        .output();
    
    // Wait a bit for the service to stop
    std::thread::sleep(Duration::from_secs(2));
    
    // Delete the service
    let output = Command::new("sc")
        .args(&["delete", SERVICE_NAME])
        .output()?;
    
    if output.status.success() {
        println!("✓ Service uninstalled successfully");
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to uninstall service: {}", error)
    }
}
