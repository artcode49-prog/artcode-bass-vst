use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, EguiState};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

// ========== PRESET SYSTEM ==========
#[derive(Clone, Copy, PartialEq)]
enum PresetCategory { Init, Sub, Fat, Acid, Wobble, Growl, Clean, User }

impl PresetCategory {
    fn name(&self) -> &'static str {
        match self { Self::Init => "Init", Self::Sub => "Sub", Self::Fat => "Fat", Self::Acid => "Acid", Self::Wobble => "Wobble", Self::Growl => "Growl", Self::Clean => "Clean", Self::User => "User" }
    }
    fn all() -> &'static [PresetCategory] { &[Self::Init, Self::Sub, Self::Fat, Self::Acid, Self::Wobble, Self::Growl, Self::Clean, Self::User] }
}

#[derive(Clone)]
struct PresetData {
    name: String, category: PresetCategory,
    osc1_wave: i32, osc1_det: f32, osc2_wave: i32, osc2_det: f32, osc2_mix: f32, sub_vol: f32,
    unison: i32, spread: f32,
    filter_cut: f32, filter_res: f32, filter_env: f32, filter_type: i32, filter_slope: i32,
    drive: f32, drive_type: i32, low_boost: f32,
    attack: f32, decay: f32, sustain: f32, release: f32,
    lfo_rate: f32, lfo_depth: f32, lfo_wave: i32, lfo_target: i32, porta: f32,
    delay_mix: f32, delay_time: f32, delay_fb: f32, reverb_mix: f32, reverb_size: f32,
}

impl Default for PresetData {
    fn default() -> Self {
        Self {
            name: "Init".into(), category: PresetCategory::Init,
            osc1_wave: 1, osc1_det: 0.0, osc2_wave: 1, osc2_det: 7.0, osc2_mix: 0.5, sub_vol: 0.5,
            unison: 4, spread: 0.25,
            filter_cut: 600.0, filter_res: 0.4, filter_env: 0.07, filter_type: 0, filter_slope: 1,
            drive: 0.1, drive_type: 2, low_boost: 0.5,
            attack: 0.005, decay: 0.15, sustain: 0.7, release: 0.15,
            lfo_rate: 2.0, lfo_depth: 0.0, lfo_wave: 0, lfo_target: 1, porta: 0.007,
            delay_mix: 0.0, delay_time: 0.3, delay_fb: 0.4, reverb_mix: 0.0, reverb_size: 0.5,
        }
    }
}

