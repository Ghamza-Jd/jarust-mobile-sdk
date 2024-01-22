use crate::connection::RawJaConnection;
use crate::session::RawJaSession;
use std::fmt::Debug;
use std::sync::Arc;

pub trait RawJaConnectionCallback: Send + Sync + Debug {
    fn on_connection_success(&self, connection: Arc<RawJaConnection>);
    fn on_connection_failure(&self);

    fn on_session_creation_success(&self, session: Arc<RawJaSession>);
    fn on_session_creation_failure(&self);
}