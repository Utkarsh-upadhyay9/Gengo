// Navbar.tsx - Navigation component
import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import '../styles/Navbar.css';

const Navbar: React.FC = () => {
  const location = useLocation();
  
  return (
    <nav className="navbar">
      <div className="navbar-container">
        <Link to="/" className="navbar-logo">
          Gengo<span className="logo-highlight">AI</span>
        </Link>
        
        <div className="nav-menu">
          <Link 
            to="/" 
            className={`nav-link ${location.pathname === '/' ? 'active' : ''}`}
          >
            Home
          </Link>
          <Link 
            to="/practice" 
            className={`nav-link ${location.pathname === '/practice' ? 'active' : ''}`}
          >
            Practice
          </Link>
          <Link 
            to="/dashboard" 
            className={`nav-link ${location.pathname.includes('/dashboard') ? 'active' : ''}`}
          >
            Dashboard
          </Link>
        </div>
        
        <div className="nav-user">
          {/* Add login/profile button here */}
          <Link to="/login" className="btn btn-secondary">
            Login
          </Link>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;