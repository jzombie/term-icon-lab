//! Verdict output schema and Pass1×Pass2 merge.

use crate::schema::Pass1Status;
use serde::{Deserialize, Serialize};

/// Per-candidate combined verdict.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Verdict {
    pub id: String,
    pub codepoint: u32,
    /// Pass-1 status string (`pass | fail | inconclusive`).
    pub pass1: String,
    /// Pass-2 visibility (false = blank or tofu box).
    pub visible: bool,
    /// Pass-2 spatial isolation (true = no bleed into sentinel B).
    pub no_bleed: bool,
    /// AND gate at candidate level: `pass && visible && no_bleed`.
    pub overall: String,
}

/// Full platform verdicts (`verdicts.json`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerdictReport {
    pub schema_version: u32,
    pub platform: String,
    pub host: String,
    pub pass1_support: bool,
    /// Set when the capture could not be analyzed at all (band mismatch,
    /// calibration failure, implausible capture). All candidates then fail.
    pub structural_fail: Option<String>,
    pub results: Vec<Verdict>,
}

impl VerdictReport {
    /// Ids whose candidates passed everything.
    #[must_use]
    pub fn passing_ids(&self) -> Vec<&str> {
        if self.structural_fail.is_some() || !self.pass1_support {
            return Vec::new();
        }
        self.results
            .iter()
            .filter(|v| v.overall == "pass")
            .map(|v| v.id.as_str())
            .collect()
    }
}

#[must_use]
pub fn merge(
    id: &str,
    codepoint: u32,
    pass1: Pass1Status,
    visible: bool,
    no_bleed: bool,
) -> Verdict {
    let p1 = match pass1 {
        Pass1Status::Pass => "pass",
        Pass1Status::Fail => "fail",
        Pass1Status::Inconclusive => "inconclusive",
    };
    let overall = if p1 == "pass" && visible && no_bleed {
        "pass"
    } else {
        "fail"
    };
    Verdict {
        id: id.to_owned(),
        codepoint,
        pass1: p1.to_owned(),
        visible,
        no_bleed,
        overall: overall.to_owned(),
    }
}
