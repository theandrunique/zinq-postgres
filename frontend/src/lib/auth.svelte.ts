const DEFAULT_BASE_URL = "/api/v1";
const DEFAULT_STORAGE_KEY = "zinq.auth.user";
const DEFAULT_EXPIRING_NOTIFICATION_SECONDS = 60;
const RENEW_RETRY_DELAYS_MS = [5_000, 15_000, 30_000];

function nowSeconds(): number {
  return Math.floor(Date.now() / 1000);
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

export type UserSessionLifetime = "WEEK" | "MONTH" | "MONTH3" | "MONTH6" | "MONTH12";

export interface UserProfile {
  id: string;
  username: string;
  display_name: string;
  bio: string | null;
  avatar: string | null;
  created_at: string;
  sessions_ttl: UserSessionLifetime;
  email: string;
  email_verified: boolean;
}

export interface AuthUserData {
  access_token: string;
  refresh_token: string;
  token_type: "Bearer";
  expires_at: number;
  profile: UserProfile;
}

export class AuthUser {
  readonly access_token: string;
  readonly refresh_token: string;
  readonly token_type: "Bearer";
  readonly expires_at: number;
  readonly profile: UserProfile;

  constructor(data: AuthUserData) {
    this.access_token = data.access_token;
    this.refresh_token = data.refresh_token;
    this.token_type = "Bearer";
    this.expires_at = data.expires_at;
    this.profile = data.profile;
  }

  get expired(): boolean {
    return nowSeconds() >= this.expires_at;
  }

  get expires_in(): number {
    return Math.max(0, this.expires_at - nowSeconds());
  }

  toJSON(): AuthUserData {
    return {
      access_token: this.access_token,
      refresh_token: this.refresh_token,
      token_type: this.token_type,
      expires_at: this.expires_at,
      profile: this.profile,
    };
  }

  static fromJSON(value: unknown): AuthUser {
    if (!isRecord(value)) {
      throw new Error("Invalid stored auth user");
    }

    const { access_token, refresh_token, expires_at, profile } = value;

    if (
      typeof access_token !== "string" ||
      typeof refresh_token !== "string" ||
      typeof expires_at !== "number" ||
      !Number.isFinite(expires_at) ||
      !isRecord(profile) ||
      typeof profile.id !== "string" ||
      typeof profile.username !== "string"
    ) {
      throw new Error("Invalid stored auth user");
    }

    return new AuthUser({
      access_token,
      refresh_token,
      token_type: "Bearer",
      expires_at,
      profile: profile as unknown as UserProfile,
    });
  }
}

export interface AuthErrorOptions {
  code: string;
  message: string;
  status?: number;
  errors?: Record<string, string[]>;
  metadata?: Record<string, string>;
  cause?: unknown;
}

export class AuthError extends Error {
  readonly code: string;
  readonly status?: number;
  readonly errors?: Record<string, string[]>;
  readonly metadata?: Record<string, string>;

  constructor(options: AuthErrorOptions) {
    super(options.message);
    this.name = "AuthError";
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

  get requiresTotp(): boolean {
    return this.code === "AUTH_TOTP_REQUIRED";
  }

  static async fromResponse(response: Response): Promise<AuthError> {
    let body: unknown;

    try {
      body = await response.json();
    } catch {
      body = null;
    }

    const record = isRecord(body) ? body : {};
    const code = typeof record.code === "string" ? record.code : "UNKNOWN_ERROR";
    const message =
      typeof record.message === "string" && record.message.length > 0
        ? record.message
        : response.statusText || "Request failed";
    const errors = isRecord(record.errors) ? (record.errors as Record<string, string[]>) : undefined;
    const metadata = isRecord(record.metadata) ? (record.metadata as Record<string, string>) : undefined;

    return new AuthError({
      code,
      message,
      status: response.status,
      errors,
      metadata,
    });
  }
}

function isSessionInvalid(cause: unknown): boolean {
  return cause instanceof AuthError && (cause.status === 401 || cause.code === "AUTH_INVALID_TOKEN");
}

export interface SigninCredentials {
  username: string;
  password: string;
  totp?: string;
}

export interface SignupData {
  username: string;
  email: string;
  display_name: string;
  password: string;
}

export interface AuthManagerSettings {
  baseUrl?: string;
  storageKey?: string;
  automaticSilentRenew?: boolean;
  accessTokenExpiringNotificationTimeInSeconds?: number;
  monitorSession?: boolean;
}

interface TokensResponse {
  access_token: string;
  refresh_token: string;
  expires_in: number;
}

export class AuthManager {
  user = $state<AuthUser | null>(null);

  private readonly baseUrl: string;
  private readonly storageKey: string;
  private readonly automaticSilentRenew: boolean;
  private readonly expiringNotificationTime: number;
  private readonly monitorSession: boolean;

  private restored = false;
  private refreshPromise: Promise<AuthUser> | null = null;
  private renewTimer: ReturnType<typeof setTimeout> | null = null;
  private expiryTimer: ReturnType<typeof setTimeout> | null = null;
  private storageListenerAttached = false;
  private sessionVersion = 0;
  private renewRetry = 0;

  constructor(settings: AuthManagerSettings = {}) {
    this.baseUrl = (settings.baseUrl ?? DEFAULT_BASE_URL).replace(/\/+$/, "");
    this.storageKey = settings.storageKey ?? DEFAULT_STORAGE_KEY;
    this.automaticSilentRenew = settings.automaticSilentRenew ?? true;
    this.expiringNotificationTime =
      settings.accessTokenExpiringNotificationTimeInSeconds ?? DEFAULT_EXPIRING_NOTIFICATION_SECONDS;
    this.monitorSession = settings.monitorSession ?? true;
    this.attachStorageListener();
  }

  async getUser(): Promise<AuthUser | null> {
    this.restoreIfNeeded();

    let user = this.user;

    if (!user) {
      return null;
    }

    if (user.expired) {
      try {
        user = await this.refresh();
      } catch (cause) {
        if (isSessionInvalid(cause)) {
          this.clearSession();
          return null;
        }

        if (cause instanceof AuthError && cause.code === "STALE_REFRESH") {
          return this.user;
        }

        throw cause;
      }
    }

    return user;
  }

  async signin(credentials: SigninCredentials): Promise<AuthUser> {
    this.sessionVersion += 1;

    const tokens = await this.request<TokensResponse>("/auth/sign-in", {
      method: "POST",
      body: JSON.stringify({
        username: credentials.username,
        password: credentials.password,
        totp: credentials.totp,
      }),
    });

    const user = await this.createUserFromTokens(tokens);
    this.persistAndSetUser(user);

    return user;
  }

  async signup(data: SignupData): Promise<AuthUser> {
    await this.request<UserProfile>("/auth/sign-up", {
      method: "POST",
      body: JSON.stringify(data),
    });

    return this.signin({
      username: data.username,
      password: data.password,
    });
  }

  refresh(): Promise<AuthUser> {
    if (this.refreshPromise) {
      return this.refreshPromise;
    }

    this.refreshPromise = this.doRefresh().finally(() => {
      this.refreshPromise = null;
    });

    return this.refreshPromise;
  }

  async getAccessToken(): Promise<string | null> {
    const user = await this.getUser();
    return user?.access_token ?? null;
  }

  async signout(): Promise<void> {
    this.clearSession();
  }

  private restoreIfNeeded(): void {
    if (this.restored) {
      return;
    }

    this.restored = true;

    const stored = this.readStoredUser();

    if (!stored) {
      return;
    }

    if (stored.expired) {
      this.user = stored;
      return;
    }

    this.setUser(stored);
  }

  private async doRefresh(): Promise<AuthUser> {
    const current = this.user ?? this.readStoredUser();

    if (!current) {
      throw new AuthError({
        code: "NOT_AUTHENTICATED",
        message: "Not authenticated",
      });
    }

    const version = this.sessionVersion;

    const tokens = await this.request<TokensResponse>("/auth/refresh", {
      method: "POST",
      body: JSON.stringify({ refresh_token: current.refresh_token }),
    });

    if (version !== this.sessionVersion) {
      throw new AuthError({
        code: "STALE_REFRESH",
        message: "Session changed during refresh",
      });
    }

    let profile = current.profile;

    try {
      profile = await this.fetchProfile(tokens.access_token);
    } catch {
      // Keep the previous profile if userinfo is temporarily unavailable.
    }

    const user = new AuthUser({
      access_token: tokens.access_token,
      refresh_token: tokens.refresh_token,
      token_type: "Bearer",
      expires_at: nowSeconds() + tokens.expires_in,
      profile,
    });

    this.persistAndSetUser(user);

    return user;
  }

  private async createUserFromTokens(tokens: TokensResponse): Promise<AuthUser> {
    const profile = await this.fetchProfile(tokens.access_token);

    return new AuthUser({
      access_token: tokens.access_token,
      refresh_token: tokens.refresh_token,
      token_type: "Bearer",
      expires_at: nowSeconds() + tokens.expires_in,
      profile,
    });
  }

  private fetchProfile(accessToken: string): Promise<UserProfile> {
    return this.request<UserProfile>("/users/@me", {
      headers: {
        Authorization: `Bearer ${accessToken}`,
      },
    });
  }

  private async request<T>(path: string, init: RequestInit = {}): Promise<T> {
    const headers = new Headers(init.headers);
    headers.set("Accept", "application/json");

    if (init.body !== undefined) {
      headers.set("Content-Type", "application/json");
    }

    let response: Response;

    try {
      response = await fetch(`${this.baseUrl}${path}`, {
        ...init,
        headers,
      });
    } catch (cause) {
      throw new AuthError({
        code: "NETWORK_ERROR",
        message: "Failed to reach the server",
        cause,
      });
    }

    if (!response.ok) {
      throw await AuthError.fromResponse(response);
    }

    if (response.status === 204) {
      return undefined as T;
    }

    return (await response.json()) as T;
  }

  private persistAndSetUser(user: AuthUser): void {
    this.writeStoredUser(user);
    this.setUser(user);
  }

  private readStoredUser(): AuthUser | null {
    if (typeof window === "undefined") {
      return null;
    }

    try {
      const raw = window.localStorage.getItem(this.storageKey);

      if (!raw) {
        return null;
      }

      return AuthUser.fromJSON(JSON.parse(raw));
    } catch {
      this.clearStoredUser();
      return null;
    }
  }

  private writeStoredUser(user: AuthUser): void {
    if (typeof window === "undefined") {
      return;
    }

    try {
      window.localStorage.setItem(this.storageKey, JSON.stringify(user));
    } catch {
      // Storage may be unavailable or full; the session still works in memory.
    }
  }

  private clearStoredUser(): void {
    if (typeof window === "undefined") {
      return;
    }

    try {
      window.localStorage.removeItem(this.storageKey);
    } catch {
      // Ignore storage failures during logout.
    }
  }

  private setUser(user: AuthUser | null): void {
    this.user = user;
    this.renewRetry = 0;
    this.clearTimers();

    if (user && this.automaticSilentRenew) {
      this.scheduleRenew(user);
    }
  }

  private scheduleRenew(user: AuthUser): void {
    const delayMs = Math.max(0, (user.expires_at - this.expiringNotificationTime - nowSeconds()) * 1000);

    this.renewTimer = setTimeout(() => {
      this.renewTimer = null;
      void this.renewSilently();
    }, delayMs);
  }

  private async renewSilently(): Promise<void> {
    try {
      await this.refresh();
    } catch (cause) {
      if (cause instanceof AuthError && cause.code === "STALE_REFRESH") {
        return;
      }

      if (isSessionInvalid(cause)) {
        this.clearSession();
        return;
      }

      this.scheduleRenewRetry();
    }
  }

  private scheduleRenewRetry(): void {
    this.clearRenewTimer();

    if (this.renewRetry >= RENEW_RETRY_DELAYS_MS.length) {
      this.scheduleExpiry();
      return;
    }

    const delayMs = RENEW_RETRY_DELAYS_MS[this.renewRetry];
    this.renewRetry += 1;

    this.renewTimer = setTimeout(() => {
      this.renewTimer = null;
      void this.renewSilently();
    }, delayMs);
  }

  private scheduleExpiry(): void {
    this.clearExpiryTimer();

    const user = this.user;

    if (!user) {
      return;
    }

    const delayMs = Math.max(0, (user.expires_at - nowSeconds()) * 1000);

    this.expiryTimer = setTimeout(() => {
      this.expiryTimer = null;
      this.clearSession();
    }, delayMs);
  }

  private clearSession(): void {
    this.sessionVersion += 1;
    this.renewRetry = 0;
    this.clearTimers();
    this.user = null;
    this.clearStoredUser();
  }

  private clearTimers(): void {
    this.clearRenewTimer();
    this.clearExpiryTimer();
  }

  private clearRenewTimer(): void {
    if (this.renewTimer !== null) {
      clearTimeout(this.renewTimer);
      this.renewTimer = null;
    }
  }

  private clearExpiryTimer(): void {
    if (this.expiryTimer !== null) {
      clearTimeout(this.expiryTimer);
      this.expiryTimer = null;
    }
  }

  private attachStorageListener(): void {
    if (!this.monitorSession || this.storageListenerAttached || typeof window === "undefined") {
      return;
    }

    this.storageListenerAttached = true;
    window.addEventListener("storage", this.handleStorage);
  }

  private readonly handleStorage = (event: StorageEvent): void => {
    if (event.key !== this.storageKey) {
      return;
    }

    if (!event.newValue) {
      this.clearTimers();
      this.user = null;
      return;
    }

    const stored = this.readStoredUser();

    if (stored) {
      this.setUser(stored);
    }
  };
}

export const auth = new AuthManager();
