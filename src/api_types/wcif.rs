use std::ops::{Deref, DerefMut};

use super::Competition;

use crate::{Disabled, Enabled};

pub struct PublicWcif {
        inner: Competition<Disabled>,
}

impl Deref for PublicWcif {
        type Target = Competition<Disabled>;

        fn deref(&self) -> &Self::Target {
                &self.inner
        }
}

impl DerefMut for PublicWcif {
        fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
        }
}

pub struct PrivateWcif {
        inner: Competition<Enabled>,
}

impl Deref for PrivateWcif {
        type Target = Competition<Enabled>;

        fn deref(&self) -> &Self::Target {
                &self.inner
        }
}

impl DerefMut for PrivateWcif {
        fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
        }
}
