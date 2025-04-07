// index.ts - Type definitions for the application

// User profile
export interface User {
  id: string;
  username: string;
  email: string;
  preferredLanguage: string;
}

// Practice session
export interface Session {
  id: string;
  userId: string;
  audioUrl: string;
  transcription: string;
  score: number;
  createdAt: string;
}

// Feedback data
export interface Feedback {
  id: string;
  sessionId: string;
  score: number;
  pronunciation: string;
  grammar: string;
  suggestions: string[];
}

// Authentication
export interface AuthResponse {
  token: string;
  userId: string;
}

export interface LoginCredentials {
  email: string;
  password: string;
}