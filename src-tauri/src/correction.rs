use crate::error::CoreError;
use serde::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CorrectionCategory {
    Spelling,
    Grammar,
    Punctuation,
    Style,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    High,
    Medium,
    Low,
}

/// A correction which has been checked against the immutable source text.
///
/// Its fields are private so deserialising external data cannot construct a
/// trusted correction. OpenRouter data first enters [`CorrectionCandidate`]
/// and must pass [`validate_candidates`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Correction {
    start: usize,
    end: usize,
    original: String,
    replacement: String,
    category: CorrectionCategory,
    confidence: Confidence,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation: Option<String>,
}

impl Correction {
    pub const fn start(&self) -> usize {
        self.start
    }

    pub const fn end(&self) -> usize {
        self.end
    }

    pub fn original(&self) -> &str {
        &self.original
    }

    pub fn replacement(&self) -> &str {
        &self.replacement
    }

    pub const fn category(&self) -> CorrectionCategory {
        self.category
    }

    pub const fn confidence(&self) -> Confidence {
        self.confidence
    }

    pub fn explanation(&self) -> Option<&str> {
        self.explanation.as_deref()
    }

    /// Apply this correction to a working text.
    ///
    /// This revalidates text identity. It also permits the same unique-match
    /// fallback used at inference time, which keeps subsequent corrections
    /// applicable after an earlier correction shifts their scalar positions.
    pub fn apply_to(&self, text: &str) -> Result<String, CoreError> {
        let candidate = CorrectionCandidate {
            start: self.start,
            end: self.end,
            original: self.original.clone(),
            replacement: self.replacement.clone(),
            category: self.category,
            confidence: self.confidence,
            explanation: self.explanation.clone(),
        };

        let correction = validate_candidate(text, candidate).map_err(|rejected| {
            CoreError::ValidationError(format!(
                "Correction is no longer applicable: {}",
                rejected.reason.description()
            ))
        })?;
        let byte_range = scalar_range_to_byte_range(text, correction.start..correction.end)
            .ok_or_else(|| {
                CoreError::ValidationError(
                    "Correction range is outside the current text".to_owned(),
                )
            })?;

        let mut corrected = String::with_capacity(
            text.len() - (byte_range.end - byte_range.start) + correction.replacement.len(),
        );
        corrected.push_str(&text[..byte_range.start]);
        corrected.push_str(&correction.replacement);
        corrected.push_str(&text[byte_range.end..]);
        Ok(corrected)
    }
}

/// Structurally typed, but not yet semantically trusted, provider output.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct CorrectionCandidate {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) original: String,
    pub(crate) replacement: String,
    pub(crate) category: CorrectionCategory,
    pub(crate) confidence: Confidence,
    pub(crate) explanation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum NonApplicableReason {
    EmptyOriginal,
    RangeOutOfBounds {
        start: usize,
        end: usize,
        text_length: usize,
    },
    OriginalNotFound,
    AmbiguousOriginal {
        occurrences: usize,
    },
    OverlapsCorrection {
        other_start: usize,
        other_end: usize,
    },
    NoChange,
}

impl NonApplicableReason {
    pub fn description(&self) -> String {
        match self {
            Self::EmptyOriginal => "the original text is empty".to_owned(),
            Self::RangeOutOfBounds {
                start,
                end,
                text_length,
            } => format!("scalar range {start}..{end} is outside text of length {text_length}"),
            Self::OriginalNotFound => "the original text does not occur in the source".to_owned(),
            Self::AmbiguousOriginal { occurrences } => {
                format!("the original text occurs {occurrences} times")
            }
            Self::OverlapsCorrection {
                other_start,
                other_end,
            } => format!("it overlaps correction {other_start}..{other_end}"),
            Self::NoChange => "the replacement does not change the source".to_owned(),
        }
    }
}

/// Diagnostic information for an individual provider correction that must not
/// enter applicable suggestion state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NonApplicableCorrection {
    start: usize,
    end: usize,
    original: String,
    replacement: String,
    reason: NonApplicableReason,
}

