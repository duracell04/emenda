import type { Correction, ErrorKind, WorkflowState } from "../schemas";
import {
  ArrowIcon,
  CheckIcon,
  CloseIcon,
  RefreshIcon,
  SparkIcon,
  WarningIcon,
} from "./Icons";

type SuggestionPanelProps = {
  workflow: WorkflowState;
  hotkey: string;
  apiKeyConfigured: boolean;
  applyingIndex: number | null;
  dismissing: boolean;
  onApply: (index: number) => void;
  onCheck: () => void;
  onDismiss: () => void;
  onOpenSettings: () => void;
};

const CATEGORY_LABELS: Record<Correction["category"], string> = {
  spelling: "Spelling",
  grammar: "Grammar",
  punctuation: "Punctuation",
  style: "Style",
};

const CONFIDENCE_LABELS: Record<Correction["confidence"], string> = {
  high: "High confidence",
  medium: "Medium confidence",
  low: "Low confidence",
};

const ERROR_TITLES: Record<ErrorKind, string> = {
  configuration: "OpenRouter needs configuring",
  authentication: "The API key was not accepted",
  network: "OpenRouter could not be reached",
  inference: "The model could not finish this check",
  structuredOutput: "The model returned an invalid response",
  validation: "A suggestion could not be validated",
  staleRevision: "This result is no longer current",
  textCapture: "The selected text could not be captured",
  textReplacement: "The correction could not be applied",
  protectedSurface: "This application blocks text access",
};

function sourceName(workflow: WorkflowState): string | null {
  if (!("sourceApplication" in workflow) || !workflow.sourceApplication) {
    return null;
  }

  return typeof workflow.sourceApplication === "string"
    ? workflow.sourceApplication
    : workflow.sourceApplication.applicationName;
}

function CorrectionCard({
  correction,
  index,
  applying,
  actionsDisabled,
  onApply,
}: {
  correction: Correction;
  index: number;
  applying: boolean;
  actionsDisabled: boolean;
  onApply: (index: number) => void;
}) {
  return (
    <article className="correction-card">
      <div className="correction-meta">
        <span className={`category-tag category-${correction.category}`}>
          {CATEGORY_LABELS[correction.category]}
        </span>
        <span className={`confidence confidence-${correction.confidence}`}>
          <span className="confidence-dot" />
          {CONFIDENCE_LABELS[correction.confidence]}
        </span>
      </div>

      <div className="correction-change" aria-label={`${correction.original} to ${correction.replacement}`}>
        <del>{correction.original || "Remove"}</del>
        <ArrowIcon className="change-arrow" />
        <ins>{correction.replacement || "Remove"}</ins>
      </div>

      {correction.explanation ? (
        <p className="correction-explanation">{correction.explanation}</p>
      ) : null}

      <button
        className="button button-primary correction-apply"
        type="button"
        disabled={actionsDisabled}
        onClick={() => onApply(index)}
      >
        {applying ? <span className="spinner spinner-small" /> : <CheckIcon />}
        {applying ? "Applying…" : "Apply correction"}
      </button>
    </article>
  );
}

function EmptyState({
  hotkey,
  apiKeyConfigured,
  onCheck,
  onOpenSettings,
}: Pick<SuggestionPanelProps, "hotkey" | "apiKeyConfigured" | "onCheck" | "onOpenSettings">) {
  return (
    <section className="state-card hero-state" aria-labelledby="ready-title">
      <div className="state-symbol spark-symbol">
        <SparkIcon />
      </div>
      <p className="eyebrow">Ready to refine</p>
      <h1 id="ready-title">Keep your voice.<br />Polish the details.</h1>
      <p className="state-description">
        Select a passage in any editable app, then use the shortcut to check it with Emenda.
      </p>

      {apiKeyConfigured ? (
        <button className="button button-primary button-wide" type="button" onClick={onCheck}>
          <SparkIcon />
          Check current selection
        </button>
      ) : (
        <button className="button button-primary button-wide" type="button" onClick={onOpenSettings}>
          Configure OpenRouter
          <ArrowIcon />
        </button>
      )}

      <div className="shortcut-hint">
        <span>{apiKeyConfigured ? "Or use the global shortcut" : "Your shortcut after setup"}</span>
        <kbd>{hotkey}</kbd>
      </div>
    </section>
  );
}

