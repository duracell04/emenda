import { useCallback, useEffect, useRef, useState } from "react";

import { ChevronIcon, GearIcon } from "./components/Icons";
import { SettingsPanel } from "./components/SettingsPanel";
import { SuggestionPanel } from "./components/SuggestionPanel";
import {
  workflowErrorSchema,
  type ErrorKind,
  type Model,
  type PublicSettings,
  type SaveSettingsInput,
  type WorkflowState,
} from "./schemas";
import {
  applyCorrection,
  checkCurrentSelection,
  dismissSuggestions,
  errorMessage,
  getSettings,
  listModels,
  saveSettings,
  subscribeToWorkflow,
  testOpenRouter,
} from "./tauri";

const FALLBACK_SETTINGS: PublicSettings = {
  apiKeyConfigured: false,
  modelId: "openrouter/free",
  languageMode: "auto",
  hotkey: "Ctrl+Alt+E",
};

const DEFAULT_MODEL: Model = {
  id: "openrouter/free",
  name: "Free automatically",
  description: "OpenRouter selects a currently available free model for each request.",
};

function toErrorState(error: unknown, fallbackKind: ErrorKind): WorkflowState {
  const parsed = workflowErrorSchema.safeParse(error);
  if (parsed.success) {
    return { status: "error", error: parsed.data };
  }

  return {
    status: "error",
    error: {
      kind: fallbackKind,
      message: errorMessage(error),
    },
  };
}

export function App() {
  const [view, setView] = useState<"review" | "settings">("review");
  const [settings, setSettings] = useState<PublicSettings>(FALLBACK_SETTINGS);
  const [settingsLoaded, setSettingsLoaded] = useState(false);
  const [workflow, setWorkflow] = useState<WorkflowState>({ status: "idle" });
  const [models, setModels] = useState<Model[]>([DEFAULT_MODEL]);
  const [modelsLoaded, setModelsLoaded] = useState(false);
  const [modelsLoading, setModelsLoading] = useState(false);
  const [modelsError, setModelsError] = useState<string | null>(null);
  const [applyingIndex, setApplyingIndex] = useState<number | null>(null);
  const [dismissing, setDismissing] = useState(false);
  const modelRequest = useRef<Promise<void> | null>(null);

  const loadModels = useCallback(() => {
    if (modelRequest.current) {
      return modelRequest.current;
    }

    setModelsLoading(true);
    setModelsError(null);

    const pending = listModels()
      .then((catalogue) => {
        setModels(
          catalogue.some((model) => model.id === DEFAULT_MODEL.id)
            ? catalogue
            : [DEFAULT_MODEL, ...catalogue],
        );
      })
      .catch((error: unknown) => {
        setModelsError(errorMessage(error));
      });

    modelRequest.current = pending;
    void pending.finally(() => {
      if (modelRequest.current === pending) {
        modelRequest.current = null;
      }
      setModelsLoaded(true);
      setModelsLoading(false);
    });

    return pending;
  }, []);

  useEffect(() => {
    let disposed = false;
    let unsubscribe: (() => void) | undefined;

    void getSettings()
      .then((loadedSettings) => {
        if (disposed) return;
        setSettings(loadedSettings);
        setSettingsLoaded(true);
        if (!loadedSettings.apiKeyConfigured) {
          setView("settings");
        }
      })
      .catch((error: unknown) => {
        if (disposed) return;
        setSettingsLoaded(true);
        setWorkflow(toErrorState(error, "configuration"));
      });

    void subscribeToWorkflow(
      (nextState) => {
        if (!disposed) setWorkflow(nextState);
      },
      (error) => {
        if (!disposed) setWorkflow(toErrorState(error, "validation"));
      },
    )
      .then((stopListening) => {
        if (disposed) {
          stopListening();
        } else {
          unsubscribe = stopListening;
        }
      })
      .catch((error: unknown) => {
        if (!disposed) setWorkflow(toErrorState(error, "configuration"));
      });

    return () => {
      disposed = true;
      unsubscribe?.();
    };
  }, []);

  useEffect(() => {
    if (view === "settings" && settingsLoaded && !modelsLoaded && !modelsLoading) {
      void loadModels();
    }
  }, [loadModels, modelsLoaded, modelsLoading, settingsLoaded, view]);

  async function handleCheck() {
    setView("review");
    setWorkflow({ status: "checking" });

    try {
      setWorkflow(await checkCurrentSelection());
    } catch (error) {
      setWorkflow(toErrorState(error, "inference"));
    }
  }

  async function handleApply(index: number) {
    setApplyingIndex(index);

    try {
      setWorkflow(await applyCorrection(index));
    } catch (error) {
      setWorkflow(toErrorState(error, "textReplacement"));
    } finally {
      setApplyingIndex(null);
    }
  }

  async function handleDismiss() {
    setDismissing(true);

    try {
      setWorkflow(await dismissSuggestions());
    } catch (error) {
      setWorkflow(toErrorState(error, "validation"));
    } finally {
      setDismissing(false);
    }
  }

  async function handleSave(nextSettings: SaveSettingsInput) {
    const saved = await saveSettings(nextSettings);
    setSettings(saved);
    return saved;
  }

  return (
    <main className="app-shell">
      <header className="app-header">
        <button
          className="brand-button"
          type="button"
          aria-label="Open suggestion review"
          onClick={() => setView("review")}
        >
          <span className="brand-mark">E</span>
          <span className="brand-copy">
            <strong>Emenda</strong>
            <small>Preserve your Duktus</small>
          </span>
        </button>

        {view === "settings" ? (
          <button className="header-action back-action" type="button" onClick={() => setView("review")}>
            <ChevronIcon />
            Review
          </button>
        ) : (
          <button className="header-action" type="button" onClick={() => setView("settings")}>
            <GearIcon />
            Settings
          </button>
        )}
      </header>

      <div className="app-content">
        {view === "settings" ? (
          settingsLoaded ? (
            <SettingsPanel
              settings={settings}
              models={models}
              modelsLoading={modelsLoading}
              modelsError={modelsError}
              onReloadModels={() => void loadModels()}
              onSave={handleSave}
              onTest={testOpenRouter}
            />
          ) : (
            <section className="state-card checking-state" aria-live="polite" aria-busy="true">
              <span className="spinner" />
              <h1>Loading settings…</h1>
            </section>
          )
        ) : (
          <SuggestionPanel
            workflow={workflow}
            hotkey={settings.hotkey}
            apiKeyConfigured={settings.apiKeyConfigured}
            applyingIndex={applyingIndex}
            dismissing={dismissing}
            onApply={(index) => void handleApply(index)}
            onCheck={() => void handleCheck()}
            onDismiss={() => void handleDismiss()}
            onOpenSettings={() => setView("settings")}
          />
        )}
      </div>

      <footer className="app-footer">
        <span className="privacy-mark"><span /> Local core</span>
        <span>Text is sent only when you invoke Emenda.</span>
      </footer>
    </main>
  );
}
