export type AppVersion = {
  version: string;
  commit: string;
  dev: boolean;
};

export const appVersion: AppVersion = __APP_VERSION__;

export const versionLabel = `v${appVersion.version}${appVersion.dev ? "-dev" : ""}`;
