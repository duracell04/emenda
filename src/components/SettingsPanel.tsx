import { useDeferredValue, useMemo, useState } from "react";

import type {
  LanguageMode,
  Model,
  PublicSettings,
  SaveSettingsInput,
} from "../schemas";
import { errorMessage } from "../tauri";
import { CheckIcon, EyeIcon, LockIcon, RefreshIcon } from "./Icons";

type SettingsPanelProps = {
  settings: PublicSettings;
  models: Model[];
  modelsLoading: boolean;
  modelsError: string | null;
  onReloadModels: () => void;
  onSave: (settings: SaveSettingsInput) => Promise<PublicSettings>;
  onTest: () => Promise<void>;
};

const LANGUAGE_OPTIONS: ReadonlyArray<{
  value: LanguageMode;
  label: string;
  hint: string;
}> = [
  { value: "auto", label: "Automatic", hint: "Detect from each passage" },
  { value: "de-CH", label: "Swiss Standard German", hint: "de-CH" },
  { value: "en-GB", label: "British English", hint: "en-GB" },
  { value: "en-US", label: "American English", hint: "en-US" },
  { value: "fr-FR", label: "French", hint: "fr-FR" },
  { value: "ka-GE", label: "Georgian", hint: "ka-GE" },
  { value: "ru-RU", label: "Russian", hint: "ru-RU" },
];

type Feedback = { tone: "success" | "error"; message: string } | null;

