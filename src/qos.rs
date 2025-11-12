use chrono::Duration;

use zenoh::key_expr::format::kedefine;

kedefine!(
    pub(crate) ke_liveliness: "@ros2_lv/${domain:*}/${zid:*}/${node:*}/${entity:*}/${entity_kind:*}/${enclave:*}/${namespace:*}/${node_name:*}/${topic:*}/${rostype:*}/${hash:*}/${qos:*}",
);

#[derive(Debug)]
pub enum History {
    KeepLast,
    KeepAll,
}

#[derive(Debug)]
pub enum Reliability {
    Reliable,
    BestEffort,
}

#[derive(Debug)]
pub enum Durability {
    Volatile,
    TransientLocal,
}

#[derive(Debug)]
pub enum Liveliness {
    Automatic,
    Manual,
}

#[derive(Debug)]
pub struct RosQos {
    pub history: History,
    pub depth: Option<u32>,
    pub reliability: Reliability,
    pub durability: Durability,
    pub deadline: Duration,
    pub lifespan: Duration,
    pub liveliness: Liveliness,
    pub liveliness_lease_duration: Duration,
    pub avoid_ros_namespace_conventions: bool,
}
