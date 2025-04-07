// pages/Home.tsx - Updated with better styling
import { Link } from 'react-router-dom';
import '../styles/Home.css'; // Create this file with the CSS below

const Home = () => {
  return (
    <div className="home-page">
      <section className="hero">
        <div className="container">
          <div className="hero-content">
            <h1 className="hero-title">
              Master Languages with <span className="highlight">AI</span> Precision
            </h1>
            <p className="hero-description">
              Gengo uses advanced artificial intelligence to analyze your speech,
              provide personalized feedback, and accelerate your language learning journey.
            </p>
            <div className="hero-actions">
              <Link to="/practice" className="btn btn-primary btn-large">
                <span>Start Practice</span>
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                  <line x1="5" y1="12" x2="19" y2="12"></line>
                  <polyline points="12 5 19 12 12 19"></polyline>
                </svg>
              </Link>
              <Link to="/dashboard" className="btn btn-secondary">
                View Dashboard
              </Link>
            </div>
          </div>
          <div className="hero-visual">
            <div className="orbital-animation">
              <div className="planet"></div>
              <div className="orbit">
                <div className="satellite"></div>
              </div>
              <div className="orbit orbit-2">
                <div className="satellite satellite-2"></div>
              </div>
            </div>
          </div>
        </div>
        <div className="bg-accent-1"></div>
        <div className="bg-accent-2"></div>
      </section>
      
      <section className="features">
        <div className="container">
          <h2 className="section-title">How Gengo <span className="highlight">Works</span></h2>
          <div className="feature-grid">
            <div className="feature-card">
              <div className="feature-icon">🎙️</div>
              <h3>Record Your Speech</h3>
              <p>Speak into your microphone and Gengo will capture your audio with crystal clarity.</p>
            </div>
            
            <div className="feature-card">
              <div className="feature-icon">🔍</div>
              <h3>AI Analysis</h3>
              <p>Our powerful AI analyzes your pronunciation, grammar, and fluency in real-time.</p>
            </div>
            
            <div className="feature-card">
              <div className="feature-icon">📊</div>
              <h3>Detailed Feedback</h3>
              <p>Receive personalized suggestions and track your progress over time.</p>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
};

export default Home;