fn create_factory_presets() -> Vec<PresetData> {
    vec![
        PresetData::default(),
        // Sub (5)
        PresetData { name: "Deep Sub".into(), category: PresetCategory::Sub, osc1_wave: 0, osc1_det: 0.0, osc2_wave: 0, osc2_det: 0.0, osc2_mix: 0.0, sub_vol: 1.0, unison: 1, spread: 0.0, filter_cut: 150.0, filter_res: 0.2, filter_env: 0.025, filter_type: 0, filter_slope: 1, drive: 0.1, drive_type: 2, low_boost: 0.8, attack: 0.005, decay: 0.1, sustain: 0.9, release: 0.2, lfo_rate: 0.0, lfo_depth: 0.0, lfo_wave: 0, lfo_target: 1, porta: 0.005, delay_mix: 0.0, delay_time: 0.3, delay_fb: 0.3, reverb_mix: 0.0, reverb_size: 0.3 },
        PresetData { name: "808 Sub".into(), category: PresetCategory::Sub, osc1_wave: 0, osc1_det: 0.0, osc2_wave: 0, osc2_det: 0.0, osc2_mix: 0.0, sub_vol: 0.8, unison: 1, spread: 0.0, filter_cut: 200.0, filter_res: 0.3, filter_env: 0.025, filter_type: 0, filter_slope: 1, drive: 0.05, drive_type: 2, low_boost: 0.9, attack: 0.001, decay: 0.8, sustain: 0.0, release: 0.5, lfo_rate: 0.0, lfo_depth: 0.0, lfo_wave: 0, lfo_target: 1, porta: 0.0052, delay_mix: 0.0, delay_time: 0.3, delay_fb: 0.3, reverb_mix: 0.0, reverb_size: 0.3 },
        PresetData { name: "Sine Sub".into(), category: PresetCategory::Sub, osc1_wave: 0, osc1_det: 0.0, osc2_wave: 0, osc2_det: 0.0, osc2_mix: 0.0, sub_vol: 0.9, unison: 1, spread: 0.0, filter_cut: 120.0, filter_res: 0.1, filter_env: 0.0, filter_type: 0, filter_slope: 1, drive: 0.1, drive_type: 0, low_boost: 1.0, attack: 0.01, decay: 0.1, sustain: 1.0, release: 0.2, lfo_rate: 0.0, lfo_depth: 0.0, lfo_wave: 0, lfo_target: 1, porta: 0.005, delay_mix: 0.0, delay_time: 0.3, delay_fb: 0.3, reverb_mix: 0.0, reverb_size: 0.3 },
        PresetData { name: "Dark Sub".into(), category: PresetCategory::Sub, osc1_wave: 1, osc1_det: 0.0, osc2_wave: 0, osc2_det: 0.0, osc2_mix: 0.2, sub_vol: 0.85, unison: 1, spread: 0.0, filter_cut: 180.0, filter_res: 0.4, filter_env: 0.025, filter_type: 0, filter_slope: 1, drive: 0.15, drive_type: 2, low_boost: 0.7, attack: 0.005, decay: 0.2, sustain: 0.8, release: 0.25, lfo_rate: 0.0, lfo_depth: 0.0, lfo_wave: 0, lfo_target: 1, porta: 0.007, delay_mix: 0.0, delay_time: 0.3, delay_fb: 0.3, reverb_mix: 0.0, reverb_size: 0.3 },
        PresetData { name: "Rumble Sub".into(), category: PresetCategory::Sub, osc1_wave: 0, osc1_det: 0.0, osc2_wave: 1, osc2_det: -5.0, osc2_mix: 0.15, sub_vol: 0.9, unison: 2, spread: 0.1, filter_cut: 160.0, filter_res: 0.35, filter_env: 0.03, filter_type: 0, filter_slope: 1, drive: 0.1, drive_type: 2, low_boost: 0.85, attack: 0.01, decay: 0.15, sustain: 0.85, release: 0.3, lfo_rate: 0.3, lfo_depth: 0.1, lfo_wave: 0, lfo_target: 1, porta: 0.005, delay_mix: 0.0, delay_time: 0.3, delay_fb: 0.3, reverb_mix: 0.0, reverb_size: 0.3 },
        // Fat (6)
        PresetData { name: "Fat Saw".into(), category: PresetCategory::Fat, osc1_wave: 1, osc1_det: -5.0, osc2_wave: 1, osc2_det: 5.0, osc2_mix: 0.5, sub_vol: 0.6, unison: 6, spread: 0.3, filter_cut: 500.0, filter_res: 0.5, filter_env: 0.025, filter_type: 0, filter_slope: 1, drive: 0.15, drive_type: 2, low_boost: 0.6, attack: 0.005, decay: 0.2, sustain: 0.7, release: 0.15, lfo_rate: 0.5, lfo_depth: 0.05, lfo_wave: 0, lfo_target: 1, porta: 0.007, delay_mix: 0.0, delay_time: 0.3, delay_fb: 0.3, reverb_mix: 0.0, reverb_size: 0.3 },
        PresetData { name: "Massive".into(), category: PresetCategory::Fat, osc1_wave: 1, osc1_det: -10.0, osc2_wave: 2, osc2_det: 10.0, osc2_mix: 0.6, sub_vol: 0.7, unison: 8, spread: 0.4, filter_cut: 600.0, filter_res: 0.45, filter_env: 0.08, filter_type: 0, filter_slope: 1, drive: 0.05, drive_type: 1, low_boost: 0.7, attack: 0.01, decay: 0.15, sustain: 0.75, release: 0.2, lfo_rate: 0.3, lfo_depth: 0.08, lfo_wave: 0, lfo_target: 1, porta: 0.005, delay_mix: 0.05, delay_time: 0.25, delay_fb: 0.3, reverb_mix: 0.05, reverb_size: 0.4 },
        PresetData { name: "Wall of Bass".into(), category: PresetCategory::Fat, osc1_wave: 1, osc1_det: -15.0, osc2_wave: 1, osc2_det: 15.0, osc2_mix: 0.5, sub_vol: 0.5, unison: 8, spread: 0.5, filter_cut: 800.0, filter_res: 0.4, filter_env: 0.07, filter_type: 0, filter_slope: 1, drive: 0.18, drive_type: 2, low_boost: 0.5, attack: 0.02, decay: 0.2, sustain: 0.8, release: 0.25, lfo_rate: 0.2, lfo_depth: 0.1, lfo_wave: 0, lfo_target: 1, porta: 0.0052, delay_mix: 0.1, delay_time: 0.3, delay_fb: 0.35, reverb_mix: 0.1, reverb_size: 0.5 },
        PresetData { name: "Thick Square".into(), category: PresetCategory::Fat, osc1_wave: 2, osc1_det: -7.0, osc2_wave: 2, osc2_det: 7.0, osc2_mix: 0.5, sub_vol: 0.55, unison: 5, spread: 0.25, filter_cut: 450.0, filter_res: 0.55, filter_env: 0.0252, filter_type: 0, filter_slope: 1, drive: 0.12, drive_type: 2, low_boost: 0.55, attack: 0.005, decay: 0.18, sustain: 0.65, release: 0.15, lfo_rate: 0.0, lfo_depth: 0.0, lfo_wave: 0, lfo_target: 1, porta: 0.007, delay_mix: 0.0, delay_time: 0.3, delay_fb: 0.3, reverb_mix: 0.0, reverb_size: 0.3 },
        PresetData { name: "Reese Monster".into(), category: PresetCategory::Fat, osc1_wave: 1, osc1_det: -20.0, osc2_wave: 1, osc2_det: 20.0, osc2_mix: 0.5, sub_vol: 0.4, unison: 4, spread: 0.35, filter_cut: 700.0, filter_res: 0.35, filter_env: 0.07, filter_type: 0, filter_slope: 1, drive: 0.1, drive_type: 2, low_boost: 0.45, attack: 0.01, decay: 0.2, sustain: 0.75, release: 0.2, lfo_rate: 0.15, lfo_depth: 0.15, lfo_wave: 0, lfo_target: 1, porta: 0.0, delay_mix: 0.0, delay_time: 0.3, delay_fb: 0.3, reverb_mix: 0.05, reverb_size: 0.4 },
        PresetData { name: "Phat Mono".into(), category: PresetCategory::Fat, osc1_wave: 1, osc1_det: 0.0, osc2_wave: 2, osc2_det: 0.0, osc2_mix: 0.4, sub_vol: 0.65, unison: 4, spread: 0.2, filter_cut: 550.0, filter_res: 0.5, filter_env: 0.0252, filter_type: 0, filter_slope: 1, drive: 0.15, drive_type: 2, low_boost: 0.6, attack: 0.005, decay: 0.15, sustain: 0.6, release: 0.12, lfo_rate: 0.0, lfo_depth: 0.0, lfo_wave: 0, lfo_target: 1, porta: 0.005, delay_mix: 0.0, delay_time: 0.3, delay_fb: 0.3, reverb_mix: 0.0, reverb_size: 0.3 },
        // Acid (5)
        PresetData { name: "303 Acid".into(), category: PresetCategory::Acid, osc1_wave: 1, osc1_det: 0.0, osc2_wave: 2, osc2_det: 0.0, osc2_mix: 0.3, sub_vol: 0.3, unison: 1, spread: 0.0, filter_cut: 400.0, filter_res: 0.85, filter_env: 0.025, filter_type: 0, filter_slope: 1, drive: 0.05, drive_type: 3, low_boost: 0.4, attack: 0.001, decay: 0.15, sustain: 0.0, release: 0.1, lfo_rate: 0.0, lfo_depth: 0.0, lfo_wave: 0, lfo_target: 1, porta: 0.005, delay_mix: 0.15, delay_time: 0.2, delay_fb: 0.4, reverb_mix: 0.05, reverb_size: 0.3 },
        PresetData { name: "Squelch".into(), category: PresetCategory::Acid, osc1_wave: 2, osc1_det: 0.0, osc2_wave: 1, osc2_det: 0.0, osc2_mix: 0.2, sub_vol: 0.35, unison: 1, spread: 0.0, filter_cut: 350.0, filter_res: 0.9, filter_env: 0.0252, filter_type: 0, filter_slope: 1, drive: 0.05, drive_type: 3, low_boost: 0.35, attack: 0.001, decay: 0.12, sustain: 0.0, release: 0.08, lfo_rate: 0.0, lfo_depth: 0.0, lfo_wave: 0, lfo_target: 1, porta: 0.007, delay_mix: 0.1, delay_time: 0.18, delay_fb: 0.35, reverb_mix: 0.0, reverb_size: 0.3 },
        PresetData { name: "Resonant Acid".into(), category: PresetCategory::Acid, osc1_wave: 1, osc1_det: 0.0, osc2_wave: 1, osc2_det: 5.0, osc2_mix: 0.25, sub_vol: 0.4, unison: 2, spread: 0.1, filter_cut: 450.0, filter_res: 0.92, filter_env: 0.0258, filter_type: 0, filter_slope: 1, drive: 0.05, drive_type: 3, low_boost: 0.45, attack: 0.001, decay: 0.18, sustain: 0.1, release: 0.12, lfo_rate: 0.0, lfo_depth: 0.0, lfo_wave: 0, lfo_target: 1, porta: 0.0052, delay_mix: 0.2, delay_time: 0.22, delay_fb: 0.45, reverb_mix: 0.08, reverb_size: 0.35 },
        PresetData { name: "Dirty Acid".into(), category: PresetCategory::Acid, osc1_wave: 1, osc1_det: 0.0, osc2_wave: 2, osc2_det: 0.0, osc2_mix: 0.4, sub_vol: 0.3, unison: 1, spread: 0.0, filter_cut: 380.0, filter_res: 0.88, filter_env: 0.0252, filter_type: 0, filter_slope: 1, drive: 0.05, drive_type: 1, low_boost: 0.4, attack: 0.001, decay: 0.14, sustain: 0.0, release: 0.1, lfo_rate: 0.0, lfo_depth: 0.0, lfo_wave: 0, lfo_target: 1, porta: 0.008, delay_mix: 0.12, delay_time: 0.2, delay_fb: 0.38, reverb_mix: 0.03, reverb_size: 0.3 },
        PresetData { name: "Acid Stab".into(), category: PresetCategory::Acid, osc1_wave: 2, osc1_det: 0.0, osc2_wave: 2, osc2_det: 7.0, osc2_mix: 0.35, sub_vol: 0.25, unison: 2, spread: 0.15, filter_cut: 500.0, filter_res: 0.8, filter_env: 0.0257, filter_type: 0, filter_slope: 1, drive: 0.05, drive_type: 3, low_boost: 0.35, attack: 0.001, decay: 0.1, sustain: 0.0, release: 0.08, lfo_rate: 0.0, lfo_depth: 0.0, lfo_wave: 0, lfo_target: 1, porta: 0.0, delay_mix: 0.18, delay_time: 0.15, delay_fb: 0.5, reverb_mix: 0.1, reverb_size: 0.4 },
        // Wobble (5)
        PresetData { name: "Dubstep Wobble".into(), category: PresetCategory::Wobble, osc1_wave: 1, osc1_det: 0.0, osc2_wave: 1, osc2_det: -7.0, osc2_mix: 0.5, sub_vol: 0.5, unison: 4, spread: 0.2, filter_cut: 800.0, filter_res: 0.7, filter_env: 0.0, filter_type: 0, filter_slope: 1, drive: 0.15, drive_type: 2, low_boost: 0.5, attack: 0.01, decay: 0.1, sustain: 0.8, release: 0.15, lfo_rate: 4.0, lfo_depth: 0.8, lfo_wave: 0, lfo_target: 1, porta: 0.0, delay_mix: 0.0, delay_time: 0.3, delay_fb: 0.3, reverb_mix: 0.1, reverb_size: 0.4 },
        PresetData { name: "Slow Wobble".into(), category: PresetCategory::Wobble, osc1_wave: 1, osc1_det: -5.0, osc2_wave: 2, osc2_det: 5.0, osc2_mix: 0.45, sub_vol: 0.55, unison: 3, spread: 0.25, filter_cut: 700.0, filter_res: 0.65, filter_env: 0.025, filter_type: 0, filter_slope: 1, drive: 0.12, drive_type: 2, low_boost: 0.55, attack: 0.02, decay: 0.15, sustain: 0.75, release: 0.2, lfo_rate: 1.5, lfo_depth: 0.75, lfo_wave: 0, lfo_target: 1, porta: 0.0, delay_mix: 0.05, delay_time: 0.35, delay_fb: 0.35, reverb_mix: 0.15, reverb_size: 0.5 },
        PresetData { name: "Fast Wobble".into(), category: PresetCategory::Wobble, osc1_wave: 1, osc1_det: 0.0, osc2_wave: 1, osc2_det: 0.0, osc2_mix: 0.4, sub_vol: 0.45, unison: 4, spread: 0.15, filter_cut: 900.0, filter_res: 0.75, filter_env: 0.0, filter_type: 0, filter_slope: 1, drive: 0.18, drive_type: 2, low_boost: 0.45, attack: 0.005, decay: 0.1, sustain: 0.85, release: 0.12, lfo_rate: 12.0, lfo_depth: 0.7, lfo_wave: 0, lfo_target: 1, porta: 0.0, delay_mix: 0.0, delay_time: 0.3, delay_fb: 0.3, reverb_mix: 0.08, reverb_size: 0.35 },
        PresetData { name: "Square Wobble".into(), category: PresetCategory::Wobble, osc1_wave: 2, osc1_det: 0.0, osc2_wave: 1, osc2_det: 5.0, osc2_mix: 0.5, sub_vol: 0.5, unison: 3, spread: 0.2, filter_cut: 750.0, filter_res: 0.68, filter_env: 0.025, filter_type: 0, filter_slope: 1, drive: 0.15, drive_type: 2, low_boost: 0.5, attack: 0.01, decay: 0.12, sustain: 0.8, release: 0.15, lfo_rate: 6.0, lfo_depth: 0.85, lfo_wave: 2, lfo_target: 1, porta: 0.0, delay_mix: 0.05, delay_time: 0.28, delay_fb: 0.32, reverb_mix: 0.1, reverb_size: 0.4 },
        PresetData { name: "Morphing Wobble".into(), category: PresetCategory::Wobble, osc1_wave: 1, osc1_det: -10.0, osc2_wave: 2, osc2_det: 10.0, osc2_mix: 0.5, sub_vol: 0.4, unison: 5, spread: 0.3, filter_cut: 850.0, filter_res: 0.72, filter_env: 0.03, filter_type: 0, filter_slope: 1, drive: 0.16, drive_type: 2, low_boost: 0.48, attack: 0.015, decay: 0.12, sustain: 0.78, release: 0.18, lfo_rate: 3.0, lfo_depth: 0.82, lfo_wave: 3, lfo_target: 1, porta: 0.0, delay_mix: 0.08, delay_time: 0.32, delay_fb: 0.38, reverb_mix: 0.12, reverb_size: 0.45 },
        // Growl (5)
        PresetData { name: "Growl".into(), category: PresetCategory::Growl, osc1_wave: 1, osc1_det: -15.0, osc2_wave: 2, osc2_det: 15.0, osc2_mix: 0.6, sub_vol: 0.35, unison: 6, spread: 0.4, filter_cut: 700.0, filter_res: 0.75, filter_env: 0.0, filter_type: 0, filter_slope: 1, drive: 0.155, drive_type: 1, low_boost: 0.4, attack: 0.01, decay: 0.1, sustain: 0.8, release: 0.15, lfo_rate: 8.0, lfo_depth: 0.6, lfo_wave: 2, lfo_target: 1, porta: 0.0, delay_mix: 0.0, delay_time: 0.3, delay_fb: 0.3, reverb_mix: 0.05, reverb_size: 0.35 },
        PresetData { name: "Aggressive".into(), category: PresetCategory::Growl, osc1_wave: 1, osc1_det: -20.0, osc2_wave: 1, osc2_det: 20.0, osc2_mix: 0.55, sub_vol: 0.3, unison: 7, spread: 0.45, filter_cut: 800.0, filter_res: 0.8, filter_env: 0.025, filter_type: 0, filter_slope: 1, drive: 0.1, drive_type: 1, low_boost: 0.35, attack: 0.005, decay: 0.08, sustain: 0.85, release: 0.12, lfo_rate: 10.0, lfo_depth: 0.65, lfo_wave: 2, lfo_target: 1, porta: 0.0, delay_mix: 0.0, delay_time: 0.3, delay_fb: 0.3, reverb_mix: 0.03, reverb_size: 0.3 },
        PresetData { name: "Screamer".into(), category: PresetCategory::Growl, osc1_wave: 1, osc1_det: -25.0, osc2_wave: 2, osc2_det: 25.0, osc2_mix: 0.65, sub_vol: 0.25, unison: 8, spread: 0.5, filter_cut: 1000.0, filter_res: 0.85, filter_env: 0.025, filter_type: 0, filter_slope: 1, drive: 0.12, drive_type: 3, low_boost: 0.3, attack: 0.003, decay: 0.1, sustain: 0.9, release: 0.1, lfo_rate: 12.0, lfo_depth: 0.7, lfo_wave: 1, lfo_target: 1, porta: 0.0, delay_mix: 0.05, delay_time: 0.2, delay_fb: 0.35, reverb_mix: 0.05, reverb_size: 0.35 },
        PresetData { name: "Metallic".into(), category: PresetCategory::Growl, osc1_wave: 2, osc1_det: -30.0, osc2_wave: 2, osc2_det: 30.0, osc2_mix: 0.5, sub_vol: 0.2, unison: 6, spread: 0.35, filter_cut: 1200.0, filter_res: 0.7, filter_env: 0.03, filter_type: 0, filter_slope: 1, drive: 0.158, drive_type: 1, low_boost: 0.25, attack: 0.001, decay: 0.15, sustain: 0.7, release: 0.15, lfo_rate: 15.0, lfo_depth: 0.5, lfo_wave: 2, lfo_target: 1, porta: 0.0, delay_mix: 0.1, delay_time: 0.15, delay_fb: 0.4, reverb_mix: 0.08, reverb_size: 0.4 },
        PresetData { name: "Chaos".into(), category: PresetCategory::Growl, osc1_wave: 1, osc1_det: -35.0, osc2_wave: 2, osc2_det: 35.0, osc2_mix: 0.6, sub_vol: 0.2, unison: 8, spread: 0.5, filter_cut: 900.0, filter_res: 0.82, filter_env: 0.07, filter_type: 0, filter_slope: 1, drive: 0.12, drive_type: 3, low_boost: 0.25, attack: 0.002, decay: 0.12, sustain: 0.88, release: 0.1, lfo_rate: 8.0, lfo_depth: 0.9, lfo_wave: 1, lfo_target: 1, porta: 0.0, delay_mix: 0.08, delay_time: 0.18, delay_fb: 0.45, reverb_mix: 0.1, reverb_size: 0.45 },
        // Clean (4)
        PresetData { name: "Clean Finger".into(), category: PresetCategory::Clean, osc1_wave: 0, osc1_det: 0.0, osc2_wave: 3, osc2_det: 0.0, osc2_mix: 0.3, sub_vol: 0.4, unison: 1, spread: 0.0, filter_cut: 1500.0, filter_res: 0.2, filter_env: 0.025, filter_type: 0, filter_slope: 0, drive: 0.0, drive_type: 0, low_boost: 0.3, attack: 0.005, decay: 0.3, sustain: 0.5, release: 0.3, lfo_rate: 0.0, lfo_depth: 0.0, lfo_wave: 0, lfo_target: 1, porta: 0.0, delay_mix: 0.1, delay_time: 0.35, delay_fb: 0.3, reverb_mix: 0.15, reverb_size: 0.5 },
        PresetData { name: "Soft Synth".into(), category: PresetCategory::Clean, osc1_wave: 0, osc1_det: 0.0, osc2_wave: 0, osc2_det: 5.0, osc2_mix: 0.4, sub_vol: 0.5, unison: 2, spread: 0.1, filter_cut: 1200.0, filter_res: 0.25, filter_env: 0.03, filter_type: 0, filter_slope: 0, drive: 0.05, drive_type: 0, low_boost: 0.4, attack: 0.02, decay: 0.2, sustain: 0.6, release: 0.35, lfo_rate: 0.0, lfo_depth: 0.0, lfo_wave: 0, lfo_target: 1, porta: 0.005, delay_mix: 0.12, delay_time: 0.4, delay_fb: 0.35, reverb_mix: 0.2, reverb_size: 0.55 },
        PresetData { name: "Mellow".into(), category: PresetCategory::Clean, osc1_wave: 3, osc1_det: 0.0, osc2_wave: 0, osc2_det: 0.0, osc2_mix: 0.35, sub_vol: 0.45, unison: 1, spread: 0.0, filter_cut: 800.0, filter_res: 0.15, filter_env: 0.025, filter_type: 0, filter_slope: 0, drive: 0.0, drive_type: 0, low_boost: 0.35, attack: 0.03, decay: 0.25, sustain: 0.55, release: 0.4, lfo_rate: 0.0, lfo_depth: 0.0, lfo_wave: 0, lfo_target: 1, porta: 0.007, delay_mix: 0.08, delay_time: 0.38, delay_fb: 0.28, reverb_mix: 0.25, reverb_size: 0.6 },
        PresetData { name: "Warm DI".into(), category: PresetCategory::Clean, osc1_wave: 1, osc1_det: 0.0, osc2_wave: 0, osc2_det: 0.0, osc2_mix: 0.2, sub_vol: 0.55, unison: 1, spread: 0.0, filter_cut: 1000.0, filter_res: 0.2, filter_env: 0.07, filter_type: 0, filter_slope: 0, drive: 0.1, drive_type: 2, low_boost: 0.5, attack: 0.01, decay: 0.2, sustain: 0.65, release: 0.25, lfo_rate: 0.0, lfo_depth: 0.0, lfo_wave: 0, lfo_target: 1, porta: 0.0, delay_mix: 0.05, delay_time: 0.3, delay_fb: 0.25, reverb_mix: 0.1, reverb_size: 0.45 },
    ]
}

