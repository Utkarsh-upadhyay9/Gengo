// Dashboard.tsx - User dashboard page
import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { api } from '../services/api';
import { Session } from '../types';

const Dashboard = () => {
  const [sessions, setSessions] = useState<Session[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  
  useEffect(() => {
    const fetchData = async () => {
      try {
        // In a real app, we'd get this from the API
        // const data = await api.getUserSessions();
        
        // For demo purposes, using mock data
        const mockSessions: Session[] = [
          {
            id: '1',
            userId: 'user1',
            audioUrl: '/audio/session1.mp3',
            transcription: 'Good morning! How are you doing today?',
            score: 85,
            createdAt: '2025-04-05T14:30:00Z'
          },
          {
            id: '2',
            userId: 'user1',
            audioUrl: '/audio/session2.mp3',
            transcription: 'I am learning to speak a new language.',
            score: 72,
            createdAt: '2025-04-04T10:15:00Z'
          }
        ];
        
        setSessions(mockSessions);
      } catch (err) {
        setError('Failed to load sessions');
        console.error(err);
      } finally {
        setLoading(false);
      }
    };
    
    fetchData();
  }, []);
  
  if (loading) return <div className="loading">Loading dashboard...</div>;
  if (error) return <div className="error">{error}</div>;
  
  return (
    <div className="dashboard-container">
      <header className="dashboard-header">
        <h1>Your Learning Dashboard</h1>
        <Link to="/practice" className="btn btn-primary">
          Start New Practice
        </Link>
      </header>
      
      <section className="dashboard-stats">
        <div className="stat-card">
          <h3>Sessions</h3>
          <div className="stat-value">{sessions.length}</div>
        </div>
        <div className="stat-card">
          <h3>Average Score</h3>
          <div className="stat-value">
            {sessions.length 
              ? Math.round(sessions.reduce((sum, session) => sum + session.score, 0) / sessions.length)
              : 0}%
          </div>
        </div>
        <div className="stat-card">
          <h3>Progress</h3>
          <div className="stat-value">Beginner</div>
        </div>
      </section>
      
      <section className="recent-sessions">
        <h2>Recent Practice Sessions</h2>
        {sessions.length === 0 ? (
          <div className="empty-state">
            <p>You haven't completed any practice sessions yet.</p>
            <Link to="/practice" className="btn btn-secondary">
              Start Practicing
            </Link>
          </div>
        ) : (
          <div className="sessions-list">
            {sessions.map(session => (
              <div key={session.id} className="session-card">
                <div className="session-info">
                  <div className="session-date">
                    {new Date(session.createdAt).toLocaleDateString()}
                  </div>
                  <div className="session-text">
                    <h3>Practice Session #{session.id}</h3>
                    <p>{session.transcription}</p>
                  </div>
                </div>
                <div className="session-score">
                  <div className="score-circle">
                    <span>{session.score}%</span>
                  </div>
                </div>
                <div className="session-actions">
                  <Link to={`/practice/${session.id}/feedback`} className="btn btn-sm">
                    View Feedback
                  </Link>
                </div>
              </div>
            ))}
          </div>
        )}
      </section>
    </div>
  );
};

export default Dashboard;