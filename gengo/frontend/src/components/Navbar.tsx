// Navbar.tsx - Navigation component
import React, { useState, useEffect } from 'react';
import { Link, useLocation } from 'react-router-dom';
import '../styles/Navbar.css';
import Logo from './Logo';

const Navbar: React.FC = () => {
  const location = useLocation();
  const [menuOpen, setMenuOpen] = useState(false);
  const [scrolled, setScrolled] = useState(false);
  
  // Track scrolling for navbar appearance
  useEffect(() => {
    const handleScroll = () => {
      if (window.scrollY > 20) {
        setScrolled(true);
      } else {
        setScrolled(false);
      }
    };
    
    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);
  
  // Close mobile menu when changing routes
  useEffect(() => {
    setMenuOpen(false);
  }, [location.pathname]);
  
  return (
    <nav className={`navbar ${scrolled ? 'scrolled' : ''}`}>
      <div className="navbar-container">
        <Link to="/" className="navbar-logo">
          <Logo />
        </Link>
        
        {/* Mobile menu toggle */}
        <div 
          className={`menu-toggle ${menuOpen ? 'active' : ''}`} 
          onClick={() => setMenuOpen(!menuOpen)}
        >
          <span></span>
          <span></span>
          <span></span>
        </div>
        
        {/* Navigation links */}
        <div className={`nav-menu ${menuOpen ? 'active' : ''}`}>
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
          <Link 
            to="/feedback" 
            className={`nav-link ${location.pathname.includes('/feedback') ? 'active' : ''}`}
          >
            Feedback
          </Link>
        </div>
        
        <div className="nav-user">
          <Link to="/login" className="btn btn-secondary btn-login">
            Login
          </Link>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;