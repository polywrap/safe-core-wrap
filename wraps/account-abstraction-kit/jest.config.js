module.exports = {
  collectCoverage: false,
  preset: "ts-jest",
  testEnvironment: "node",
  testMatch: ["**/tests/**/?(*.)+(spec|test).[jt]s?(x)"],
  globals: {
    "ts-jest": {
      tsconfig: "tsconfig.json",
      diagnostics: false,
    },
  },
  testPathIgnorePatterns: [
    "/.polywrap/"
  ],
};
