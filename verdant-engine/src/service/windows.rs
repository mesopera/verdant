use std::error::Error;
use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;
use windows_service::{
    define_windows_service,
    service::{
        ServiceAccess, ServiceControl, ServiceControlAccept, ServiceErrorControl, ServiceExitCode,
        ServiceInfo, ServiceStartType, ServiceState, ServiceStatus, ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher,
    service_manager::{ServiceManager, ServiceManagerAccess},
    Result as ServiceResult,
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
    println!("Installing Verdant Engine as Windows service...");

    let service_manager = ServiceManager::local_computer(
        None::<&str>,
        ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE,
    )
    .map_err(|e| anyhow::anyhow!("Failed to open service manager: {}", e))?;

    let exe_path = std::env::current_exe()?;
    let binary_path = exe_path.display().to_string();

    let service_info = ServiceInfo {
        name: SERVICE_NAME.into(),
        display_name: "Verdant™ Contribution Optimization Engine".into(),
        service_type: SERVICE_TYPE,
        start_type: ServiceStartType::AutoStart,
        error_control: ServiceErrorControl::Normal,
        executable_path: PathBuf::from(binary_path),
        launch_arguments: vec!["service".into()],
        dependencies: vec![],
        account_name: None,
        account_password: None,
    };

    service_manager
        .create_service(&service_info, ServiceAccess::CHANGE_CONFIG)
        .map_err(|e| {
            let detail = e
                .source()
                .map(|s| format!(" (underlying: {})", s))
                .unwrap_or_default();
            anyhow::anyhow!("Failed to install service: {}{}", e, detail)
        })?;

    println!("✓ Service installed successfully");
    println!("\nTo start the service, run:");
    println!("  verdant-engine.exe start");
    println!("\nOr use Windows Services Manager (services.msc)");
    Ok(())
}

pub fn uninstall_service() -> anyhow::Result<()> {
    println!("Uninstalling Verdant Engine service...");

    let service_manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)
        .map_err(|e| anyhow::anyhow!("Failed to open service manager: {}", e))?;

    // Stop the service first if it's running
    if let Ok(service) = service_manager.open_service(SERVICE_NAME, ServiceAccess::QUERY_STATUS) {
        let _ = service.stop();
    }

    // Wait a bit for the service to stop
    std::thread::sleep(Duration::from_secs(2));

    let service = service_manager
        .open_service(SERVICE_NAME, ServiceAccess::DELETE)
        .map_err(|e| anyhow::anyhow!("Failed to find service '{}': {}", SERVICE_NAME, e))?;

    service
        .delete()
        .map_err(|e| anyhow::anyhow!("Failed to uninstall service: {}", e))?;

    println!("✓ Service uninstalled successfully");
    Ok(())
}