impl NonApplicableCorrection {
    pub const fn reason(&self) -> &NonApplicableReason {
        &self.reason
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CorrectionValidationReport {
    corrections: Vec<Correction>,
    non_applicable: Vec<NonApplicableCorrection>,
}

impl CorrectionValidationReport {
    pub fn corrections(&self) -> &[Correction] {
        &self.corrections
    }

    pub fn into_corrections(self) -> Vec<Correction> {
        self.corrections
    }

    pub fn non_applicable(&self) -> &[NonApplicableCorrection] {
        &self.non_applicable
    }

    pub fn into_parts(self) -> (Vec<Correction>, Vec<NonApplicableCorrection>) {
        (self.corrections, self.non_applicable)
    }
}

pub(crate) fn validate_candidates(
    text: &str,
    candidates: Vec<CorrectionCandidate>,
) -> CorrectionValidationReport {
    let mut report = CorrectionValidationReport::default();
    for candidate in candidates {
        match validate_candidate(text, candidate) {
            Ok(correction) => report.corrections.push(correction),
            Err(rejected) => report.non_applicable.push(rejected),
        }
    }
    report
        .corrections
        .sort_by_key(|item| (item.start, item.end));
    let mut non_overlapping: Vec<Correction> = Vec::with_capacity(report.corrections.len());
    for correction in report.corrections {
        if let Some(previous) = non_overlapping.last() {
            if correction.start < previous.end {
                let reason = NonApplicableReason::OverlapsCorrection {
                    other_start: previous.start,
                    other_end: previous.end,
                };
                report
                    .non_applicable
                    .push(reject_trusted(correction, reason));
                continue;
            }
        }
        non_overlapping.push(correction);
    }
    report.corrections = non_overlapping;
    report
}

/// Apply a validated, non-overlapping correction set in descending scalar
/// order so earlier ranges are not shifted by later replacements.
pub fn apply_corrections(text: &str, corrections: &[Correction]) -> Result<String, CoreError> {
    let candidates = corrections
        .iter()
        .map(|correction| CorrectionCandidate {
            start: correction.start,
            end: correction.end,
            original: correction.original.clone(),
            replacement: correction.replacement.clone(),
            category: correction.category,
            confidence: correction.confidence,
            explanation: correction.explanation.clone(),
        })
        .collect();
    let report = validate_candidates(text, candidates);
    if let Some(rejected) = report.non_applicable.first() {
        return Err(CoreError::ValidationError(format!(
            "Correction set is not applicable: {}",
            rejected.reason.description()
        )));
    }

    let mut corrected = text.to_owned();
    for correction in report.corrections.iter().rev() {
        let byte_range = scalar_range_to_byte_range(text, correction.start..correction.end)
            .ok_or_else(|| {
                CoreError::ValidationError(
                    "Correction range is outside the current text".to_owned(),
                )
            })?;
        corrected.replace_range(byte_range, &correction.replacement);
    }
    Ok(corrected)
}

fn validate_candidate(
    text: &str,
    candidate: CorrectionCandidate,
) -> Result<Correction, NonApplicableCorrection> {
    if candidate.original.is_empty() {
        return Err(reject(candidate, NonApplicableReason::EmptyOriginal));
    }
    if candidate.original == candidate.replacement {
        return Err(reject(candidate, NonApplicableReason::NoChange));
    }

    let text_length = text.chars().count();
    let proposed_range = candidate.start..candidate.end;
    let proposed_slice =
        scalar_range_to_byte_range(text, proposed_range.clone()).map(|range| &text[range]);

    if proposed_slice == Some(candidate.original.as_str()) {
        return Ok(trust(candidate, proposed_range));
    }

    let occurrences = scalar_occurrences(text, &candidate.original);
    match occurrences.as_slice() {
        [start] => {
            let end = start + candidate.original.chars().count();
            Ok(trust(candidate, *start..end))
        }
        [] if candidate.start > candidate.end || candidate.end > text_length => {
            let start = candidate.start;
            let end = candidate.end;
            Err(reject(
                candidate,
                NonApplicableReason::RangeOutOfBounds {
                    start,
                    end,
                    text_length,
                },
            ))
        }
        [] => Err(reject(candidate, NonApplicableReason::OriginalNotFound)),
        matches => Err(reject(
            candidate,
            NonApplicableReason::AmbiguousOriginal {
                occurrences: matches.len(),
            },
        )),
    }
}

fn trust(candidate: CorrectionCandidate, resolved_range: Range<usize>) -> Correction {
    let explanation = candidate
        .explanation
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());
    Correction {
        start: resolved_range.start,
        end: resolved_range.end,
        original: candidate.original,
        replacement: candidate.replacement,
        category: candidate.category,
        confidence: candidate.confidence,
        explanation,
    }
}

fn reject(candidate: CorrectionCandidate, reason: NonApplicableReason) -> NonApplicableCorrection {
    NonApplicableCorrection {
        start: candidate.start,
        end: candidate.end,
        original: candidate.original,
        replacement: candidate.replacement,
        reason,
    }
}

fn reject_trusted(correction: Correction, reason: NonApplicableReason) -> NonApplicableCorrection {
    NonApplicableCorrection {
        start: correction.start,
        end: correction.end,
        original: correction.original,
        replacement: correction.replacement,
        reason,
    }
}

/// Convert Unicode scalar-value positions to a UTF-8 byte range.
pub fn scalar_range_to_byte_range(text: &str, range: Range<usize>) -> Option<Range<usize>> {
    if range.start > range.end {
        return None;
    }

    let scalar_length = text.chars().count();
    if range.end > scalar_length {
        return None;
    }

    let byte_at = |scalar_index: usize| {
        if scalar_index == scalar_length {
            Some(text.len())
        } else {
            text.char_indices()
                .nth(scalar_index)
                .map(|(byte_index, _)| byte_index)
        }
    };

    Some(byte_at(range.start)?..byte_at(range.end)?)
}

fn scalar_occurrences(text: &str, needle: &str) -> Vec<usize> {
    let text: Vec<char> = text.chars().collect();
    let needle: Vec<char> = needle.chars().collect();
    if needle.is_empty() || needle.len() > text.len() {
        return Vec::new();
    }

    text.windows(needle.len())
        .enumerate()
        .filter_map(|(index, window)| (window == needle.as_slice()).then_some(index))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(
        start: usize,
        end: usize,
        original: &str,
        replacement: &str,
    ) -> CorrectionCandidate {
        CorrectionCandidate {
            start,
            end,
            original: original.to_owned(),
            replacement: replacement.to_owned(),
            category: CorrectionCategory::Spelling,
            confidence: Confidence::High,
            explanation: Some("  Typo.  ".to_owned()),
        }
    }

    #[test]
    fn validates_a_correction_at_its_scalar_range() {
        let report = validate_candidates(
            "I liek this sentence.",
            vec![candidate(2, 6, "liek", "like")],
        );

        assert!(report.non_applicable().is_empty());
        let correction = &report.corrections()[0];
        assert_eq!(correction.original(), "liek");
        assert_eq!(correction.replacement(), "like");
        assert_eq!(correction.explanation(), Some("Typo."));
    }

    #[test]
    fn ranges_count_unicode_scalar_values_not_utf8_bytes() {
        let report = validate_candidates("მე ვწერ ტექსტს", vec![candidate(3, 7, "ვწერ", "ვწერდი")]);

        let correction = &report.corrections()[0];
        assert_eq!(correction.start(), 3);
        assert_eq!(correction.end(), 7);
        assert_eq!(
            correction.apply_to("მე ვწერ ტექსტს").unwrap(),
            "მე ვწერდი ტექსტს"
        );
    }

    #[test]
    fn resolves_a_wrong_range_when_original_is_unique() {
        let report = validate_candidates("The quik fox.", vec![candidate(0, 3, "quik", "quick")]);

        let correction = &report.corrections()[0];
        assert_eq!((correction.start(), correction.end()), (4, 8));
    }

    #[test]
    fn ambiguous_fallback_is_typed_non_applicable() {
        let report = validate_candidates("very very clear", vec![candidate(0, 4, "very", "quite")]);

        // The supplied range is exact, so ambiguity is irrelevant.
        assert_eq!(report.corrections().len(), 1);

        let report =
            validate_candidates("very very clear", vec![candidate(5, 10, "very", "quite")]);
        assert!(matches!(
            report.non_applicable()[0].reason(),
            NonApplicableReason::AmbiguousOriginal { occurrences: 2 }
        ));
    }

    #[test]
    fn applying_after_an_earlier_edit_relocates_a_unique_original() {
        let report = validate_candidates(
            "A quik brownn fox",
            vec![
                candidate(2, 6, "quik", "surprisingly quick"),
                candidate(7, 13, "brownn", "brown"),
            ],
        );
        let first = report.corrections()[0]
            .apply_to("A quik brownn fox")
            .unwrap();
        let second = report.corrections()[1].apply_to(&first).unwrap();

        assert_eq!(second, "A surprisingly quick brown fox");
    }

    #[test]
    fn overlapping_and_duplicate_corrections_are_not_admitted() {
        let report = validate_candidates(
            "abcdef",
            vec![
                candidate(1, 4, "bcd", "B"),
                candidate(3, 5, "de", "D"),
                candidate(1, 4, "bcd", "other"),
            ],
        );

        assert_eq!(report.corrections().len(), 1);
        assert_eq!(report.non_applicable().len(), 2);
        assert!(report.non_applicable().iter().all(|item| matches!(
            item.reason(),
            NonApplicableReason::OverlapsCorrection { .. }
        )));
    }

    #[test]
    fn applies_a_set_from_the_end_for_stable_scalar_ranges() {
        let report = validate_candidates(
            "მე ვწერ bad ტექსტს",
            vec![
                candidate(3, 7, "ვწერ", "დავწერე"),
                candidate(8, 11, "bad", "good"),
            ],
        );

        let corrected = apply_corrections("მე ვწერ bad ტექსტს", report.corrections()).unwrap();
        assert_eq!(corrected, "მე დავწერე good ტექსტს");
    }

    #[test]
    fn malformed_enum_values_cannot_deserialize() {
        let malformed = r#"{
            "start": 2,
            "end": 6,
            "original": "liek",
            "replacement": "like",
            "category": "rewrite",
            "confidence": "certain",
            "explanation": null
        }"#;

        assert!(serde_json::from_str::<CorrectionCandidate>(malformed).is_err());
    }
}
