// Home.tsx - Landing page
import { Link } from 'react-router-dom';

const Home = () => {
  return (
    <div className="home-page">
      <section className="hero">
        <div className="container">
          <h1>Improve Your Language Speaking Skills with AI</h1>
          <p className="hero-description">
            Gengo uses AI to analyze your speech, provide feedback on grammar and pronunciation,
            and help you become more confident in speaking a new language.
          </p>
          <Link to="/practice" className="btn btn-primary btn-large">
            Start Practice
          </Link>
        </div>
      </section>
      
      <section className="features">
        <div className="container">
          <h2>How It Works</h2>
          <div className="feature-grid">
            <div className="feature-card">
              <div className="feature-icon">🎙️</div>
              <h3>Record Your Speech</h3>
              <p>Speak into your microphone and Gengo will capture your audio.</p>
            </div>
            
            <div className="feature-card">
              <div className="feature-icon">🔍</div>
              <h3>AI Analysis</h3>
              <p>Our AI analyzes your grammar, pronunciation, and fluency.</p>
            </div>
            
            <div className="feature-card">
              <div className="feature-icon">📊</div>
              <h3>Get Feedback</h3>
              <p>Receive detailed feedback and suggestions for improvement.</p>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
};

export default Home;