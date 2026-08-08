import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { z } from "zod";

import {
  modelSchema,
  publicSettingsSchema,
  saveSettingsInputSchema,
  workflowStateSchema,
  type Model,
  type PublicSettings,
  type SaveSettingsInput,
  type WorkflowState,
} from "./schemas";

class BoundaryValidationError extends Error {
  constructor(boundary: string, details: string) {
    super(`Invalid data from ${boundary}: ${details}`);
    this.name = "BoundaryValidationError";
  }
}

function parseBoundary<TSchema extends z.ZodType>(
  schema: TSchema,
  value: unknown,
  boundary: string,
): z.output<TSchema> {
  const result = schema.safeParse(value);

  if (!result.success) {
    throw new BoundaryValidationError(boundary, result.error.issues[0]?.message ?? "unknown shape");
  }

  return result.data;
}

export async function getSettings(): Promise<PublicSettings> {
  const result = await invoke<unknown>("get_settings");
  return parseBoundary(publicSettingsSchema, result, "get_settings");
}

export async function saveSettings(input: SaveSettingsInput): Promise<PublicSettings> {
  const settings = parseBoundary(saveSettingsInputSchema, input, "settings form");
  const result = await invoke<unknown>("save_settings", { settings });
  return parseBoundary(publicSettingsSchema, result, "save_settings");
}

export async function testOpenRouter(): Promise<void> {
  await invoke("test_openrouter");
}

export async function listModels(): Promise<Model[]> {
  const result = await invoke<unknown>("list_models");
  return parseBoundary(modelSchema.array(), result, "list_models");
}

export async function checkCurrentSelection(): Promise<WorkflowState> {
  const result = await invoke<unknown>("check_current_selection");
  return parseBoundary(workflowStateSchema, result, "check_current_selection");
}

export async function applyCorrection(correctionIndex: number): Promise<WorkflowState> {
  const result = await invoke<unknown>("apply_correction", { correctionIndex });
  return parseBoundary(workflowStateSchema, result, "apply_correction");
}

export async function dismissSuggestions(): Promise<WorkflowState> {
  const result = await invoke<unknown>("dismiss_suggestions");
  return parseBoundary(workflowStateSchema, result, "dismiss_suggestions");
}

export async function subscribeToWorkflow(
  onState: (state: WorkflowState) => void,
  onInvalidPayload: (error: Error) => void,
): Promise<UnlistenFn> {
  return listen<unknown>("emenda://workflow-state", ({ payload }) => {
    try {
      onState(parseBoundary(workflowStateSchema, payload, "emenda://workflow-state"));
    } catch (error) {
      onInvalidPayload(error instanceof Error ? error : new Error(String(error)));
    }
  });
}

export function errorMessage(error: unknown): string {
  if (error instanceof Error) {
    return error.message;
  }

  if (typeof error === "string") {
    return error;
  }

  if (error && typeof error === "object" && "message" in error) {
    return String(error.message);
  }

  return "An unexpected error occurred.";
}
