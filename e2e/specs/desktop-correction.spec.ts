import "@wdio/tauri-service";
import { browser, expect, $, $$ } from "@wdio/globals";

import { FixtureClient } from "../support/fixture.js";

const ORIGINAL_TEXT = "I liek this sentence.";
const EXPECTED_TEXT = "I like this sentence.";

type Editor = "notepad" | "vscode";

function selectedEditor(): Editor {
  const editor = process.env.EMENDA_E2E_EDITOR;
  if (editor !== "notepad" && editor !== "vscode") {
    throw new Error("EMENDA_E2E_EDITOR must be notepad or vscode");
  }
  return editor;
}

describe("the feature-gated desktop correction path", () => {
  it("applies one validated openrouter/free correction to the owned editor", async () => {
    const editor = selectedEditor();
    const fixture = FixtureClient.start();
    try {
      await fixture.request("hello");

      const settings = (await browser.tauri.execute(({ core }) =>
        core.invoke("get_settings"),
      )) as {
        apiKeyConfigured: boolean;
        modelId: string;
      };
      expect(settings.modelId).toBe("openrouter/free");
      expect(settings.apiKeyConfigured).toBe(true);
      await expect($("[aria-labelledby='ready-title']")).toBeDisplayed();

      await fixture.request("launch", {
        editor,
        originalText: ORIGINAL_TEXT,
        expectedText: EXPECTED_TEXT,
      });
      const trigger = await fixture.request<{ invocations: number }>("triggerHotkey");
      expect(trigger.invocations).toBe(1);

      await browser.waitUntil(
        async () =>
          (await $(".suggestions-view").isDisplayed()) ||
          (await $("section[role='alert']").isDisplayed()) ||
          (await $(".result-state").isDisplayed()),
        {
          timeout: 120_000,
          timeoutMsg: "the single openrouter/free attempt produced no terminal UI state",
        },
      );

      const alert = $("section[role='alert']");
      if (await alert.isDisplayed()) {
        throw new Error(`the single provider attempt failed: ${await alert.getText()}`);
      }
      const suggestions = $(".suggestions-view");
      if (!(await suggestions.isDisplayed())) {
        throw new Error(
          `the single provider attempt did not produce the required 'liek' to 'like' suggestion`,
        );
      }

      const expectedSource = editor === "notepad" ? "Notepad" : "Visual Studio Code";
      await expect($(".source-chip")).toHaveText(expectedSource);

      const cards = await $$(".correction-card");
      let matchingCard: WebdriverIO.Element | undefined;
      for (const card of cards) {
        const original = await card.$("del").getText();
        const replacement = await card.$("ins").getText();
        if (original === "liek" && replacement === "like") {
          matchingCard = card;
          break;
        }
      }
      if (!matchingCard) {
        throw new Error("validated suggestions omitted the exact 'liek' to 'like' correction");
      }

      await matchingCard.$(".correction-apply").click();
      const acceptedBanner = $(".accepted-banner");
      await browser.waitUntil(
        async () => !(await suggestions.isDisplayed()) || (await acceptedBanner.isDisplayed()),
        {
          timeout: 30_000,
          timeoutMsg: "accepting the exact correction produced no source-apply or review state",
        },
      );

      if (await alert.isDisplayed()) {
        throw new Error(`accepting the exact correction failed: ${await alert.getText()}`);
      }
      if ((await suggestions.isDisplayed()) && (await acceptedBanner.isDisplayed())) {
        const finalApply = $(".dismiss-button");
        await expect(finalApply).toHaveText("Apply 1 accepted change");
        await finalApply.click();
        await browser.waitUntil(
          async () => !(await suggestions.isDisplayed()) || (await alert.isDisplayed()),
          {
            timeout: 30_000,
            timeoutMsg: "the finalizer did not return control to the source editor",
          },
        );
      }
      if (await alert.isDisplayed()) {
        throw new Error(`applying the accepted correction failed: ${await alert.getText()}`);
      }

      // A successful product apply intentionally hides Emenda. The native
      // fixture proves success at the authoritative boundary: the owned
      // source window has focus and its persisted text is exactly expected.
      const verification = await fixture.request<{ verified: boolean }>("verifyAndSave");
      expect(verification.verified).toBe(true);
    } finally {
      await fixture.shutdown();
    }
  });
});
