use super::devnull::{DevNull, DevNullWorker};
use super::{Feedback, FeedbackWorker};

#[derive(Clone)]
pub enum Client {
    DevNull(DevNull),
}

impl Feedback for Client {
    async fn get_feature_ptr_payload<
        T: serde::ser::Serialize + serde::de::DeserializeOwned + Send + std::fmt::Debug,
    >(
        &self,
        name: impl Into<String> + core::marker::Send + std::fmt::Debug,
    ) -> Option<T> {
        match self {
            Self::DevNull(d) => d.get_feature_ptr_payload(name).await,
        }
    }

    async fn set_planner(
        &mut self,
        planner: &crate::planner::BuiltinPlanner,
    ) -> Result<(), crate::planner::PlannerError> {
        match self {
            Self::DevNull(d) => d.set_planner(planner).await,
        }
    }

    async fn planning_failed(&mut self, error: &crate::error::NixInstallerError) {
        match self {
            Self::DevNull(d) => d.planning_failed(error).await,
        }
    }

    async fn planning_succeeded(&mut self) {
        match self {
            Self::DevNull(d) => d.planning_succeeded().await,
        }
    }

    async fn install_cancelled(&mut self) {
        match self {
            Self::DevNull(d) => d.install_cancelled().await,
        }
    }

    async fn install_failed(&mut self, error: &crate::error::NixInstallerError) {
        match self {
            Self::DevNull(d) => d.install_failed(error).await,
        }
    }

    async fn self_test_failed(&mut self, error: &crate::error::NixInstallerError) {
        match self {
            Self::DevNull(d) => d.self_test_failed(error).await,
        }
    }

    async fn install_succeeded(&mut self) {
        match self {
            Self::DevNull(d) => d.install_succeeded().await,
        }
    }

    async fn uninstall_cancelled(&mut self) {
        match self {
            Self::DevNull(d) => d.uninstall_cancelled().await,
        }
    }

    async fn uninstall_failed(&mut self, error: &crate::error::NixInstallerError) {
        match self {
            Self::DevNull(d) => d.uninstall_failed(error).await,
        }
    }

    async fn uninstall_succeeded(&mut self) {
        match self {
            Self::DevNull(d) => d.uninstall_succeeded().await,
        }
    }
}

pub enum Worker {
    DevNull(DevNullWorker),
}

#[derive(thiserror::Error, Debug)]
pub enum WorkerError {
    #[error("DevNull is infallible.")]
    DevNull(#[from] std::convert::Infallible),
}

impl FeedbackWorker for Worker {
    async fn submit(self) {
        match self {
            Self::DevNull(d) => d.submit().await,
        }
    }
}
