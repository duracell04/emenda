import { z } from "zod";

export const languageModeSchema = z.enum([
  "auto",
  "de-CH",
  "en-GB",
  "en-US",
  "fr-FR",
  "ka-GE",
  "ru-RU",
]);

export const publicSettingsSchema = z.object({
  apiKeyConfigured: z.boolean(),
  modelId: z.string().min(1),
  languageMode: languageModeSchema,
  hotkey: z.string().min(1),
});

export const saveSettingsInputSchema = z.object({
  apiKey: z.string().min(1).nullable().optional(),
  modelId: z.string().min(1),
  languageMode: languageModeSchema,
});

export const modelSchema = z.object({
  id: z.string().min(1),
  name: z.string().min(1),
  description: z.string().optional(),
});

export const correctionSchema = z
  .object({
    start: z.number().int().nonnegative(),
    end: z.number().int().nonnegative(),
    original: z.string(),
    replacement: z.string(),
    category: z.enum(["spelling", "grammar", "punctuation", "style"]),
    confidence: z.enum(["high", "medium", "low"]),
    explanation: z.string().optional(),
  })
  .refine(({ start, end }) => end >= start, {
    message: "A correction must end at or after its start position.",
  });

const errorAliases: Record<string, string> = {
  configurationerror: "configuration",
  authenticationerror: "authentication",
  networkerror: "network",
  inferenceerror: "inference",
  structuredoutputerror: "structuredOutput",
  validationerror: "validation",
  stalerevisionerror: "staleRevision",
  textcaptureerror: "textCapture",
  textreplacementerror: "textReplacement",
  protectedsurfaceerror: "protectedSurface",
};

function normaliseErrorKind(value: unknown): unknown {
  if (typeof value !== "string") {
    return value;
  }

  const compact = value.replaceAll(/[_\-\s]/g, "").toLowerCase();
  return errorAliases[compact] ?? value;
}

export const errorKindSchema = z.preprocess(
  normaliseErrorKind,
  z.enum([
    "configuration",
    "authentication",
    "network",
    "inference",
    "structuredOutput",
    "validation",
    "staleRevision",
    "textCapture",
    "textReplacement",
    "protectedSurface",
  ]),
);

export const workflowErrorSchema = z.object({
  kind: errorKindSchema,
  message: z.string().min(1),
});

const workflowContextSchema = z.object({
  revisionId: z.number().int().nonnegative().optional(),
  sourceText: z.string().optional(),
  workingText: z.string().optional(),
  sourceApplication: z
    .union([
      z.string().min(1),
      z
        .object({
          processId: z.number().int().nonnegative(),
          applicationName: z.string().min(1),
          executable: z.string().nullable().optional(),
          windowTitle: z.string(),
          windowId: z.string(),
        })
        .passthrough(),
    ])
    .optional(),
});

export const workflowStateSchema = z.discriminatedUnion("status", [
  workflowContextSchema.extend({ status: z.literal("idle") }),
  workflowContextSchema.extend({ status: z.literal("checking") }),
  workflowContextSchema.extend({
    status: z.literal("suggestions"),
    corrections: z.array(correctionSchema).min(1),
    acceptedCount: z.number().int().nonnegative().optional(),
  }),
  workflowContextSchema.extend({
    status: z.literal("clean"),
    applied: z.boolean().optional(),
  }),
  workflowContextSchema.extend({
    status: z.literal("error"),
    error: workflowErrorSchema,
  }),
]);

export type LanguageMode = z.infer<typeof languageModeSchema>;
export type PublicSettings = z.infer<typeof publicSettingsSchema>;
export type SaveSettingsInput = z.infer<typeof saveSettingsInputSchema>;
export type Model = z.infer<typeof modelSchema>;
export type Correction = z.infer<typeof correctionSchema>;
export type ErrorKind = z.infer<typeof errorKindSchema>;
export type WorkflowError = z.infer<typeof workflowErrorSchema>;
export type WorkflowState = z.infer<typeof workflowStateSchema>;