export function SettingsPanel({
  settings,
  models,
  modelsLoading,
  modelsError,
  onReloadModels,
  onSave,
  onTest,
}: SettingsPanelProps) {
  const [apiKey, setApiKey] = useState("");
  const [showApiKey, setShowApiKey] = useState(false);
  const [modelId, setModelId] = useState(settings.modelId);
  const [languageMode, setLanguageMode] = useState<LanguageMode>(settings.languageMode);
  const [modelQuery, setModelQuery] = useState("");
  const [pendingAction, setPendingAction] = useState<"save" | "test" | "clear" | null>(null);
  const [feedback, setFeedback] = useState<Feedback>(null);
  const deferredQuery = useDeferredValue(modelQuery);

  const selectableModels = useMemo(
    () =>
      models.some((model) => model.id === modelId)
        ? models
        : [
            {
              id: modelId,
              name: "Current selection",
              description: "This saved model is not present in the latest catalogue.",
            },
            ...models,
          ],
    [modelId, models],
  );

  const filteredModels = useMemo(() => {
    const query = deferredQuery.trim().toLocaleLowerCase();
    if (!query) {
      return selectableModels;
    }

    return selectableModels.filter((model) => {
      const haystack = `${model.name} ${model.id} ${model.description ?? ""}`.toLocaleLowerCase();
      return haystack.includes(query);
    });
  }, [deferredQuery, selectableModels]);

  const visibleModels = filteredModels.slice(0, 80);
  const hasDraftKey = apiKey.trim().length > 0;
  const hasChanges =
    hasDraftKey || modelId !== settings.modelId || languageMode !== settings.languageMode;
  const busy = pendingAction !== null;

  async function handleSave() {
    setPendingAction("save");
    setFeedback(null);

    try {
      await onSave({
        ...(hasDraftKey ? { apiKey: apiKey.trim() } : {}),
        modelId,
        languageMode,
      });
      setApiKey("");
      setFeedback({ tone: "success", message: "Settings saved locally." });
    } catch (error) {
      setFeedback({ tone: "error", message: errorMessage(error) });
    } finally {
      setPendingAction(null);
    }
  }

  async function handleTest() {
    setPendingAction("test");
    setFeedback(null);

    try {
      await onTest();
      setFeedback({ tone: "success", message: "OpenRouter connection is working." });
    } catch (error) {
      setFeedback({ tone: "error", message: errorMessage(error) });
    } finally {
      setPendingAction(null);
    }
  }

  async function handleClearKey() {
    setPendingAction("clear");
    setFeedback(null);

    try {
      await onSave({ apiKey: null, modelId, languageMode });
      setApiKey("");
      setFeedback({ tone: "success", message: "Saved API key removed." });
    } catch (error) {
      setFeedback({ tone: "error", message: errorMessage(error) });
    } finally {
      setPendingAction(null);
    }
  }

  return (
    <section className="settings-view" aria-labelledby="settings-title">
      <div className="settings-heading">
        <p className="eyebrow">Preferences</p>
        <h1 id="settings-title">Settings</h1>
        <p>Configure the model once, then keep Emenda out of your way.</p>
      </div>

      <div className="settings-section">
        <div className="section-title-row">
          <div>
            <h2>OpenRouter API key</h2>
            <p>Stored by the local Rust core, never in the web interface.</p>
          </div>
          <span className={`connection-pill ${settings.apiKeyConfigured ? "is-connected" : ""}`}>
            <span />
            {settings.apiKeyConfigured ? "Configured" : "Not configured"}
          </span>
        </div>

        <label className="field-label" htmlFor="api-key">API key</label>
        <div className="secure-input">
          <LockIcon />
          <input
            id="api-key"
            type={showApiKey ? "text" : "password"}
            value={apiKey}
            placeholder={settings.apiKeyConfigured ? "Saved securely — enter to replace" : "sk-or-v1-…"}
            autoComplete="off"
            spellCheck={false}
            onChange={(event) => {
              setApiKey(event.target.value);
              setFeedback(null);
            }}
          />
          <button
            className="icon-button reveal-button"
            type="button"
            aria-label={showApiKey ? "Hide API key" : "Show API key"}
            aria-pressed={showApiKey}
            onClick={() => setShowApiKey((visible) => !visible)}
          >
            <EyeIcon crossed={showApiKey} />
          </button>
        </div>

        <div className="inline-actions">
          <button
            className="button button-secondary"
            type="button"
            disabled={busy || !settings.apiKeyConfigured || hasDraftKey}
            title={hasDraftKey ? "Save the new key before testing it" : undefined}
            onClick={() => void handleTest()}
          >
            {pendingAction === "test" ? <span className="spinner spinner-small" /> : <RefreshIcon />}
            {pendingAction === "test" ? "Testing…" : "Test connection"}
          </button>
          {settings.apiKeyConfigured ? (
            <button
              className="button button-quiet danger-button"
              type="button"
              disabled={busy}
              onClick={() => void handleClearKey()}
            >
              {pendingAction === "clear" ? "Removing…" : "Remove key"}
            </button>
          ) : null}
        </div>
      </div>

      <div className="settings-section">
        <div className="section-title-row">
          <div>
            <h2>AI model</h2>
            <p>Free automatically is the reliable default.</p>
          </div>
        </div>

        <label className="field-label" htmlFor="model-search">Search current models</label>
        <input
          className="text-input search-input"
          id="model-search"
          type="search"
          value={modelQuery}
          placeholder="Search by name or model ID"
          onChange={(event) => setModelQuery(event.target.value)}
        />

        <div className="model-list" role="radiogroup" aria-label="OpenRouter model">
          {modelsLoading ? (
            <div className="model-message" aria-live="polite">
              <span className="spinner" /> Loading current models…
            </div>
          ) : null}

          {!modelsLoading && modelsError ? (
            <div className="model-message model-error" role="alert">
              <span>{modelsError}</span>
              <button className="text-button" type="button" onClick={onReloadModels}>Try again</button>
            </div>
          ) : null}

          {!modelsLoading && !modelsError && visibleModels.length === 0 ? (
            <div className="model-message">No current models match “{modelQuery}”.</div>
          ) : null}

          {visibleModels.map((model) => {
            const selected = model.id === modelId;
            return (
              <button
                className={`model-option ${selected ? "is-selected" : ""}`}
                key={model.id}
                type="button"
                role="radio"
                aria-checked={selected}
                onClick={() => {
                  setModelId(model.id);
                  setFeedback(null);
                }}
              >
                <span className="radio-mark">{selected ? <CheckIcon /> : null}</span>
                <span className="model-copy">
                  <strong>{model.name}</strong>
                  <code>{model.id}</code>
                  {model.description ? <small>{model.description}</small> : null}
                </span>
              </button>
            );
          })}
        </div>
        {filteredModels.length > visibleModels.length ? (
          <p className="list-limit-note">
            Showing the first {visibleModels.length} of {filteredModels.length} matches. Refine the search to narrow the list.
          </p>
        ) : null}
      </div>

      <div className="settings-section settings-grid">
        <div>
          <h2>Language</h2>
          <p className="section-copy">Automatic keeps the author’s detected language variety.</p>
          <label className="field-label" htmlFor="language-mode">Correction profile</label>
          <select
            className="select-input"
            id="language-mode"
            value={languageMode}
            onChange={(event) => {
              setLanguageMode(event.target.value as LanguageMode);
              setFeedback(null);
            }}
          >
            {LANGUAGE_OPTIONS.map((option) => (
              <option key={option.value} value={option.value}>
                {option.label} · {option.hint}
              </option>
            ))}
          </select>
        </div>

        <div>
          <h2>Global shortcut</h2>
          <p className="section-copy">Select text in another application, then press:</p>
          <kbd className="hotkey-display">{settings.hotkey}</kbd>
        </div>
      </div>

      <div className="settings-save-row">
        <div className="feedback-slot" aria-live="polite">
          {feedback ? (
            <span className={`feedback-message feedback-${feedback.tone}`}>
              {feedback.tone === "success" ? <CheckIcon /> : null}
              {feedback.message}
            </span>
          ) : (
            <span>Changes remain local to this device.</span>
          )}
        </div>
        <button
          className="button button-primary"
          type="button"
          disabled={busy || !hasChanges}
          onClick={() => void handleSave()}
        >
          {pendingAction === "save" ? <span className="spinner spinner-small" /> : <CheckIcon />}
          {pendingAction === "save" ? "Saving…" : "Save settings"}
        </button>
      </div>
    </section>
  );
}
