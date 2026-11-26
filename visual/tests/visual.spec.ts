import { expect, test } from "@playwright/test";

const pages = [
  "accordion",
  "alert",
  "avatar",
  "badge",
  "breadcrumb",
  "button",
  "card",
  "checkbox",
  "dialog",
  "empty-state",
  "input",
  "kbd",
  "label",
  "menu",
  "progress",
  "radio-group",
  "separator",
  "skeleton",
  "slider",
  "spinner",
  "switch",
  "tabs",
  "textarea",
  "toast",
  "tooltip",
];

for (const name of pages) {
  test(`${name} matches the visual baseline`, async ({ page }) => {
    await page.goto(`/${name}.html`);
    await expect(page.locator("body")).toBeVisible();
    await expect(page).toHaveScreenshot(`${name}.png`);
  });
}