// ========== PARAMETERS ==========
#[derive(Params)]
struct BassParams {
    #[persist = "editor-state"] pub editor_state: Arc<EguiState>,
    #[id = "osc1_waveform"] pub osc1_waveform: IntParam,
    #[id = "osc1_detune"] pub osc1_detune: FloatParam,
    #[id = "osc2_waveform"] pub osc2_waveform: IntParam,
    #[id = "osc2_detune"] pub osc2_detune: FloatParam,
    #[id = "osc2_mix"] pub osc2_mix: FloatParam,
    #[id = "sub_volume"] pub sub_volume: FloatParam,
    #[id = "unison_voices"] pub unison_voices: IntParam,
    #[id = "unison_spread"] pub unison_spread: FloatParam,
    #[id = "filter_cutoff"] pub filter_cutoff: FloatParam,
    #[id = "filter_resonance"] pub filter_resonance: FloatParam,
    #[id = "filter_env_amount"] pub filter_env_amount: FloatParam,
    #[id = "filter_type"] pub filter_type: IntParam,
    #[id = "filter_slope"] pub filter_slope: IntParam,
    #[id = "drive"] pub drive: FloatParam,
    #[id = "drive_type"] pub drive_type: IntParam,
    #[id = "low_boost"] pub low_boost: FloatParam,
    #[id = "amp_attack"] pub amp_attack: FloatParam,
    #[id = "amp_decay"] pub amp_decay: FloatParam,
    #[id = "amp_sustain"] pub amp_sustain: FloatParam,
    #[id = "amp_release"] pub amp_release: FloatParam,
    #[id = "lfo_rate"] pub lfo_rate: FloatParam,
    #[id = "lfo_depth"] pub lfo_depth: FloatParam,
    #[id = "lfo_waveform"] pub lfo_waveform: IntParam,
    #[id = "lfo_target"] pub lfo_target: IntParam,
    #[id = "portamento"] pub portamento: FloatParam,
    #[id = "arp_on"] pub arp_on: IntParam,
    #[id = "arp_mode"] pub arp_mode: IntParam,
    #[id = "arp_rate"] pub arp_rate: IntParam,
    #[id = "arp_octaves"] pub arp_octaves: IntParam,
    #[id = "delay_mix"] pub delay_mix: FloatParam,
    #[id = "delay_time"] pub delay_time: FloatParam,
    #[id = "delay_feedback"] pub delay_feedback: FloatParam,
    #[id = "reverb_mix"] pub reverb_mix: FloatParam,
    #[id = "reverb_size"] pub reverb_size: FloatParam,
    #[id = "master_gain"] pub master_gain: FloatParam,
}

impl Default for BassParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(680, 580),
            osc1_waveform: IntParam::new("Wave1", 1, IntRange::Linear { min: 0, max: 3 }),
            osc1_detune: FloatParam::new("Det1", 0.0, FloatRange::Linear { min: -100.0, max: 100.0 }),
            osc2_waveform: IntParam::new("Wave2", 1, IntRange::Linear { min: 0, max: 3 }),
            osc2_detune: FloatParam::new("Det2", 7.0, FloatRange::Linear { min: -100.0, max: 100.0 }),
            osc2_mix: FloatParam::new("Mix", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),
            sub_volume: FloatParam::new("SubVol", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),
            unison_voices: IntParam::new("Unison", 4, IntRange::Linear { min: 1, max: 8 }),
            unison_spread: FloatParam::new("Spread", 0.25, FloatRange::Linear { min: 0.0, max: 1.0 }),
            filter_cutoff: FloatParam::new("Cutoff", 600.0, FloatRange::Skewed { min: 20.0, max: 20000.0, factor: FloatRange::skew_factor(-2.0) }),
            filter_resonance: FloatParam::new("Reso", 0.4, FloatRange::Linear { min: 0.0, max: 0.99 }),
            filter_env_amount: FloatParam::new("FltEnv", 0.07, FloatRange::Linear { min: -1.0, max: 1.0 }),
            filter_type: IntParam::new("Type", 0, IntRange::Linear { min: 0, max: 2 }),
            filter_slope: IntParam::new("Slope", 1, IntRange::Linear { min: 0, max: 1 }),
            drive: FloatParam::new("Drive", 0.1, FloatRange::Linear { min: 0.0, max: 1.0 }),
            drive_type: IntParam::new("DriveType", 2, IntRange::Linear { min: 0, max: 3 }),
            low_boost: FloatParam::new("LowBoost", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),
            amp_attack: FloatParam::new("Atk", 0.005, FloatRange::Skewed { min: 0.001, max: 5.0, factor: FloatRange::skew_factor(-2.0) }),
            amp_decay: FloatParam::new("Dec", 0.15, FloatRange::Skewed { min: 0.001, max: 5.0, factor: FloatRange::skew_factor(-2.0) }),
            amp_sustain: FloatParam::new("Sus", 0.7, FloatRange::Linear { min: 0.0, max: 1.0 }),
            amp_release: FloatParam::new("Rel", 0.15, FloatRange::Skewed { min: 0.001, max: 10.0, factor: FloatRange::skew_factor(-2.0) }),
            lfo_rate: FloatParam::new("Rate", 2.0, FloatRange::Skewed { min: 0.01, max: 50.0, factor: FloatRange::skew_factor(-1.5) }),
            lfo_depth: FloatParam::new("Depth", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
            lfo_waveform: IntParam::new("LfoWv", 0, IntRange::Linear { min: 0, max: 3 }),
            lfo_target: IntParam::new("Target", 1, IntRange::Linear { min: 0, max: 2 }),
            portamento: FloatParam::new("Porta", 0.007, FloatRange::Skewed { min: 0.0, max: 1.0, factor: FloatRange::skew_factor(-1.5) }),
            arp_on: IntParam::new("ArpOn", 0, IntRange::Linear { min: 0, max: 1 }),
            arp_mode: IntParam::new("ArpMode", 0, IntRange::Linear { min: 0, max: 3 }),
            arp_rate: IntParam::new("ArpRate", 1, IntRange::Linear { min: 0, max: 3 }),
            arp_octaves: IntParam::new("ArpOct", 0, IntRange::Linear { min: 0, max: 3 }),
            delay_mix: FloatParam::new("DlyMix", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
            delay_time: FloatParam::new("DlyTime", 0.3, FloatRange::Linear { min: 0.05, max: 1.0 }),
            delay_feedback: FloatParam::new("DlyFB", 0.4, FloatRange::Linear { min: 0.0, max: 0.95 }),
            reverb_mix: FloatParam::new("RevMix", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
            reverb_size: FloatParam::new("RevSize", 0.5, FloatRange::Linear { min: 0.1, max: 0.99 }),
            master_gain: FloatParam::new("Vol", 0.6, FloatRange::Linear { min: 0.0, max: 1.0 }),
        }
    }
}