export function SuggestionPanel({
  workflow,
  hotkey,
  apiKeyConfigured,
  applyingIndex,
  dismissing,
  onApply,
  onCheck,
  onDismiss,
  onOpenSettings,
}: SuggestionPanelProps) {
  const source = sourceName(workflow);

  if (workflow.status === "idle") {
    return (
      <EmptyState
        hotkey={hotkey}
        apiKeyConfigured={apiKeyConfigured}
        onCheck={onCheck}
        onOpenSettings={onOpenSettings}
      />
    );
  }

  if (workflow.status === "checking") {
    return (
      <section className="state-card checking-state" aria-live="polite" aria-busy="true">
        <div className="checking-orbit" aria-hidden="true">
          <span />
        </div>
        <p className="eyebrow">Checking selection</p>
        <h1>Reading with restraint…</h1>
        <p className="state-description">
          Emenda is looking for the smallest useful corrections while preserving your wording.
        </p>
        <div className="checking-lines" aria-hidden="true">
          <span />
          <span />
          <span />
        </div>
      </section>
    );
  }

  if (workflow.status === "clean") {
    const applied = workflow.applied === true;
    return (
      <section className="state-card result-state" aria-live="polite">
        <div className="state-symbol success-symbol">
          <CheckIcon />
        </div>
        <p className="eyebrow">{applied ? "Correction applied" : "Check complete"}</p>
        <h1>{applied ? "Back to your writing." : "Text looks good."}</h1>
        <p className="state-description">
          {applied
            ? "The selected passage was updated in its source application as one undoable replacement."
            : "No useful corrections were found. Your original passage has been left untouched."}
        </p>
        {source ? <p className="source-label">Checked in {source}</p> : null}
        <button className="button button-secondary button-wide" type="button" onClick={onCheck}>
          <RefreshIcon />
          Check another selection
        </button>
      </section>
    );
  }

  if (workflow.status === "error") {
    const settingsAction =
      workflow.error.kind === "configuration" || workflow.error.kind === "authentication";

    return (
      <section className="state-card result-state error-state" role="alert">
        <div className="state-symbol warning-symbol">
          <WarningIcon />
        </div>
        <p className="eyebrow">Unable to continue</p>
        <h1>{ERROR_TITLES[workflow.error.kind]}</h1>
        <p className="state-description">{workflow.error.message}</p>
        <button
          className="button button-primary button-wide"
          type="button"
          onClick={settingsAction ? onOpenSettings : onCheck}
        >
          {settingsAction ? "Open settings" : "Try current selection again"}
          {settingsAction ? <ArrowIcon /> : <RefreshIcon />}
        </button>
      </section>
    );
  }

  const actionInProgress = applyingIndex !== null || dismissing;
  const acceptedCount = workflow.acceptedCount ?? 0;

  return (
    <section className="suggestions-view" aria-labelledby="suggestions-title">
      <div className="suggestions-heading">
        <div>
          <p className="eyebrow">Review changes</p>
          <h1 id="suggestions-title">
            {workflow.corrections.length} {workflow.corrections.length === 1 ? "correction" : "corrections"}
          </h1>
        </div>
        {source ? <span className="source-chip">{source}</span> : null}
      </div>

      <p className="suggestions-intro">
        Apply on a card accepts that change locally. When you finish, Emenda updates the source text
        once so its native undo stays useful.
      </p>

      {acceptedCount > 0 ? (
        <div className="accepted-banner" aria-live="polite">
          <CheckIcon />
          {acceptedCount} {acceptedCount === 1 ? "change accepted" : "changes accepted"}
        </div>
      ) : null}

      <div className="correction-list">
        {workflow.corrections.map((correction, index) => (
          <CorrectionCard
            key={`${correction.start}-${correction.end}-${correction.original}-${correction.replacement}`}
            correction={correction}
            index={index}
            applying={applyingIndex === index}
            actionsDisabled={actionInProgress}
            onApply={onApply}
          />
        ))}
      </div>

      <button
        className={`button ${acceptedCount > 0 ? "button-primary" : "button-quiet"} dismiss-button`}
        type="button"
        disabled={actionInProgress}
        onClick={onDismiss}
      >
        {dismissing ? (
          <span className="spinner spinner-small" />
        ) : acceptedCount > 0 ? (
          <CheckIcon />
        ) : (
          <CloseIcon />
        )}
        {dismissing
          ? acceptedCount > 0
            ? "Applying to source…"
            : "Dismissing…"
          : acceptedCount > 0
            ? `Apply ${acceptedCount} accepted ${acceptedCount === 1 ? "change" : "changes"}`
            : "Dismiss all"}
      </button>
    </section>
  );
}
