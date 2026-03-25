use serde::{Deserialize, Serialize};

/// Persistent explain snapshot for one saved route.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedRouteExplain {
    pub direct_route_status: String,
    pub final_route_status: String,
    pub total_iterations: usize,
    pub final_collision: Option<SavedCollisionExplain>,
    pub direct_collision: Option<SavedCollisionExplain>,
    pub collision_explain: Option<SavedCollisionPenaltyExplain>,
    pub last_selected_detour: Option<SavedDetourExplain>,
    pub iterations: Vec<SavedIterationExplain>,
    pub final_path: Vec<SavedPointExplain>,
    pub quality: SavedQualityExplain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedPointExplain {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedCollisionExplain {
    pub obstacle_id: i64,
    pub obstacle_name: String,
    pub obstacle_x: f64,
    pub obstacle_y: f64,
    pub closest_distance: f64,
    pub required_clearance: f64,
    pub t: f64,
    pub closest_point_x: f64,
    pub closest_point_y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedCollisionPenaltyExplain {
    pub obstacle_id: i64,
    pub obstacle_name: String,
    pub obstacle_x: f64,
    pub obstacle_y: f64,
    pub obstacle_radius: f64,
    pub closest_distance: f64,
    pub required_clearance: f64,
    pub violated_by: f64,
    pub t: f64,
    pub closest_point_x: f64,
    pub closest_point_y: f64,
    pub proximity_penalty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedDetourExplain {
    pub waypoint_x: f64,
    pub waypoint_y: f64,
    pub side: String,
    pub offset_used: f64,
    pub score: f64,
    pub base_distance: f64,
    pub turn_penalty: f64,
    pub back_penalty: f64,
    pub proximity_penalty: f64,
    pub offset_penalty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedIterationExplain {
    pub iteration: usize,
    pub segment_index: usize,
    pub collision: SavedCollisionExplain,
    pub selected_candidate: Option<SavedDetourExplain>,
    pub candidates: Vec<SavedCandidateExplain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedCandidateExplain {
    pub side: String,
    pub offset_used: f64,
    pub is_valid: bool,
    pub score: f64,
    pub base_distance: f64,
    pub turn_penalty: f64,
    pub back_penalty: f64,
    pub proximity_penalty: f64,
    pub offset_penalty: f64,
    pub rejection_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedQualityExplain {
    pub waypoint_count: usize,
    pub detour_overhead_pc: f64,
    pub max_turn_penalty: f64,
    pub total_turn_penalty: f64,
    pub total_proximity_penalty: f64,
    pub max_offset_penalty: f64,
    pub total_offset_penalty: f64,
}
