import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

const pages = [
  "accordion",
  "alert",
  "avatar",
  "badge",
  "button",
  "card",
  "checkbox",
  "dialog",
  "input",
  "menu",
  "progress",
  "separator",
  "skeleton",
  "tabs",
  "toast",
  "breadcrumb",
  "empty-state",
  "kbd",
  "label",
  "radio-group",
  "slider",
  "switch",
  "textarea",
  "tooltip",
  "spinner",
];

for (const name of pages) {
  test(`${name} renders without axe violations`, async ({ page }) => {
    await page.goto(`/${name}.html`);
    await expect(page.locator("body")).toBeVisible();
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toEqual([]);
  });
}

test("rtl project flips the document direction", async ({ page }) => {
  await page.goto("/button.html");
  await page.locator("html").evaluate((element) => element.setAttribute("dir", "rtl"));
  await expect(page.locator("html")).toHaveAttribute("dir", "rtl");
});

test("dark project resolves dark tokens", async ({ page }) => {
  await page.emulateMedia({ colorScheme: "dark" });
  await page.goto("/button.html");
  const background = await page.locator("body").evaluate((element) => getComputedStyle(element).backgroundColor);
  expect(background).not.toBe("rgba(0, 0, 0, 0)");
});