// ========== VOICE ==========
const MAX_VOICES: usize = 16;
const MAX_ARP_NOTES: usize = 16;
const MAX_DELAY_SAMPLES: usize = 96000;
const PANEL_WIDTH: f32 = 284.0;
const KNOB_SIZE: f32 = 42.0;
const KNOB_FRAMES: usize = 128;
const SIDEWOOD_WIDTH: f32 = 40.0;

// Embed images
static KNOB_PNG: &[u8] = include_bytes!("../assets/knob.png");
static SIDEWOOD_PNG: &[u8] = include_bytes!("../assets/sidewood.png");
static LOGO_PNG: &[u8] = include_bytes!("../assets/logo.png");

#[derive(Clone, Copy)]
struct Voice {
    active: bool, note: u8, velocity: f32, sub_phase: f32,
    unison_phases: [f32; 8], env_stage: u8, env_value: f32, env_time: f32, 
    atk_start: f32, rel_start: f32,
    filter_lp: f32, filter_bp: f32, filter_lp2: f32, filter_bp2: f32,
    target_note: f32, current_note: f32,
}

impl Default for Voice {
    fn default() -> Self {
        Self {
            active: false, note: 0, velocity: 0.0, sub_phase: 0.0,
            unison_phases: [0.0; 8], env_stage: 0, env_value: 0.0, env_time: 0.0, 
            atk_start: 0.0, rel_start: 0.0,
            filter_lp: 0.0, filter_bp: 0.0, filter_lp2: 0.0, filter_bp2: 0.0,
            target_note: 69.0, current_note: 69.0,
        }
    }
}

// ========== EFFECTS ==========
struct LowShelf { a0: f32, a1: f32, a2: f32, b1: f32, b2: f32, x1: f32, x2: f32, y1: f32, y2: f32 }
impl LowShelf {
    fn new() -> Self { Self { a0: 1.0, a1: 0.0, a2: 0.0, b1: 0.0, b2: 0.0, x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0 } }
    fn set_params(&mut self, freq: f32, gain_db: f32, sr: f32) {
        let a = 10.0_f32.powf(gain_db / 40.0);
        let w0 = 2.0 * std::f32::consts::PI * freq / sr;
        let (cos_w0, sin_w0) = (w0.cos(), w0.sin());
        let alpha = sin_w0 / 2.0 * 1.414;
        let a0_coef = (a + 1.0) + (a - 1.0) * cos_w0 + 2.0 * a.sqrt() * alpha;
        self.a0 = (a * ((a + 1.0) - (a - 1.0) * cos_w0 + 2.0 * a.sqrt() * alpha)) / a0_coef;
        self.a1 = (2.0 * a * ((a - 1.0) - (a + 1.0) * cos_w0)) / a0_coef;
        self.a2 = (a * ((a + 1.0) - (a - 1.0) * cos_w0 - 2.0 * a.sqrt() * alpha)) / a0_coef;
        self.b1 = (-2.0 * ((a - 1.0) + (a + 1.0) * cos_w0)) / a0_coef;
        self.b2 = ((a + 1.0) + (a - 1.0) * cos_w0 - 2.0 * a.sqrt() * alpha) / a0_coef;
    }
    fn process(&mut self, x: f32) -> f32 {
        // Safety: clamp internal state
        self.x1 = self.x1.clamp(-10.0, 10.0);
        self.x2 = self.x2.clamp(-10.0, 10.0);
        self.y1 = self.y1.clamp(-10.0, 10.0);
        self.y2 = self.y2.clamp(-10.0, 10.0);
        
        let y = self.a0 * x + self.a1 * self.x1 + self.a2 * self.x2 - self.b1 * self.y1 - self.b2 * self.y2;
        self.x2 = self.x1; self.x1 = x; self.y2 = self.y1; self.y1 = y.clamp(-10.0, 10.0);
        y.clamp(-10.0, 10.0)
    }
}

struct SimpleReverb {
    comb_buffers: [Vec<f32>; 4], comb_indices: [usize; 4], comb_filters: [f32; 4],
    ap_buffers: [Vec<f32>; 2], ap_indices: [usize; 2],
}
impl SimpleReverb {
    fn new(sr: f32) -> Self {
        let ct = [0.0297, 0.0371, 0.0411, 0.0437];
        let at = [0.005, 0.0017];
        Self {
            comb_buffers: [vec![0.0; (sr * ct[0]) as usize], vec![0.0; (sr * ct[1]) as usize], vec![0.0; (sr * ct[2]) as usize], vec![0.0; (sr * ct[3]) as usize]],
            comb_indices: [0; 4], comb_filters: [0.0; 4],
            ap_buffers: [vec![0.0; (sr * at[0]) as usize], vec![0.0; (sr * at[1]) as usize]], ap_indices: [0; 2],
        }
    }
    fn process(&mut self, input: f32, size: f32) -> f32 {
        let input = input.clamp(-10.0, 10.0);
        let (fb, damp) = (0.7 + size * 0.28, 0.3);
        let mut out = 0.0;
        for i in 0..4 {
            let len = self.comb_buffers[i].len();
            let delayed = self.comb_buffers[i][self.comb_indices[i]].clamp(-10.0, 10.0);
            self.comb_filters[i] = (delayed * (1.0 - damp) + self.comb_filters[i] * damp).clamp(-10.0, 10.0);
            self.comb_buffers[i][self.comb_indices[i]] = (input + self.comb_filters[i] * fb).clamp(-10.0, 10.0);
            self.comb_indices[i] = (self.comb_indices[i] + 1) % len;
            out += delayed;
        }
        out *= 0.25;
        for i in 0..2 {
            let len = self.ap_buffers[i].len();
            let delayed = self.ap_buffers[i][self.ap_indices[i]].clamp(-10.0, 10.0);
            let temp = out + delayed * 0.5;
            self.ap_buffers[i][self.ap_indices[i]] = (out - delayed * 0.5).clamp(-10.0, 10.0);
            self.ap_indices[i] = (self.ap_indices[i] + 1) % len;
            out = temp;
        }
        out.clamp(-10.0, 10.0)
    }
}

// ========== EDITOR STATE ==========
struct EditorState { new_preset_name: String, selected_category: PresetCategory }
impl Default for EditorState { fn default() -> Self { Self { new_preset_name: String::new(), selected_category: PresetCategory::Init } } }

// ========== PLUGIN ==========
pub struct ArtcodeBass {
    params: Arc<BassParams>,
    sample_rate: f32,
    voices: [Voice; MAX_VOICES],
    lfo_phase: f64,
    last_note: f32,
    last_mono_note: u8,
    arp_notes: [u8; MAX_ARP_NOTES],
    arp_velocities: [f32; MAX_ARP_NOTES],
    arp_note_count: usize,
    arp_index: usize,
    arp_timer: f32,
    arp_playing_note: Option<u8>,
    rng_state: u32,
    delay_buffer: Vec<f32>,
    delay_index: usize,
    reverb: SimpleReverb,
    low_shelf_l: LowShelf,
    low_shelf_r: LowShelf,
    dc_filter_l: f32,
    dc_filter_r: f32,
    current_preset: Arc<AtomicUsize>,
    presets: Arc<Mutex<Vec<PresetData>>>,
    editor_state: Arc<Mutex<EditorState>>,
}

impl Default for ArtcodeBass {
    fn default() -> Self {
        Self {
            params: Arc::new(BassParams::default()),
            sample_rate: 44100.0,
            voices: [Voice::default(); MAX_VOICES],
            lfo_phase: 0.0,
            last_note: 69.0,
            last_mono_note: 0,
            arp_notes: [0; MAX_ARP_NOTES],
            arp_velocities: [0.0; MAX_ARP_NOTES],
            arp_note_count: 0,
            arp_index: 0,
            arp_timer: 0.0,
            arp_playing_note: None,
            rng_state: 12345,
            delay_buffer: vec![0.0; MAX_DELAY_SAMPLES],
            delay_index: 0,
            reverb: SimpleReverb::new(44100.0),
            low_shelf_l: LowShelf::new(),
            low_shelf_r: LowShelf::new(),
            dc_filter_l: 0.0,
            dc_filter_r: 0.0,
            current_preset: Arc::new(AtomicUsize::new(0)),
            presets: Arc::new(Mutex::new(create_factory_presets())),
            editor_state: Arc::new(Mutex::new(EditorState::default())),
        }
    }
}

fn gen_wave(ph: f64, w: i32) -> f32 {
    let p = ph as f32;
    match w { 0 => (p * std::f32::consts::TAU).sin(), 1 => 2.0 * p - 1.0, 2 => if p < 0.5 { 1.0 } else { -1.0 }, _ => 4.0 * (p - (p + 0.5).floor()).abs() - 1.0 }
}

fn apply_drive(sample: f32, drive: f32, drive_type: i32) -> f32 {
    if drive < 0.001 { return sample; }
    let gain = 1.0 + drive * 10.0;
    let driven = sample * gain;
    match drive_type {
        0 => driven.tanh(),
        1 => driven.clamp(-1.0, 1.0),
        2 => if driven > 0.0 { 1.0 - (-driven).exp() } else { -1.0 + driven.exp() },
        _ => { let x = driven.clamp(-1.0, 1.0); x.signum() * (1.0 - (1.0 - x.abs()).powi(3)) }
    }
}

