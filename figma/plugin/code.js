// Creates a Figma variable collection with light/dark modes for Ferrum UI.
// Import this folder as a local Figma plugin, then run "Create Ferrum UI variables".

const TOKENS = {
  light: {
    bg: "#ffffff",
    fg: "#0b0b0f",
    muted: "#f4f4f5",
    "muted-fg": "#71717a",
    accent: "#f4f4f5",
    "accent-fg": "#18181b",
    primary: "#18181b",
    "primary-fg": "#fafafa",
    destructive: "#dc2626",
    border: "#e4e4e7",
    ring: "#a1a1aa",
  },
  dark: {
    bg: "#0a0a0b",
    fg: "#fafafa",
    muted: "#27272a",
    "muted-fg": "#a1a1aa",
    accent: "#27272a",
    "accent-fg": "#fafafa",
    primary: "#fafafa",
    "primary-fg": "#18181b",
    destructive: "#b91c1c",
    border: "#27272a",
    ring: "#d4d4d8",
  },
};

function hexToRgb(hex) {
  const value = hex.replace("#", "");
  return {
    r: parseInt(value.slice(0, 2), 16) / 255,
    g: parseInt(value.slice(2, 4), 16) / 255,
    b: parseInt(value.slice(4, 6), 16) / 255,
    a: 1,
  };
}

async function main() {
  const collection = figma.variables.createVariableCollection("Ferrum UI");
  const lightMode = collection.modes[0].modeId;
  const darkMode = collection.addMode("dark");

  for (const name of Object.keys(TOKENS.light)) {
    const variable = figma.variables.createVariable(name, collection, "COLOR");
    variable.setValueForMode(lightMode, hexToRgb(TOKENS.light[name]));
    variable.setValueForMode(darkMode, hexToRgb(TOKENS.dark[name]));
  }

  figma.closePlugin("Created Ferrum UI variables");
}

main();