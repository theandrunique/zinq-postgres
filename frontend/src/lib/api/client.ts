import axios, { AxiosError, AxiosHeaders, type AxiosRequestConfig } from "axios";
import { auth } from "#lib/auth.svelte.ts";

export interface ApiErrorOptions {
  code: string;
  message: string;
  status?: number;
  errors?: Record<string, string[]>;
  metadata?: Record<string, string>;
  cause?: unknown;
}

export class ApiError extends Error {
  readonly code: string;
  readonly status?: number;
  readonly errors?: Record<string, string[]>;
  readonly metadata?: Record<string, string>;

  constructor(options: ApiErrorOptions) {
    super(options.message);
    this.name = "ApiError";
    this.code = options.code;
    this.status = options.status;
    this.errors = options.errors;
    this.metadata = options.metadata;

    if (options.cause !== undefined) {
      Object.defineProperty(this, "cause", {
        value: options.cause,
        configurable: true,
        writable: true,
      });
    }
  }

  static fromAxios(error: AxiosError): ApiError {
    const response = error.response;
    const data = response?.data;

    if (data && typeof data === "object") {
      const record = data as Record<string, unknown>;

      return new ApiError({
        code: typeof record.code === "string" ? record.code : "UNKNOWN_ERROR",
        message:
          typeof record.message === "string" && record.message.length > 0
            ? record.message
            : error.message || "Request failed",
        status: response?.status,
        errors:
          record.errors && typeof record.errors === "object" ? (record.errors as Record<string, string[]>) : undefined,
        metadata:
          record.metadata && typeof record.metadata === "object"
            ? (record.metadata as Record<string, string>)
            : undefined,
        cause: error,
      });
    }

    return new ApiError({
      code: error.code === "ERR_NETWORK" ? "NETWORK_ERROR" : "UNKNOWN_ERROR",
      message: error.message || "Request failed",
      status: response?.status,
      cause: error,
    });
  }
}

export function isApiError(error: unknown): error is ApiError {
  return error instanceof ApiError;
}

interface RetryConfig extends AxiosRequestConfig {
  _retry?: boolean;
}

export const api = axios.create({
  baseURL: "/api/v1",
  headers: {
    Accept: "application/json",
  },
});

api.interceptors.request.use(async (config) => {
  const token = await auth.getAccessToken();

  if (token) {
    config.headers = AxiosHeaders.from(config.headers);
    config.headers.set("Authorization", `Bearer ${token}`);
  }

  return config;
});

api.interceptors.response.use(
  (response) => response,
  async (error: AxiosError) => {
    const config = error.config as RetryConfig | undefined;

    if (error.response?.status === 401 && config && !config._retry) {
      config._retry = true;

      try {
        const user = await auth.refresh();

        config.headers = AxiosHeaders.from(config.headers as unknown as AxiosHeaders);
        (config.headers as AxiosHeaders).set("Authorization", `Bearer ${user.access_token}`);

        return api(config);
      } catch {
        // Fall through to a normalized ApiError below.
      }
    }

    throw ApiError.fromAxios(error);
  },
);
