// AI Sysadmin Supreme - Main Entry Point
// Personalized for Lou's i9-13900HX Garuda Linux System

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error, Level};
use tracing_subscriber;

mod ai;
mod system;
mod hardware;
mod monitoring;
mod database;
mod commands;
mod learning;
mod security;

use ai::AIEngine;
use system::SystemController;
use hardware::HardwareManager;
use monitoring::SystemMonitor;
use database::Database;
use learning::LearningEngine;

// Global Application State
pub struct AppState {
    pub ai_engine: Arc<Mutex<AIEngine>>,
    pub system_controller: Arc<Mutex<SystemController>>,
    pub hardware_manager: Arc<Mutex<HardwareManager>>,
    pub system_monitor: Arc<Mutex<SystemMonitor>>,
    pub database: Arc<Mutex<Database>>,
    pub learning_engine: Arc<Mutex<LearningEngine>>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize database first
        let database = Arc::new(Mutex::new(Database::new().await?));
        
        // Initialize AI engine with hardware-specific optimizations
        let ai_engine = Arc::new(Mutex::new(AIEngine::new_for_i9_13900hx().await?));
        
        // Initialize system controller with Garuda Linux optimizations
        let system_controller = Arc::new(Mutex::new(SystemController::new_garuda().await?));
        
        // Initialize hardware manager for Lou's specific hardware
        let hardware_manager = Arc::new(Mutex::new(
            HardwareManager::new_for_gaming_laptop().await?
        ));
        
        // Initialize system monitor with real-time capabilities
        let system_monitor = Arc::new(Mutex::new(SystemMonitor::new().await?));
        
        // Initialize learning engine
        let learning_engine = Arc::new(Mutex::new(
            LearningEngine::new(database.clone()).await?
        ));
        
        Ok(Self {
            ai_engine,
            system_controller,
            hardware_manager,
            system_monitor,
            database,
            learning_engine,
        })
    }
    
    // Start all background services
    pub async fn start_services(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚀 Starting AI Sysadmin Supreme services...");
        
        // Start system monitoring
        {
            let mut monitor = self.system_monitor.lock().await;
            monitor.start_real_time_monitoring().await?;
        }
        
        // Start learning engine
        {
            let mut learning = self.learning_engine.lock().await;
            learning.start_pattern_learning().await?;
        }
        
        // Start hardware optimization
        {
            let mut hardware = self.hardware_manager.lock().await;
            hardware.start_optimization_loop().await?;
        }
        
        // Initialize AI with current system state
        {
            let mut ai = self.ai_engine.lock().await;
            ai.initialize_with_system_context().await?;
        }
        
        info!("✅ All services started successfully");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging with custom format for AI Sysadmin
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .init();

    info!("🤖 AI Sysadmin Supreme - Starting up...");
    info!("💻 Hardware: Intel i9-13900HX, 64GB RAM, RTX 4080 Mobile");
    info!("🐧 OS: Garuda Linux (Arch-based)");
    
    // Initialize application state
    let app_state = AppState::new().await
        .map_err(|e| {
            error!("Failed to initialize application state: {}", e);
            e
        })?;
    
    // Start all background services
    app_state.start_services().await
        .map_err(|e| {
            error!("Failed to start services: {}", e);
            e
        })?;
    
    info!("🎯 AI Sysadmin Supreme initialized successfully");
    
    // Start Tauri application with all commands
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // AI Commands
            commands::ai::chat_with_ai,
            commands::ai::get_ai_recommendations,
            commands::ai::train_on_user_action,
            
            // System Management Commands
            commands::system::get_system_info,
            commands::system::optimize_system,
            commands::system::clean_system,
            commands::system::update_system,
            commands::system::backup_system,
            commands::system::restore_system,
            
            // Hardware Control Commands
            commands::hardware::get_hardware_status,
            commands::hardware::set_cpu_governor,
            commands::hardware::control_fans,
            commands::hardware::get_temperatures,
            commands::hardware::optimize_for_workload,
            
            // Monitoring Commands
            commands::monitoring::get_real_time_stats,
            commands::monitoring::get_historical_data,
            commands::monitoring::set_alert_thresholds,
            
            // Package Management Commands
            commands::packages::list_installed_packages,
            commands::packages::update_packages,
            commands::packages::install_package,
            commands::packages::remove_package,
            commands::packages::search_aur,
            
            // File Management Commands
            commands::files::organize_files,
            commands::files::find_duplicates,
            commands::files::clean_temp_files,
            commands::files::analyze_disk_usage,
            
            // Learning Commands
            commands::learning::get_learned_patterns,
            commands::learning::get_usage_insights,
            commands::learning::predict_maintenance_needs,
        ])
        .system_tray(commands::tray::create_system_tray())
        .on_system_tray_event(commands::tray::handle_system_tray_event)
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");

    Ok(())
}
