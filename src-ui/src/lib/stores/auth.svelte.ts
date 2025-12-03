/**
 * Authentication Store
 */

declare const __API_URL__: string | undefined;

const API_URL = typeof __API_URL__ !== 'undefined' ? __API_URL__ : 'http://localhost:3000';

export interface User {
    id: string;
    email: string;
    username: string | null;
    email_verified: boolean;
    total_xp: number;
    created_at: string;
}

export interface AuthState {
    user: User | null;
    token: string | null;
    isAuthenticated: boolean;
    isLoading: boolean;
    isEmailVerified: boolean;
    error: string | null;
}

interface AuthResponse {
    token: string;
    user: User;
}

interface MessageResponse {
    message: string;
}

const TOKEN_KEY = 'code-warrior-auth-token';
const USER_KEY = 'code-warrior-auth-user';

// Simple mutable state (not using $state rune to avoid SSR issues)
let _user: User | null = null;
let _token: string | null = null;
let _isAuthenticated = false;
let _isLoading = true;
let _isEmailVerified = false;
let _error: string | null = null;

// Initialize from localStorage (client-side only)
function initFromStorage() {
    if (typeof window === 'undefined') {
        _isLoading = false;
        return;
    }
    
    const storedToken = localStorage.getItem(TOKEN_KEY);
    const storedUser = localStorage.getItem(USER_KEY);

    if (storedToken && storedUser) {
        try {
            _user = JSON.parse(storedUser) as User;
            _token = storedToken;
            _isAuthenticated = true;
            _isEmailVerified = _user.email_verified;
        } catch {
            localStorage.removeItem(TOKEN_KEY);
            localStorage.removeItem(USER_KEY);
        }
    }
    _isLoading = false;
}

// Run init
initFromStorage();

function persistAuth(token: string, user: User): void {
    if (typeof window !== 'undefined') {
        localStorage.setItem(TOKEN_KEY, token);
        localStorage.setItem(USER_KEY, JSON.stringify(user));
    }
}

function clearAuth(): void {
    if (typeof window !== 'undefined') {
        localStorage.removeItem(TOKEN_KEY);
        localStorage.removeItem(USER_KEY);
    }
}

async function authRequest<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
    const headers: Record<string, string> = {
        'Content-Type': 'application/json',
        ...((options.headers as Record<string, string>) || {}),
    };

    if (_token) {
        headers['Authorization'] = `Bearer ${_token}`;
    }

    const response = await fetch(`${API_URL}${endpoint}`, {
        ...options,
        headers,
    });

    if (!response.ok) {
        const errorText = await response.text();
        throw new Error(errorText || `HTTP ${response.status}`);
    }

    return response.json();
}

export async function register(
    email: string,
    password: string,
    username?: string
): Promise<{ success: boolean; message: string }> {
    _isLoading = true;
    _error = null;

    try {
        const response = await authRequest<AuthResponse>('/api/auth/register', {
            method: 'POST',
            body: JSON.stringify({ email, password, username }),
        });

        persistAuth(response.token, response.user);
        _user = response.user;
        _token = response.token;
        _isAuthenticated = true;
        _isEmailVerified = response.user.email_verified;
        _isLoading = false;

        return { success: true, message: 'Registration successful' };
    } catch (err) {
        const message = err instanceof Error ? err.message : 'Registration failed';
        _error = message;
        _isLoading = false;
        return { success: false, message };
    }
}

export async function login(
    email: string,
    password: string
): Promise<{ success: boolean; message: string }> {
    _isLoading = true;
    _error = null;

    try {
        const response = await authRequest<AuthResponse>('/api/auth/login', {
            method: 'POST',
            body: JSON.stringify({ email, password }),
        });

        persistAuth(response.token, response.user);
        _user = response.user;
        _token = response.token;
        _isAuthenticated = true;
        _isEmailVerified = response.user.email_verified;
        _isLoading = false;

        return { success: true, message: 'Login successful' };
    } catch (err) {
        const message = err instanceof Error ? err.message : 'Login failed';
        _error = message;
        _isLoading = false;
        return { success: false, message };
    }
}

