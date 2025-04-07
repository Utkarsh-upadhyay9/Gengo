// api.ts - API client for backend communication
import axios from 'axios';
import { Session, Feedback, User } from '../types';

// Base URL for the API
const API_URL = 'http://localhost:8000'; // Update to match your Rust backend URL

export const api = {
  // User authentication
  login: async (email: string, password: string) => {
    const response = await axios.post(`${API_URL}/api/auth/login`, {
      email,
      password
    });
    return response.data;
  },
  
  // User profile
  getUserProfile: async (): Promise<User> => {
    const response = await axios.get<User>(`${API_URL}/api/user/profile`);
    return response.data;
  },
  
  // Get user's practice sessions
  getUserSessions: async (): Promise<Session[]> => {
    const response = await axios.get<Session[]>(`${API_URL}/api/sessions`);
    return response.data;
  },
  
  // Submit a practice recording
  submitRecording: async (audioBlob: Blob): Promise<{ sessionId: string }> => {
    const formData = new FormData();
    formData.append('audio', audioBlob);
    
    const response = await axios.post<{ sessionId: string }>(
      `${API_URL}/api/practice/submit`, 
      formData
    );
    return response.data;
  },
  
  // Get feedback for a session
  getFeedback: async (sessionId: string): Promise<Feedback> => {
    const response = await axios.get<Feedback>(`${API_URL}/api/practice/${sessionId}/feedback`);
    return response.data;
  }
};

export default api;