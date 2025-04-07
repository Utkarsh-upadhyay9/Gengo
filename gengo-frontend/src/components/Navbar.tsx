// Navbar.tsx - Navigation component
import { Link, useLocation } from 'react-router-dom';

const Navbar = () => {
  const location = useLocation();
  
  return (
    <nav className="navbar">
      <div className="navbar-container">
        <Link to="/" className="navbar-logo">
          Gengo <span className="logo-highlight">AI</span>
        </Link>
        
        <div className="nav-menu">
          <Link 
            to="/" 
            className={`nav-link ${location.pathname === '/' ? 'active' : ''}`}
          >
            Home
          </Link>
          <Link 
            to="/dashboard" 
            className={`nav-link ${location.pathname === '/dashboard' ? 'active' : ''}`}
          >
            Dashboard
          </Link>
          <Link 
            to="/practice" 
            className={`nav-link ${location.pathname === '/practice' ? 'active' : ''}`}
          >
            Practice
          </Link>
        </div>
        
        <div className="nav-user">
          <button className="btn btn-secondary">Sign In</button>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;