fn arp_rate_to_seconds(rate_idx: i32, bpm: f32) -> f32 {
    // Convert note division to seconds based on BPM
    // 1/4 = 1 beat, 1/8 = 0.5 beat, 1/16 = 0.25 beat, 1/32 = 0.125 beat
    let beats_per_note = match rate_idx { 0 => 1.0, 1 => 0.5, 2 => 0.25, _ => 0.125 };
    let seconds_per_beat = 60.0 / bpm;
    beats_per_note * seconds_per_beat
}

fn normalize_cutoff(v: f32) -> f32 { ((v / 20.0).ln() / (1000.0f32).ln()).clamp(0.0, 1.0) }
fn normalize_attack(v: f32) -> f32 { ((v / 0.001).ln() / (5000.0f32).ln()).clamp(0.0, 1.0) }
fn normalize_decay(v: f32) -> f32 { ((v / 0.001).ln() / (5000.0f32).ln()).clamp(0.0, 1.0) }
fn normalize_release(v: f32) -> f32 { ((v / 0.001).ln() / (10000.0f32).ln()).clamp(0.0, 1.0) }
fn normalize_lfo_rate(v: f32) -> f32 { ((v / 0.01).ln() / (5000.0f32).ln()).clamp(0.0, 1.0) }
fn normalize_porta(v: f32) -> f32 { (v / 1.0).sqrt().clamp(0.0, 1.0) }
fn normalize_detune(v: f32) -> f32 { (v + 100.0) / 200.0 }
fn normalize_filter_env(v: f32) -> f32 { (v + 1.0) / 2.0 }
fn normalize_delay_time(v: f32) -> f32 { (v - 0.05) / 0.95 }

impl ArtcodeBass {
    fn simple_random(&mut self) -> u32 { self.rng_state ^= self.rng_state << 13; self.rng_state ^= self.rng_state >> 17; self.rng_state ^= self.rng_state << 5; self.rng_state }

    fn note_on_voice(&mut self, note: u8, velocity: f32) {
        // Find inactive voice (artcode_synth style)
        let target_note = note as f32 - 12.0; // 1 octave down
        let target_freq = 440.0 * 2.0_f32.powf((target_note - 69.0) / 12.0);
        
        for voice in &mut self.voices {
            if !voice.active {
                *voice = Voice {
                    active: true, note, velocity, sub_phase: 0.0,
                    unison_phases: [0.0; 8], env_stage: 1, env_value: 0.0, env_time: 0.0,
                    atk_start: 0.0, rel_start: 0.0,
                    filter_lp: 0.0, filter_bp: 0.0, filter_lp2: 0.0, filter_bp2: 0.0,
                    target_note, current_note: self.last_note,
                };
                self.last_note = target_note;
                self.last_mono_note = note;
                return;
            }
        }
        // All voices active - steal voice 0
        self.voices[0] = Voice {
            active: true, note, velocity, sub_phase: 0.0,
            unison_phases: [0.0; 8], env_stage: 1, env_value: 0.0, env_time: 0.0,
            atk_start: 0.0, rel_start: 0.0,
            filter_lp: 0.0, filter_bp: 0.0, filter_lp2: 0.0, filter_bp2: 0.0,
            target_note, current_note: self.last_note,
        };
        self.last_note = target_note;
        self.last_mono_note = note;
    }

    fn note_off_voice(&mut self, note: u8) {
        for v in &mut self.voices {
            if v.active && v.note == note && v.env_stage != 4 {
                v.env_stage = 4;
                v.rel_start = v.env_value;
                v.env_time = 0.0;
            }
        }
    }

    fn arp_add_note(&mut self, note: u8, vel: f32) {
        if self.arp_note_count < MAX_ARP_NOTES {
            let was_empty = self.arp_note_count == 0;
            let mut ip = self.arp_note_count;
            for i in 0..self.arp_note_count { if note < self.arp_notes[i] { ip = i; break; } }
            for i in (ip..self.arp_note_count).rev() { self.arp_notes[i + 1] = self.arp_notes[i]; self.arp_velocities[i + 1] = self.arp_velocities[i]; }
            self.arp_notes[ip] = note; self.arp_velocities[ip] = vel; self.arp_note_count += 1;
            
            // Play first note immediately
            if was_empty {
                self.arp_index = 0;
                self.arp_timer = 0.0;
                self.note_on_voice(note, vel);
                self.arp_playing_note = Some(note);
            }
        }
    }

    fn arp_remove_note(&mut self, note: u8) {
        for i in 0..self.arp_note_count {
            if self.arp_notes[i] == note {
                for j in i..self.arp_note_count - 1 { self.arp_notes[j] = self.arp_notes[j + 1]; self.arp_velocities[j] = self.arp_velocities[j + 1]; }
                self.arp_note_count -= 1;
                if self.arp_index >= self.arp_note_count && self.arp_note_count > 0 { self.arp_index = 0; }
                break;
            }
        }
    }

    fn arp_next_note(&mut self, mode: i32, octaves: i32) -> Option<(u8, f32)> {
        if self.arp_note_count == 0 { return None; }
        let ts = self.arp_note_count * octaves as usize;
        match mode {
            0 => { let ni = self.arp_index % self.arp_note_count; let o = (self.arp_index / self.arp_note_count) as i32; let n = self.arp_notes[ni].saturating_add((o * 12) as u8); let v = self.arp_velocities[ni]; self.arp_index = (self.arp_index + 1) % ts; Some((n, v)) }
            1 => { let ri = ts - 1 - (self.arp_index % ts); let ni = ri % self.arp_note_count; let o = (ri / self.arp_note_count) as i32; let n = self.arp_notes[ni].saturating_add((o * 12) as u8); let v = self.arp_velocities[ni]; self.arp_index = (self.arp_index + 1) % ts; Some((n, v)) }
            2 => { let cl = if ts > 1 { ts * 2 - 2 } else { 1 }; let pos = self.arp_index % cl; let ai = if pos < ts { pos } else { ts * 2 - 2 - pos }; let ni = ai % self.arp_note_count; let o = (ai / self.arp_note_count) as i32; let n = self.arp_notes[ni].saturating_add((o * 12) as u8); let v = self.arp_velocities[ni]; self.arp_index = (self.arp_index + 1) % cl; Some((n, v)) }
            _ => { let i = (self.simple_random() as usize) % ts; let ni = i % self.arp_note_count; let o = (i / self.arp_note_count) as i32; let n = self.arp_notes[ni].saturating_add((o * 12) as u8); let v = self.arp_velocities[ni]; Some((n, v)) }
        }
    }
}

// ========== GUI DRAWING ==========
fn load_texture_from_bytes(ctx: &egui::Context, name: &str, bytes: &[u8]) -> egui::TextureHandle {
    let image = image::load_from_memory(bytes).expect("Failed to load image");
    let size = [image.width() as _, image.height() as _];
    let rgba = image.to_rgba8();
    let pixels = rgba.as_flat_samples();
    ctx.load_texture(name, egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()), egui::TextureOptions::LINEAR)
}

fn draw_knob(ui: &mut egui::Ui, value: f32, label: &str) -> Option<f32> {
    let size = KNOB_SIZE;
    let (rect, response) = ui.allocate_exact_size(egui::vec2(size, size + 14.0), egui::Sense::drag());
    let mut nv = None;
    if response.dragged() { nv = Some((value - response.drag_delta().y * 0.005).clamp(0.0, 1.0)); }
    if ui.is_rect_visible(rect) {
        let ctx = ui.ctx();
        let knob_tex = load_texture_from_bytes(ctx, "knob", KNOB_PNG);
        let p = ui.painter();
        let frame_idx = ((value * (KNOB_FRAMES - 1) as f32) as usize).min(KNOB_FRAMES - 1);
        let uv_top = frame_idx as f32 / KNOB_FRAMES as f32;
        let uv_bottom = (frame_idx + 1) as f32 / KNOB_FRAMES as f32;
        let knob_rect = egui::Rect::from_min_size(rect.min, egui::vec2(size, size));
        p.image(knob_tex.id(), knob_rect, egui::Rect::from_min_max(egui::pos2(0.0, uv_top), egui::pos2(1.0, uv_bottom)), egui::Color32::WHITE);
        p.text(egui::pos2(rect.center().x, rect.max.y), egui::Align2::CENTER_BOTTOM, label, egui::FontId::proportional(9.0), egui::Color32::from_rgb(180, 180, 190));
    }
    nv
}

fn draw_selector(ui: &mut egui::Ui, current: i32, options: &[&str]) -> Option<i32> {
    let mut result = None;
    let orange = egui::Color32::from_rgb(255, 100, 50);
    ui.horizontal(|ui| {
        for (i, name) in options.iter().enumerate() {
            let sel = i as i32 == current;
            if ui.add(egui::Button::new(egui::RichText::new(*name).size(9.0).color(if sel { egui::Color32::from_rgb(20, 20, 30) } else { egui::Color32::from_rgb(130, 130, 140) }))
                .fill(if sel { orange } else { egui::Color32::from_rgb(45, 45, 60) }).min_size(egui::vec2(28.0, 16.0))).clicked() { result = Some(i as i32); }
        }
    });
    result
}

fn draw_toggle(ui: &mut egui::Ui, on: bool, label: &str) -> bool {
    let orange = egui::Color32::from_rgb(255, 100, 50);
    ui.add(egui::Button::new(egui::RichText::new(label).size(9.0).color(if on { egui::Color32::from_rgb(20, 20, 30) } else { egui::Color32::from_rgb(130, 130, 140) }))
        .fill(if on { orange } else { egui::Color32::from_rgb(45, 45, 60) }).min_size(egui::vec2(32.0, 16.0))).clicked()
}

// ========== PLUGIN IMPL ==========
impl Plugin for ArtcodeBass {
    const NAME: &'static str = "artcode Bass";
    const VENDOR: &'static str = "artcode";
    const URL: &'static str = "https://artcode.jp";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout { main_input_channels: None, main_output_channels: NonZeroU32::new(2), ..AudioIOLayout::const_default() }];
    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> { self.params.clone() }

