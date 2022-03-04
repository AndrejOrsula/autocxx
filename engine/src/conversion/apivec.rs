// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::api::{AnalysisPhase, Api};

/// Newtype wrapper for a list of APIs, which enforced the invariant
/// that each API has a unique name.
pub(crate) struct ApiVec<P: AnalysisPhase> {
    apis: Vec<Api<P>>,
}

impl<P: AnalysisPhase> ApiVec<P> {
    pub(crate) fn push(&mut self, api: Api<P>) {
        self.apis.push(api)
    }

    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn append(&mut self, more: &mut ApiVec<P>) {
        self.extend(more.apis.drain(..))
    }

    pub(crate) fn extend(&mut self, it: impl Iterator<Item = Api<P>>) {
        // Could be optimized in future
        for api in it {
            self.push(api)
        }
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Api<P>> {
        self.apis.iter()
    }

    pub(crate) fn into_iter(self) -> impl Iterator<Item = Api<P>> {
        self.apis.into_iter()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.apis.is_empty()
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Api<P>) -> bool,
    {
        self.apis.retain(f);
    }
}

impl<P: AnalysisPhase> Default for ApiVec<P> {
    fn default() -> Self {
        Self {
            apis: Default::default(),
        }
    }
}

impl<P: AnalysisPhase> FromIterator<Api<P>> for ApiVec<P> {
    fn from_iter<I: IntoIterator<Item = Api<P>>>(iter: I) -> Self {
        let mut this = ApiVec::new();
        for i in iter {
            // Could be optimized in future
            this.push(i);
        }
        this
    }
}
