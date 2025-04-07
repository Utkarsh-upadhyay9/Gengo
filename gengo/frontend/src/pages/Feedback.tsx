// Feedback.tsx - Feedback display after practice
import { useEffect, useState } from 'react';
import { useParams, Link } from 'react-router-dom';
import { api } from '../services/api';
import { Feedback as FeedbackType } from '../types';
import '../styles/Feedback.css';

const Feedback = () => {
  const { id } = useParams<{ id: string }>();
  const [feedback, setFeedback] = useState<FeedbackType | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  
  useEffect(() => {
    const fetchFeedback = async () => {
      try {
        if (id) {
          // In a real app, we'd get this from the API
          // const data = await api.getFeedback(id);
          
          // For demo purposes, using mock data
          const mockFeedback: FeedbackType = {
            id: '1',
            sessionId: id,
            score: 85,
            pronunciation: 'Good pronunciation overall. Work on the "th" sound in "the".',
            grammar: 'Good grammar structure. Watch subject-verb agreement in complex sentences.',
            suggestions: [
              'Practice the "th" sound more deliberately',
              'Pay attention to verb tenses in longer sentences',
              'Continue working on intonation patterns'
            ]
          };
          
          setFeedback(mockFeedback);
        } else {
          setError("No session ID provided");
        }
      } catch (err) {
        setError("Failed to load feedback");
        console.error(err);
      } finally {
        setLoading(false);
      }
    };
    
    fetchFeedback();
  }, [id]);
  
  if (loading) return <div className="loading">Loading feedback...</div>;
  if (error) return <div className="error">{error}</div>;
  if (!feedback) return <div className="error">No feedback available</div>;
  
  return (
    <div className="feedback-container">
      <h1>Your Speaking Feedback</h1>
      
      <div className="feedback-score">
        <div className="score-circle">
          <span className="score-value">{feedback.score}%</span>
        </div>
        <p className="score-label">
          {feedback.score >= 80 ? "Great job!" : 
           feedback.score >= 60 ? "Good effort!" : "Keep practicing!"}
        </p>
      </div>
      
      <div className="feedback-details">
        <section className="feedback-section">
          <h2>Pronunciation Analysis</h2>
          <p>{feedback.pronunciation}</p>
        </section>
        
        <section className="feedback-section">
          <h2>Grammar Analysis</h2>
          <p>{feedback.grammar}</p>
        </section>
        
        <section className="feedback-section">
          <h2>Suggested Improvements</h2>
          <ul className="suggestions-list">
            {feedback.suggestions.map((suggestion, index) => (
              <li key={index}>{suggestion}</li>
            ))}
          </ul>
        </section>
      </div>
      
      <div className="feedback-actions">
        <Link to="/practice" className="btn btn-primary">
          Practice Again
        </Link>
        <Link to="/dashboard" className="btn btn-secondary">
          Return to Dashboard
        </Link>
      </div>
    </div>
  );
};

export default Feedback;