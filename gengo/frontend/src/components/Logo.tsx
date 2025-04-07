import React from 'react';

interface LogoProps {
  className?: string;
}

const Logo: React.FC<LogoProps> = ({ className = '' }) => {
  return (
    <div className={`logo-container ${className}`}>
      <svg 
        width="32" 
        height="32" 
        viewBox="0 0 32 32" 
        fill="none" 
        xmlns="http://www.w3.org/2000/svg"
        className="logo-icon"
      >
        <path 
          d="M6 8C6 6.89543 6.89543 6 8 6H24C25.1046 6 26 6.89543 26 8V24C26 25.1046 25.1046 26 24 26H8C6.89543 26 6 25.1046 6 24V8Z" 
          fill="url(#gradient)" 
        />
        <path 
          d="M10 12L14 20M14 12L10 20" 
          stroke="white" 
          strokeWidth="2" 
          strokeLinecap="round" 
        />
        <path 
          d="M18 12L22 20M22 12L18 20" 
          stroke="white" 
          strokeWidth="2" 
          strokeLinecap="round" 
        />
        <defs>
          <linearGradient id="gradient" x1="6" y1="6" x2="26" y2="26" gradientUnits="userSpaceOnUse">
            <stop stopColor="#4361EE" />
            <stop offset="1" stopColor="#F72585" />
          </linearGradient>
        </defs>
      </svg>
      <span className="logo-text">gengo<span className="logo-highlight">.live</span></span>
    </div>
  );
};

export default Logo;