export async function logout(): Promise<void> {
    try {
        await authRequest<MessageResponse>('/api/auth/logout', { method: 'POST' });
    } catch {
        // Logout even if API call fails
    }

    clearAuth();
    _user = null;
    _token = null;
    _isAuthenticated = false;
    _isLoading = false;
    _isEmailVerified = false;
    _error = null;
}

export async function verifyEmail(
    token: string
): Promise<{ success: boolean; message: string }> {
    _isLoading = true;
    _error = null;

    try {
        await authRequest<{ success: boolean }>('/api/auth/verify-email', {
            method: 'POST',
            body: JSON.stringify({ token }),
        });

        if (_user) {
            _user.email_verified = true;
            _isEmailVerified = true;
            persistAuth(_token!, _user);
        }

        _isLoading = false;
        return { success: true, message: 'Email verified successfully' };
    } catch (err) {
        const message = err instanceof Error ? err.message : 'Verification failed';
        _error = message;
        _isLoading = false;
        return { success: false, message };
    }
}

export async function resendVerification(
    email: string
): Promise<{ success: boolean; message: string }> {
    _isLoading = true;

    try {
        const response = await authRequest<MessageResponse>('/api/auth/resend-verify', {
            method: 'POST',
            body: JSON.stringify({ email }),
        });

        _isLoading = false;
        return { success: true, message: response.message };
    } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to resend verification';
        _isLoading = false;
        return { success: false, message };
    }
}

export async function requestPasswordReset(
    email: string
): Promise<{ success: boolean; message: string }> {
    _isLoading = true;

    try {
        const response = await authRequest<MessageResponse>('/api/auth/request-reset', {
            method: 'POST',
            body: JSON.stringify({ email }),
        });

        _isLoading = false;
        return { success: true, message: response.message };
    } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to request reset';
        _isLoading = false;
        return { success: false, message };
    }
}

export async function resetPassword(
    token: string,
    newPassword: string
): Promise<{ success: boolean; message: string }> {
    _isLoading = true;

    try {
        const response = await authRequest<MessageResponse>('/api/auth/reset-password', {
            method: 'POST',
            body: JSON.stringify({ token, new_password: newPassword }),
        });

        _isLoading = false;
        return { success: true, message: response.message };
    } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to reset password';
        _isLoading = false;
        return { success: false, message };
    }
}

export async function refreshUser(): Promise<void> {
    if (!_token) return;

    try {
        const user = await authRequest<User>('/api/auth/me');
        _user = user;
        _isEmailVerified = user.email_verified;
        persistAuth(_token, user);
    } catch {
        await logout();
    }
}

export function handleOAuthCallback(): boolean {
    if (typeof window === 'undefined') return false;

    const params = new URLSearchParams(window.location.search);
    const token = params.get('token');
    const userId = params.get('user_id');

    if (token && userId) {
        _token = token;
        _isLoading = true;
        window.history.replaceState({}, '', window.location.pathname);
        refreshUser().then(() => {
            _isLoading = false;
            _isAuthenticated = true;
        });
        return true;
    }

    return false;
}

export function startGoogleOAuth(): void {
    if (typeof window !== 'undefined') {
        window.location.href = `${API_URL}/api/auth/oauth/google/start`;
    }
}

export function startGitHubOAuth(): void {
    if (typeof window !== 'undefined') {
        window.location.href = `${API_URL}/api/auth/oauth/github/start`;
    }
}

export function getAuthState(): AuthState {
    return {
        user: _user,
        token: _token,
        isAuthenticated: _isAuthenticated,
        isLoading: _isLoading,
        isEmailVerified: _isEmailVerified,
        error: _error,
    };
}

export function getUser(): User | null {
    return _user;
}

export function getToken(): string | null {
    return _token;
}

export function isAuthenticated(): boolean {
    return _isAuthenticated;
}

export function isEmailVerified(): boolean {
    return _isEmailVerified;
}

export function isLoading(): boolean {
    return _isLoading;
}

export function getError(): string | null {
    return _error;
}
