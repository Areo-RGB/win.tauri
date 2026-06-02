export interface ICC3API {
  minimize: () => Promise<void>;
  maximize: () => Promise<void>;
  close: () => Promise<void>;
  getDevices: () => Promise<{id: string, state: string}[]>;
  launchScrcpy: (deviceId?: string, customArgs?: string[]) => Promise<void>;
  adbConnect: (ip: string) => Promise<{success: boolean, message: string}>;
  adbScreenshot: (deviceId: string) => Promise<{success: boolean, message: string}>;
  scanConfigs: () => Promise<{mcpConfigs: string[], ngrokConfigs: string[]}>;
  launchMcp: (port: number | string, configFileName: string) => Promise<void>;
  launchNgrok: (portOrAddress: string, url: string) => Promise<void>;
  launchKillPort: (port: string) => Promise<void>;
  launchMcpProxy: (port: string) => Promise<void>;
  launchSerena: (port: string) => Promise<void>;
  mcpApiHealth: (baseUrl: string) => Promise<{success: boolean, data?: any, message?: string}>;
  mcpApiRefreshAll: (baseUrl: string) => Promise<{success: boolean, data?: any, message?: string}>;
  mcpApiServerStart: (baseUrl: string, serverName: string) => Promise<{success: boolean, data?: any, message?: string}>;
  mcpApiServerStop: (baseUrl: string, serverName: string) => Promise<{success: boolean, data?: any, message?: string}>;
  mcpApiServerDisable: (baseUrl: string, serverName: string) => Promise<{success: boolean, data?: any, message?: string}>;
  mcpApiServerRefresh: (baseUrl: string, serverName: string) => Promise<{success: boolean, data?: any, message?: string}>;
  mcpApiRestart: (baseUrl: string) => Promise<{success: boolean, data?: any, message?: string}>;
  gitStatus: (cwd: string) => Promise<{success: boolean, stdout: string, stderr: string}>;
  gitRemote: (cwd: string) => Promise<{success: boolean, stdout: string, stderr: string}>;
  gitConnectRemote: (cwd: string, url: string) => Promise<{success: boolean, stdout: string, stderr: string}>;
  gitCreateRemote: (cwd: string) => Promise<{success: boolean, stdout: string, stderr: string}>;
  gitFetch: (cwd: string) => Promise<{success: boolean, stdout: string, stderr: string}>;
  gitStagePush: (cwd: string) => Promise<{success: boolean, stdout: string, stderr: string}>;
  zipProject: (cwd: string) => Promise<{success: boolean, stdout: string, stderr: string}>;
}

declare global {
  interface Window {
    cc3API: ICC3API;
  }
}
