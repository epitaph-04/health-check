#[cfg(feature = "ssr")]
pub mod broadcast_actor {
    use crate::types::{HealthCheckStatus, ServiceType};
    use actix::prelude::*;
    use log::info;
    use serde::{Deserialize, Serialize};
    use tokio::sync::broadcast;

    #[derive(Message, Serialize, Deserialize, Clone)]
    #[rtype(result = "()")]
    pub struct HealthCheckInfo {
        pub name: String,
        pub service_type: ServiceType,
        pub url: String,
        pub interval_seconds: u64,
        pub latest_status: HealthCheckStatus,
    }

    pub struct BroadcastActor {
        sender: broadcast::Sender<HealthCheckInfo>
    }
    
    impl BroadcastActor {
        pub fn new(sender: broadcast::Sender<HealthCheckInfo>) -> Self {
            BroadcastActor { sender }
        }
    }

    impl Actor for BroadcastActor {
        type Context = Context<Self>;

        fn started(&mut self, _ctx: &mut Self::Context) {
            info!("Broadcast actor started");
        }

        fn stopped(&mut self, _ctx: &mut Self::Context) {
            info!("Broadcast actor stopped");
        }
    }

    impl Handler<HealthCheckInfo> for BroadcastActor {
        type Result = ();

        fn handle(&mut self, msg: HealthCheckInfo, _ctx: &mut Context<Self>) -> Self::Result {
            self.sender.send(msg).ok();
        }
    }
}
