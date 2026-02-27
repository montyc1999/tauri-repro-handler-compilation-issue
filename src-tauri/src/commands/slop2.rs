use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Just a big dump of LLM slop to demonstrate typechecking cost

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SlopStruct2 {
    id: u64,
    username: String,
    role_count: u32,
    config_version: u32,
    address: Option<Address>,
    status: AccountStatus,
    preferences: Preferences,
    subscription: SubscriptionTier,
    oauth_links: Vec<OAuthLink>,
    tls: TlsConfig,
    retry: RetryPolicy,
    geo: Option<GeoCoord>,
    sessions: Vec<Session>,
    api_keys: Vec<ApiKey>,
    last_login: Option<LoginEvent>,
}

impl SlopStruct2 {
    pub async fn new(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct GeoCoord {
    lat: f64,
    lon: f64,
    altitude_m: Option<f32>,
    accuracy_m: f32,
    source: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
    state_code: String,
    postal_code: String,
    country: String,
    geo: Option<GeoCoord>,
    validated: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]

enum AccountStatus {
    #[default]
    Active,
    Suspended {
        reason: String,
        until: Option<u64>,
    },
    PendingVerification {
        token: String,
    },
    Deactivated,
    LockedOut {
        attempts: u32,
        locked_at: u64,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct Preferences {
    theme: String,
    locale: String,
    timezone: String,
    date_format: String,
    notifications_enabled: bool,
    email_digest: bool,
    custom: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]

enum SubscriptionTier {
    #[default]
    Free,
    Pro {
        seats: u32,
        expires_at: u64,
    },
    Enterprise {
        contract_id: String,
        seats: u32,
        expires_at: u64,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct OAuthLink {
    provider: String,
    provider_user_id: String,
    linked_at: u64,
    scopes: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct TlsConfig {
    enabled: bool,
    verify_peer: bool,
    ca_cert_path: Option<String>,
    client_cert_path: Option<String>,
    min_protocol_version: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct RetryPolicy {
    max_attempts: u32,
    initial_backoff_ms: u64,
    max_backoff_ms: u64,
    backoff_multiplier: f64,
    jitter: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]

enum TrustLevel {
    #[default]
    Unknown,
    Trusted {
        since: u64,
        verified_by: String,
    },
    Suspicious {
        reason: String,
        flagged_at: u64,
        risk_score: f64,
    },
    Blocked {
        at: u64,
        by: String,
        reason: String,
        appeal_token: Option<String>,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct DeviceInfo {
    os: String,
    browser: String,
    fingerprint: String,
    location: Option<GeoCoord>,
    trust: TrustLevel,
    labels: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct Session {
    id: u64,
    device: DeviceInfo,
    scopes: Vec<String>,
    created_at: u64,
    expires_at: Option<u64>,
    metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct ApiKey {
    id: String,
    name: String,
    scopes: Vec<String>,
    created_at: u64,
    expires_at: Option<u64>,
    last_used: Option<u64>,
    restrictions: HashMap<String, Vec<String>>,
    labels: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]

enum DeviceKind {
    #[default]
    Unknown,
    Desktop {
        os: String,
        arch: String,
    },
    Mobile {
        platform: String,
        model: String,
    },
    Bot {
        name: String,
        verified: bool,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct LoginContext {
    device_kind: DeviceKind,
    user_agent: String,
    fingerprint: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]

enum LoginEvent {
    #[default]
    Unknown,
    Success {
        at: u64,
        context: LoginContext,
        ip: String,
    },
    Failure {
        at: u64,
        reason: String,
        context: LoginContext,
        attempt: u32,
    },
    PasswordReset {
        at: u64,
        via: String,
    },
    MfaChallenge {
        at: u64,
        method: String,
        succeeded: bool,
    },
}
