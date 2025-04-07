// Practice.tsx - Practice page with audio recorder
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import AudioRecorder from '../components/AudioRecorder';
import { api } from '../services/api';
import '../styles/Practice.css';

const Practice = () => {
  const navigate = useNavigate();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [recordingBlob, setRecordingBlob] = useState<Blob | null>(null);
  
  const handleRecordingComplete = (blob: Blob) => {
    setRecordingBlob(blob);
  };
  
  const handleSubmit = async () => {
    if (!recordingBlob) return;
    
    try {
      setIsSubmitting(true);
      const { sessionId } = await api.submitRecording(recordingBlob);
      navigate(`/practice/${sessionId}/feedback`);
    } catch (error) {
      console.error("Error submitting recording:", error);
      alert("Failed to submit recording. Please try again.");
    } finally {
      setIsSubmitting(false);
    }
  };
  
  return (
    <div className="practice-container">
      <h1>Practice Speaking</h1>
      
      <div className="practice-prompt">
        <h2>Read the following text:</h2>
        <div className="prompt-text">
          "Good morning! How are you doing today? I'm learning a new language."
        </div>
      </div>
      
      <AudioRecorder onRecordingComplete={handleRecordingComplete} />
      
      {recordingBlob && (
        <div className="submit-section">
          <button 
            className="submit-button"
            onClick={handleSubmit}
            disabled={isSubmitting}
          >
            {isSubmitting ? "Submitting..." : "Submit Recording"}
          </button>
        </div>
      )}
    </div>
  );
};

export default Practice;