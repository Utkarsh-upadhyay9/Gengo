// AudioRecorder.tsx - Audio recording component
import { useState, useRef, useEffect } from 'react';
import '../styles/AudioRecorder.css';

interface AudioRecorderProps {
  onRecordingComplete?: (blob: Blob) => void;
}

const AudioRecorder: React.FC<AudioRecorderProps> = ({ onRecordingComplete }) => {
  const [recording, setRecording] = useState(false);
  const [audioUrl, setAudioUrl] = useState<string | null>(null);
  const [visualizationData, setVisualizationData] = useState([10, 20, 15, 30, 25, 40, 35, 50, 45]);
  
  const mediaRecorderRef = useRef<MediaRecorder | null>(null);
  const audioChunksRef = useRef<Blob[]>([]);
  const intervalRef = useRef<number | null>(null);
  
  useEffect(() => {
    return () => {
      if (intervalRef.current) {
        window.clearInterval(intervalRef.current);
      }
    };
  }, []);
  
  const startRecording = async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      const mediaRecorder = new MediaRecorder(stream);
      mediaRecorderRef.current = mediaRecorder;
      audioChunksRef.current = [];
      
      mediaRecorder.ondataavailable = (event) => {
        audioChunksRef.current.push(event.data);
      };
      
      mediaRecorder.onstop = () => {
        const audioBlob = new Blob(audioChunksRef.current, { type: 'audio/mp3' });
        const url = URL.createObjectURL(audioBlob);
        setAudioUrl(url);
        
        if (onRecordingComplete) {
          onRecordingComplete(audioBlob);
        }
      };
      
      mediaRecorder.start();
      setRecording(true);
      
      // Visual animation during recording
      intervalRef.current = window.setInterval(() => {
        setVisualizationData(prevData => 
          prevData.map(() => {
            const random = Math.random() * 40;
            return Math.round(10 + random);
          })
        );
      }, 100);
    } catch (error) {
      console.error("Error accessing microphone:", error);
    }
  };
  
  const stopRecording = () => {
    if (mediaRecorderRef.current) {
      mediaRecorderRef.current.stop();
      mediaRecorderRef.current = null;
      setRecording(false);
      
      if (intervalRef.current) {
        window.clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
    }
  };
  
  return (
    <div className="recorder-container card">
      <div className="audio-visualizer">
        {visualizationData.map((height, index) => {
          const adjustedHeight = recording 
            ? height * (1 + Math.random() * 1.2)
            : height;
            
          return (
            <div 
              key={index}
              className="visualizer-bar"
              style={{
                height: `${adjustedHeight}px`,
                animationDelay: `${index * 100}ms`
              }}
            />
          );
        })}
      </div>
      
      <div className="recorder-controls">
        {recording ? (
          <div className="btn-group">
            <button className="btn btn-stop" onClick={stopRecording}>
              <span className="btn-icon">⏹</span>
              <span className="btn-text">Stop</span>
            </button>
          </div>
        ) : audioUrl ? (
          <div className="btn-group">
            <button className="btn btn-secondary" onClick={() => setAudioUrl(null)}>
              <span className="btn-text">Record Again</span>
            </button>
            <button className="btn btn-primary">
              <span className="btn-text">Submit</span>
            </button>
          </div>
        ) : (
          <div className="btn-group">
            <button className="btn btn-record" onClick={startRecording}>
              <span className="btn-icon">⏺</span>
              <span className="btn-text">Start Recording</span>
            </button>
          </div>
        )}
      </div>
      
      {audioUrl && (
        <div className="audio-player fade-in">
          <audio controls src={audioUrl}></audio>
        </div>
      )}
    </div>
  );
};

export default AudioRecorder;