    fn editor(&mut self, _: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();
        let current_preset = self.current_preset.clone();
        let presets = self.presets.clone();
        let editor_state = self.editor_state.clone();

        create_egui_editor(self.params.editor_state.clone(), (), |_, _| {},
            move |ctx, setter, _| {
                let orange = egui::Color32::from_rgb(255, 100, 50);
                let bg = egui::Color32::from_rgb(24, 25, 32);
                let panel = egui::Color32::from_rgb(32, 34, 44);
                let row_height = 80.0;

                // Load textures
                let sidewood_tex = load_texture_from_bytes(ctx, "sidewood", SIDEWOOD_PNG);
                let logo_tex = load_texture_from_bytes(ctx, "logo", LOGO_PNG);

                egui::CentralPanel::default().frame(egui::Frame::default().fill(bg)).show(ctx, |ui| {
                    let total_rect = ui.available_rect_before_wrap();
                    
                    // Draw sidewood on left
                    let left_rect = egui::Rect::from_min_size(total_rect.min, egui::vec2(SIDEWOOD_WIDTH, total_rect.height()));
                    ui.painter().image(sidewood_tex.id(), left_rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
                    
                    // Draw sidewood on right (mirrored)
                    let right_rect = egui::Rect::from_min_size(egui::pos2(total_rect.max.x - SIDEWOOD_WIDTH, total_rect.min.y), egui::vec2(SIDEWOOD_WIDTH, total_rect.height()));
                    ui.painter().image(sidewood_tex.id(), right_rect, egui::Rect::from_min_max(egui::pos2(1.0, 0.0), egui::pos2(0.0, 1.0)), egui::Color32::WHITE);

                    // Main content area
                    let content_rect = egui::Rect::from_min_max(
                        egui::pos2(total_rect.min.x + SIDEWOOD_WIDTH + 4.0, total_rect.min.y + 4.0),
                        egui::pos2(total_rect.max.x - SIDEWOOD_WIDTH - 4.0, total_rect.max.y - 4.0)
                    );
                    ui.allocate_ui_at_rect(content_rect, |ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(4.0, 4.0);

                    // Logo
                    ui.horizontal(|ui| {
                        ui.add_space(4.0);
                        let logo_size = egui::vec2(150.0, 17.5);
                        let (logo_rect, _) = ui.allocate_exact_size(logo_size, egui::Sense::hover());
                        ui.painter().image(logo_tex.id(), logo_rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
                    });
                    ui.add_space(2.0);

                    // Preset selector
                    egui::Frame::default().fill(panel).corner_radius(4.0).inner_margin(6.0).show(ui, |ui| {
                        ui.set_min_width(PANEL_WIDTH * 2.0 + 4.0);
                        let mut es = editor_state.lock().unwrap();
                        let presets_lock = presets.lock().unwrap();
                        let cur = current_preset.load(Ordering::Relaxed);
                        let cur_name = if cur < presets_lock.len() { presets_lock[cur].name.clone() } else { "Init".to_string() };
                        let cur_cat = if cur < presets_lock.len() { presets_lock[cur].category } else { PresetCategory::Init };
                        es.selected_category = cur_cat;
                        drop(presets_lock);

                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("PRESET").size(9.0).color(orange));
                            ui.add_space(4.0);
                            egui::ComboBox::from_id_salt("cat_select").selected_text(es.selected_category.name()).width(50.0).show_ui(ui, |ui| {
                                for cat in PresetCategory::all() {
                                    if ui.selectable_label(es.selected_category == *cat, cat.name()).clicked() {
                                        es.selected_category = *cat;
                                        let presets_lock = presets.lock().unwrap();
                                        for (i, p) in presets_lock.iter().enumerate() { if p.category == *cat { current_preset.store(i, Ordering::Relaxed); break; } }
                                    }
                                }
                            });
                            egui::ComboBox::from_id_salt("preset_select").selected_text(&cur_name).width(100.0).show_ui(ui, |ui| {
                                let presets_lock = presets.lock().unwrap();
                                for (i, p) in presets_lock.iter().enumerate() {
                                    if p.category == es.selected_category {
                                        if ui.selectable_label(i == cur, &p.name).clicked() {
                                            current_preset.store(i, Ordering::Relaxed);
                                            setter.set_parameter_normalized(&params.osc1_waveform, p.osc1_wave as f32 / 3.0);
                                            setter.set_parameter_normalized(&params.osc1_detune, normalize_detune(p.osc1_det));
                                            setter.set_parameter_normalized(&params.osc2_waveform, p.osc2_wave as f32 / 3.0);
                                            setter.set_parameter_normalized(&params.osc2_detune, normalize_detune(p.osc2_det));
                                            setter.set_parameter_normalized(&params.osc2_mix, p.osc2_mix);
                                            setter.set_parameter_normalized(&params.sub_volume, p.sub_vol);
                                            setter.set_parameter_normalized(&params.unison_voices, (p.unison - 1) as f32 / 7.0);
                                            setter.set_parameter_normalized(&params.unison_spread, p.spread);
                                            setter.set_parameter_normalized(&params.filter_cutoff, normalize_cutoff(p.filter_cut));
                                            setter.set_parameter_normalized(&params.filter_resonance, p.filter_res / 0.99);
                                            setter.set_parameter_normalized(&params.filter_env_amount, normalize_filter_env(p.filter_env));
                                            setter.set_parameter_normalized(&params.filter_type, p.filter_type as f32 / 2.0);
                                            setter.set_parameter_normalized(&params.filter_slope, p.filter_slope as f32);
                                            setter.set_parameter_normalized(&params.drive, p.drive);
                                            setter.set_parameter_normalized(&params.drive_type, p.drive_type as f32 / 3.0);
                                            setter.set_parameter_normalized(&params.low_boost, p.low_boost);
                                            setter.set_parameter_normalized(&params.amp_attack, normalize_attack(p.attack));
                                            setter.set_parameter_normalized(&params.amp_decay, normalize_decay(p.decay));
                                            setter.set_parameter_normalized(&params.amp_sustain, p.sustain);
                                            setter.set_parameter_normalized(&params.amp_release, normalize_release(p.release));
                                            setter.set_parameter_normalized(&params.lfo_rate, normalize_lfo_rate(p.lfo_rate));
                                            setter.set_parameter_normalized(&params.lfo_depth, p.lfo_depth);
                                            setter.set_parameter_normalized(&params.lfo_waveform, p.lfo_wave as f32 / 3.0);
                                            setter.set_parameter_normalized(&params.lfo_target, p.lfo_target as f32 / 2.0);
                                            setter.set_parameter_normalized(&params.portamento, normalize_porta(p.porta));
                                            setter.set_parameter_normalized(&params.delay_mix, p.delay_mix);
                                            setter.set_parameter_normalized(&params.delay_time, normalize_delay_time(p.delay_time));
                                            setter.set_parameter_normalized(&params.delay_feedback, p.delay_fb / 0.95);
                                            setter.set_parameter_normalized(&params.reverb_mix, p.reverb_mix);
                                            setter.set_parameter_normalized(&params.reverb_size, (p.reverb_size - 0.1) / 0.89);
                                        }
                                    }
                                }
                            });
                            ui.add_space(8.0);
                            ui.add(egui::TextEdit::singleline(&mut es.new_preset_name).desired_width(80.0).hint_text("New name"));
                            if ui.add(egui::Button::new(egui::RichText::new("Save").size(9.0)).min_size(egui::vec2(40.0, 16.0))).clicked() {
                                if !es.new_preset_name.is_empty() {
                                    let new_preset = PresetData {
                                        name: es.new_preset_name.clone(), category: PresetCategory::User,
                                        osc1_wave: params.osc1_waveform.value(), osc1_det: params.osc1_detune.value(),
                                        osc2_wave: params.osc2_waveform.value(), osc2_det: params.osc2_detune.value(), osc2_mix: params.osc2_mix.value(),
                                        sub_vol: params.sub_volume.value(), unison: params.unison_voices.value(), spread: params.unison_spread.value(),
                                        filter_cut: params.filter_cutoff.value(), filter_res: params.filter_resonance.value(), filter_env: params.filter_env_amount.value(),
                                        filter_type: params.filter_type.value(), filter_slope: params.filter_slope.value(),
                                        drive: params.drive.value(), drive_type: params.drive_type.value(), low_boost: params.low_boost.value(),
                                        attack: params.amp_attack.value(), decay: params.amp_decay.value(), sustain: params.amp_sustain.value(), release: params.amp_release.value(),
                                        lfo_rate: params.lfo_rate.value(), lfo_depth: params.lfo_depth.value(), lfo_wave: params.lfo_waveform.value(), lfo_target: params.lfo_target.value(),
                                        porta: params.portamento.value(), delay_mix: params.delay_mix.value(), delay_time: params.delay_time.value(), delay_fb: params.delay_feedback.value(),
                                        reverb_mix: params.reverb_mix.value(), reverb_size: params.reverb_size.value(),
                                    };
                                    let mut presets_lock = presets.lock().unwrap();
                                    let new_idx = presets_lock.len();
                                    presets_lock.push(new_preset);
                                    drop(presets_lock);
                                    current_preset.store(new_idx, Ordering::Relaxed);
                                    es.selected_category = PresetCategory::User;
                                    es.new_preset_name.clear();
                                }
                            }
                        });
                    });

                    ui.add_space(2.0);

                    // Row 1: OSC1 + OSC2/SUB
                    ui.horizontal(|ui| {
                        egui::Frame::default().fill(panel).corner_radius(4.0).inner_margin(6.0).show(ui, |ui| {
                            ui.set_min_width(PANEL_WIDTH); ui.set_min_height(row_height);
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new("OSC 1").size(9.0).color(orange));
                                if let Some(v) = draw_selector(ui, params.osc1_waveform.value(), &["Sin", "Saw", "Sqr", "Tri"]) { setter.set_parameter_normalized(&params.osc1_waveform, v as f32 / 3.0); }
                                ui.add_space(2.0);
                                ui.horizontal(|ui| {
                                    if let Some(v) = draw_knob(ui, params.osc1_detune.modulated_normalized_value(), "Detune") { setter.set_parameter_normalized(&params.osc1_detune, v); }
                                    if let Some(v) = draw_knob(ui, params.master_gain.modulated_normalized_value(), "Volume") { setter.set_parameter_normalized(&params.master_gain, v); }
                                });
                            });
                        });
                        egui::Frame::default().fill(panel).corner_radius(4.0).inner_margin(6.0).show(ui, |ui| {
                            ui.set_min_width(PANEL_WIDTH); ui.set_min_height(row_height);
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new("OSC 2 / SUB").size(9.0).color(orange));
                                if let Some(v) = draw_selector(ui, params.osc2_waveform.value(), &["Sin", "Saw", "Sqr", "Tri"]) { setter.set_parameter_normalized(&params.osc2_waveform, v as f32 / 3.0); }
                                ui.add_space(2.0);
                                ui.horizontal(|ui| {
                                    if let Some(v) = draw_knob(ui, params.osc2_detune.modulated_normalized_value(), "Detune") { setter.set_parameter_normalized(&params.osc2_detune, v); }
                                    if let Some(v) = draw_knob(ui, params.osc2_mix.modulated_normalized_value(), "Mix") { setter.set_parameter_normalized(&params.osc2_mix, v); }
                                    if let Some(v) = draw_knob(ui, params.sub_volume.modulated_normalized_value(), "Sub") { setter.set_parameter_normalized(&params.sub_volume, v); }
                                });
                            });
                        });
                    });

                    ui.add_space(2.0);

                    // Row 2: UNISON + FILTER
                    ui.horizontal(|ui| {
                        egui::Frame::default().fill(panel).corner_radius(4.0).inner_margin(6.0).show(ui, |ui| {
                            ui.set_min_width(PANEL_WIDTH); ui.set_min_height(row_height);
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new("UNISON").size(9.0).color(orange));
                                ui.add_space(2.0);
                                ui.horizontal(|ui| {
                                    if let Some(v) = draw_selector(ui, params.unison_voices.value() - 1, &["1", "2", "3", "4", "5", "6", "7", "8"]) { setter.set_parameter_normalized(&params.unison_voices, v as f32 / 7.0); }
                                });
                                ui.horizontal(|ui| {
                                    if let Some(v) = draw_knob(ui, params.unison_spread.modulated_normalized_value(), "Spread") { setter.set_parameter_normalized(&params.unison_spread, v); }
                                    if let Some(v) = draw_knob(ui, params.portamento.modulated_normalized_value(), "Porta") { setter.set_parameter_normalized(&params.portamento, v); }
                                });
                            });
                        });
                        egui::Frame::default().fill(panel).corner_radius(4.0).inner_margin(6.0).show(ui, |ui| {
                            ui.set_min_width(PANEL_WIDTH); ui.set_min_height(row_height);
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("FILTER").size(9.0).color(orange));
                                    ui.add_space(4.0);
                                    if let Some(v) = draw_selector(ui, params.filter_type.value(), &["LP", "HP", "BP"]) { setter.set_parameter_normalized(&params.filter_type, v as f32 / 2.0); }
                                    ui.add_space(4.0);
                                    if let Some(v) = draw_selector(ui, params.filter_slope.value(), &["12", "24"]) { setter.set_parameter_normalized(&params.filter_slope, v as f32); }
                                });
                                ui.add_space(2.0);
                                ui.horizontal(|ui| {
                                    if let Some(v) = draw_knob(ui, params.filter_cutoff.modulated_normalized_value(), "Cutoff") { setter.set_parameter_normalized(&params.filter_cutoff, v); }
                                    if let Some(v) = draw_knob(ui, params.filter_resonance.modulated_normalized_value(), "Reso") { setter.set_parameter_normalized(&params.filter_resonance, v); }
                                    if let Some(v) = draw_knob(ui, params.filter_env_amount.modulated_normalized_value(), "Env") { setter.set_parameter_normalized(&params.filter_env_amount, v); }
                                });
                            });
                        });
                    });

                    ui.add_space(2.0);

                    // Row 3: DRIVE + EFFECTS
                    ui.horizontal(|ui| {
                        egui::Frame::default().fill(panel).corner_radius(4.0).inner_margin(6.0).show(ui, |ui| {
                            ui.set_min_width(PANEL_WIDTH); ui.set_min_height(row_height);
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("DRIVE").size(9.0).color(orange));
                                    ui.add_space(4.0);
                                    if let Some(v) = draw_selector(ui, params.drive_type.value(), &["Soft", "Hard", "Tube", "Fuzz"]) { setter.set_parameter_normalized(&params.drive_type, v as f32 / 3.0); }
                                });
                                ui.add_space(2.0);
                                ui.horizontal(|ui| {
                                    if let Some(v) = draw_knob(ui, params.drive.modulated_normalized_value(), "Drive") { setter.set_parameter_normalized(&params.drive, v); }
                                    if let Some(v) = draw_knob(ui, params.low_boost.modulated_normalized_value(), "Low+") { setter.set_parameter_normalized(&params.low_boost, v); }
                                });
                            });
                        });
                        egui::Frame::default().fill(panel).corner_radius(4.0).inner_margin(6.0).show(ui, |ui| {
                            ui.set_min_width(PANEL_WIDTH); ui.set_min_height(row_height);
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new("EFFECTS").size(9.0).color(orange));
                                ui.add_space(2.0);
                                ui.horizontal(|ui| {
                                    if let Some(v) = draw_knob(ui, params.delay_mix.modulated_normalized_value(), "Dly") { setter.set_parameter_normalized(&params.delay_mix, v); }
                                    if let Some(v) = draw_knob(ui, params.delay_time.modulated_normalized_value(), "Time") { setter.set_parameter_normalized(&params.delay_time, v); }
                                    if let Some(v) = draw_knob(ui, params.reverb_mix.modulated_normalized_value(), "Rev") { setter.set_parameter_normalized(&params.reverb_mix, v); }
                                    if let Some(v) = draw_knob(ui, params.reverb_size.modulated_normalized_value(), "Size") { setter.set_parameter_normalized(&params.reverb_size, v); }
                                });
                            });
                        });
                    });

                    ui.add_space(2.0);

                    // Row 4: ENVELOPE + LFO
                    ui.horizontal(|ui| {
                        egui::Frame::default().fill(panel).corner_radius(4.0).inner_margin(6.0).show(ui, |ui| {
                            ui.set_min_width(PANEL_WIDTH); ui.set_min_height(row_height);
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new("ENVELOPE").size(9.0).color(orange));
                                ui.add_space(2.0);
                                ui.horizontal(|ui| {
                                    if let Some(v) = draw_knob(ui, params.amp_attack.modulated_normalized_value(), "A") { setter.set_parameter_normalized(&params.amp_attack, v); }
                                    if let Some(v) = draw_knob(ui, params.amp_decay.modulated_normalized_value(), "D") { setter.set_parameter_normalized(&params.amp_decay, v); }
                                    if let Some(v) = draw_knob(ui, params.amp_sustain.modulated_normalized_value(), "S") { setter.set_parameter_normalized(&params.amp_sustain, v); }
                                    if let Some(v) = draw_knob(ui, params.amp_release.modulated_normalized_value(), "R") { setter.set_parameter_normalized(&params.amp_release, v); }
                                });
                            });
                        });
                        egui::Frame::default().fill(panel).corner_radius(4.0).inner_margin(6.0).show(ui, |ui| {
                            ui.set_min_width(PANEL_WIDTH); ui.set_min_height(row_height);
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("LFO").size(9.0).color(orange));
                                    ui.add_space(4.0);
                                    if let Some(v) = draw_selector(ui, params.lfo_waveform.value(), &["Sin", "Saw", "Sqr", "Tri"]) { setter.set_parameter_normalized(&params.lfo_waveform, v as f32 / 3.0); }
                                    ui.add_space(4.0);
                                    if let Some(v) = draw_selector(ui, params.lfo_target.value(), &["Pit", "Flt", "Amp"]) { setter.set_parameter_normalized(&params.lfo_target, v as f32 / 2.0); }
                                });
                                ui.add_space(2.0);
                                ui.horizontal(|ui| {
                                    if let Some(v) = draw_knob(ui, params.lfo_rate.modulated_normalized_value(), "Rate") { setter.set_parameter_normalized(&params.lfo_rate, v); }
                                    if let Some(v) = draw_knob(ui, params.lfo_depth.modulated_normalized_value(), "Depth") { setter.set_parameter_normalized(&params.lfo_depth, v); }
                                });
                            });
                        });
                    });

                    ui.add_space(2.0);

                    // Row 5: Arpeggiator
                    ui.horizontal(|ui| {
                        egui::Frame::default().fill(panel).corner_radius(4.0).inner_margin(6.0).show(ui, |ui| {
                            ui.set_min_width(PANEL_WIDTH * 2.0 + 4.0);
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("ARPEGGIATOR").size(9.0).color(orange));
                                    ui.add_space(8.0);
                                    let arp_on = params.arp_on.value() == 1;
                                    if draw_toggle(ui, arp_on, if arp_on { "ON" } else { "OFF" }) { setter.set_parameter_normalized(&params.arp_on, if arp_on { 0.0 } else { 1.0 }); }
                                });
                                ui.add_space(4.0);
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("Mode").size(8.0).color(egui::Color32::GRAY));
                                    if let Some(v) = draw_selector(ui, params.arp_mode.value(), &["Up", "Dn", "U/D", "Rnd"]) { setter.set_parameter_normalized(&params.arp_mode, v as f32 / 3.0); }
                                    ui.add_space(10.0);
                                    ui.label(egui::RichText::new("Speed").size(8.0).color(egui::Color32::GRAY));
                                    if let Some(v) = draw_selector(ui, params.arp_rate.value(), &["1/4", "1/8", "1/16", "1/32"]) { setter.set_parameter_normalized(&params.arp_rate, v as f32 / 3.0); }
                                    ui.add_space(10.0);
                                    ui.label(egui::RichText::new("Oct").size(8.0).color(egui::Color32::GRAY));
                                    if let Some(v) = draw_selector(ui, params.arp_octaves.value(), &["1", "2", "3", "4"]) { setter.set_parameter_normalized(&params.arp_octaves, v as f32 / 3.0); }
                                });
                            });
                        });
                    });
                    }); // allocate_ui_at_rect
                });
            },
        )
    }

    fn initialize(&mut self, _: &AudioIOLayout, cfg: &BufferConfig, _: &mut impl InitContext<Self>) -> bool {
        self.sample_rate = cfg.sample_rate;
        self.delay_buffer = vec![0.0; MAX_DELAY_SAMPLES];
        self.reverb = SimpleReverb::new(cfg.sample_rate);
        self.voices = [Voice::default(); MAX_VOICES];
        true
    }

    fn reset(&mut self) {
        self.voices = [Voice::default(); MAX_VOICES];
        self.lfo_phase = 0.0;
        self.last_note = 69.0;
        self.last_mono_note = 0;
        self.arp_notes = [0; MAX_ARP_NOTES];
        self.arp_velocities = [0.0; MAX_ARP_NOTES];
        self.arp_note_count = 0;
        self.arp_index = 0;
        self.arp_timer = 0.0;
        self.arp_playing_note = None;
        self.delay_buffer.fill(0.0);
        self.delay_index = 0;
        self.dc_filter_l = 0.0;
        self.dc_filter_r = 0.0;
    }

    fn process(&mut self, buffer: &mut Buffer, _: &mut AuxiliaryBuffers, ctx: &mut impl ProcessContext<Self>) -> ProcessStatus {
        let sr = self.sample_rate;
        let dt = 1.0 / sr;

        let osc1_w = self.params.osc1_waveform.value();
        let osc1_det = self.params.osc1_detune.value();
        let osc2_w = self.params.osc2_waveform.value();
        let osc2_det = self.params.osc2_detune.value();
        let osc2_mix = self.params.osc2_mix.value();
        let sub_vol = self.params.sub_volume.value();
        let unison_count = self.params.unison_voices.value() as usize;
        let spread = self.params.unison_spread.value() * 50.0;

        let flt_cut = self.params.filter_cutoff.value();
        let flt_res = self.params.filter_resonance.value();
        let flt_type = self.params.filter_type.value();
        let flt_slope = self.params.filter_slope.value();
        let flt_env = self.params.filter_env_amount.value();

        let drive = self.params.drive.value();
        let drive_type = self.params.drive_type.value();
        let low_boost = self.params.low_boost.value();

        let atk = self.params.amp_attack.value();
        let dec = self.params.amp_decay.value();
        let sus = self.params.amp_sustain.value();
        let rel = self.params.amp_release.value();

        let lfo_w = self.params.lfo_waveform.value();
        let lfo_r = self.params.lfo_rate.value() as f64;
        let lfo_d = self.params.lfo_depth.value();
        let lfo_t = self.params.lfo_target.value();

        let porta_time = self.params.portamento.value();
        let gain = self.params.master_gain.value();

        let arp_on = self.params.arp_on.value() == 1;
        let arp_mode = self.params.arp_mode.value();
        let arp_rate_idx = self.params.arp_rate.value();
        let arp_octaves = self.params.arp_octaves.value() + 1;

        // Get tempo from DAW (default 120 BPM if not available)
        let tempo = ctx.transport().tempo.unwrap_or(120.0) as f32;
        let arp_interval = arp_rate_to_seconds(arp_rate_idx, tempo);

        let delay_mix = self.params.delay_mix.value();
        let delay_time = self.params.delay_time.value();
        let delay_fb = self.params.delay_feedback.value();
        let reverb_mix = self.params.reverb_mix.value();
        let reverb_size = self.params.reverb_size.value();

        while let Some(ev) = ctx.next_event() {
            match ev {
                NoteEvent::NoteOn { note, velocity, .. } => {
                    if arp_on { self.arp_add_note(note, velocity); }
                    else { self.note_on_voice(note, velocity); }
                }
                NoteEvent::NoteOff { note, .. } => {
                    if arp_on {
                        self.arp_remove_note(note);
                        if self.arp_note_count == 0 {
                            if let Some(p) = self.arp_playing_note { self.note_off_voice(p); self.arp_playing_note = None; }
                        }
                    } else { self.note_off_voice(note); }
                }
                _ => {}
            }
        }

        if low_boost > 0.0 {
            let boost_db = low_boost * 12.0;
            self.low_shelf_l.set_params(100.0, boost_db, sr);
            self.low_shelf_r.set_params(100.0, boost_db, sr);
        }

        let delay_samples = ((delay_time * sr) as usize).min(MAX_DELAY_SAMPLES - 1);

        for mut frame in buffer.iter_samples() {
            // Arpeggiator (synced to DAW tempo) - artcode_synth style
            if arp_on && self.arp_note_count > 0 {
                self.arp_timer += dt;
                if self.arp_timer >= arp_interval {
                    self.arp_timer -= arp_interval;
                    // Release previous note, then play next
                    if let Some(p) = self.arp_playing_note { self.note_off_voice(p); }
                    if let Some((n, v)) = self.arp_next_note(arp_mode, arp_octaves) {
                        self.note_on_voice(n, v);
                        self.arp_playing_note = Some(n);
                    }
                }
            }

            // LFO
            let lfo = gen_wave(self.lfo_phase, lfo_w);
            self.lfo_phase += lfo_r / sr as f64;
            if self.lfo_phase >= 1.0 { self.lfo_phase -= 1.0; }

            let mut out = 0.0f32;

            for v in &mut self.voices {
                if !v.active { continue; }

                // Portamento
                if porta_time > 0.001 {
                    let rate = 1.0 / (porta_time * sr);
                    let diff = v.target_note - v.current_note;
                    if diff.abs() > 0.01 { v.current_note += diff.signum() * rate.min(diff.abs()); }
                    else { v.current_note = v.target_note; }
                } else { v.current_note = v.target_note; }

                // Envelope with smooth transitions
                let env = match v.env_stage {
                    1 => { 
                        v.env_time += dt;
                        if v.env_time >= atk { 
                            v.env_stage = 2; v.env_time = 0.0; v.env_value = 1.0; 
                        } else { 
                            // Linear attack from atk_start to 1.0
                            let progress = v.env_time / atk;
                            v.env_value = v.atk_start + (1.0 - v.atk_start) * progress;
                        }
                        v.env_value
                    }
                    2 => { v.env_time += dt; if v.env_time >= dec { v.env_stage = 3; v.env_value = sus; } else { v.env_value = 1.0 - (1.0 - sus) * v.env_time / dec; } v.env_value }
                    3 => sus,
                    4 => { 
                        v.env_time += dt; 
                        if v.env_time >= rel { 
                            v.env_value = 0.0;
                            v.active = false; 
                            0.0 
                        } else { 
                            v.env_value = v.rel_start * (1.0 - v.env_time / rel);
                            v.env_value
                        } 
                    }
                    _ => 0.0,
                };

                if !v.active { continue; }

                let base_freq = 440.0 * 2.0_f32.powf((v.current_note - 69.0) / 12.0);
                let pm = if lfo_t == 0 { 2.0_f32.powf(lfo * lfo_d * 0.5) } else { 1.0 };

                // Unison oscillators
                let mut osc1_out = 0.0f32;
                let mut osc2_out = 0.0f32;

                for i in 0..unison_count {
                    let detune_offset = if unison_count > 1 {
                        (i as f32 / (unison_count - 1) as f32 - 0.5) * 2.0 * spread
                    } else { 0.0 };

                    let freq1 = base_freq * 2.0_f32.powf((osc1_det + detune_offset) / 1200.0) * pm;
                    let freq2 = base_freq * 2.0_f32.powf((osc2_det + detune_offset) / 1200.0) * pm;

                    v.unison_phases[i] = (v.unison_phases[i] + freq1 / sr) % 1.0;
                    osc1_out += gen_wave(v.unison_phases[i] as f64, osc1_w);

                    let phase2 = (v.unison_phases[i] + 0.3) % 1.0;
                    let _ = freq2; // Use same detuned phase relationship
                    osc2_out += gen_wave(phase2 as f64, osc2_w);
                }
                osc1_out /= unison_count as f32;
                osc2_out /= unison_count as f32;

                // Sub oscillator
                let sub_freq = base_freq * 0.5 * pm;
                v.sub_phase = (v.sub_phase + sub_freq / sr) % 1.0;
                let sub_out = (v.sub_phase * std::f32::consts::TAU).sin();

                let osc_mix = osc1_out * (1.0 - osc2_mix) + osc2_out * osc2_mix + sub_out * sub_vol;

                // Drive
                let driven = apply_drive(osc_mix, drive, drive_type);

                // Filter
                let cm = env * flt_env * 5000.0 + if lfo_t == 1 { lfo * lfo_d * 2000.0 } else { 0.0 };
                let cut = (flt_cut + cm).clamp(20.0, 20000.0);
                let g = (std::f32::consts::PI * cut / sr).tan().min(1.0);
                let k = 2.0 - 2.0 * flt_res.min(0.98);

                // Clamp filter state to prevent blowup
                v.filter_lp = v.filter_lp.clamp(-10.0, 10.0);
                v.filter_bp = v.filter_bp.clamp(-10.0, 10.0);

                let hp = (driven - v.filter_lp - k * v.filter_bp) / (1.0 + k * g + g * g);
                let bp = g * hp + v.filter_bp;
                let lp = g * bp + v.filter_lp;
                v.filter_bp = (bp + g * hp).clamp(-10.0, 10.0);
                v.filter_lp = (lp + g * bp).clamp(-10.0, 10.0);

                let mut flt_out = match flt_type { 0 => lp, 1 => hp, _ => bp };

                // 24dB mode
                if flt_slope == 1 {
                    v.filter_lp2 = v.filter_lp2.clamp(-10.0, 10.0);
                    v.filter_bp2 = v.filter_bp2.clamp(-10.0, 10.0);

                    let hp2 = (flt_out - v.filter_lp2 - k * v.filter_bp2) / (1.0 + k * g + g * g);
                    let bp2 = g * hp2 + v.filter_bp2;
                    let lp2 = g * bp2 + v.filter_lp2;
                    v.filter_bp2 = (bp2 + g * hp2).clamp(-10.0, 10.0);
                    v.filter_lp2 = (lp2 + g * bp2).clamp(-10.0, 10.0);
                    flt_out = match flt_type { 0 => lp2, 1 => hp2, _ => bp2 };
                }

                // Safety clamp output
                flt_out = flt_out.clamp(-10.0, 10.0);

                let am = if lfo_t == 2 { 1.0 - lfo_d * 0.5 * (1.0 - lfo) } else { 1.0 };
                out += flt_out * env * v.velocity * am;
            }

            // Low boost EQ
            let (mut out_l, mut out_r) = (out, out);
            if low_boost > 0.0 {
                out_l = self.low_shelf_l.process(out_l);
                out_r = self.low_shelf_r.process(out_r);
            }

            // Delay
            if delay_mix > 0.0 {
                let dri = (self.delay_index + MAX_DELAY_SAMPLES - delay_samples) % MAX_DELAY_SAMPLES;
                let delayed = self.delay_buffer[dri];
                self.delay_buffer[self.delay_index] = out_l + delayed * delay_fb;
                self.delay_index = (self.delay_index + 1) % MAX_DELAY_SAMPLES;
                out_l += delayed * delay_mix;
                out_r += delayed * delay_mix;
            }

            // Reverb
            if reverb_mix > 0.0 {
                let rev = self.reverb.process((out_l + out_r) * 0.5, reverb_size);
                out_l = out_l * (1.0 - reverb_mix) + rev * reverb_mix;
                out_r = out_r * (1.0 - reverb_mix) + rev * reverb_mix;
            }

            // DC removal filter (simple highpass)
            let dc_coef = 0.995;
            let new_dc_l = out_l + dc_coef * self.dc_filter_l;
            out_l = new_dc_l - self.dc_filter_l;
            self.dc_filter_l = new_dc_l;
            
            let new_dc_r = out_r + dc_coef * self.dc_filter_r;
            out_r = new_dc_r - self.dc_filter_r;
            self.dc_filter_r = new_dc_r;

            let mut samples = frame.iter_mut().collect::<Vec<_>>();
            *samples[0] = (out_l * gain).clamp(-1.0, 1.0);
            *samples[1] = (out_r * gain).clamp(-1.0, 1.0);
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for ArtcodeBass {
    const CLAP_ID: &'static str = "com.artcode.bass";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Fat bass synthesizer");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Instrument, ClapFeature::Synthesizer, ClapFeature::Mono];
}

impl Vst3Plugin for ArtcodeBass {
    const VST3_CLASS_ID: [u8; 16] = *b"artcodeBASS_vst3";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_clap!(ArtcodeBass);
nih_export_vst3!(ArtcodeBass);
