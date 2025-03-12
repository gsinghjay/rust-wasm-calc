/**
 * Navigation module for the Rust WASM Calculator
 * Handles navigation between different sections of the application
 */

/**
 * Initialize the navigation functionality
 */
export function initNavigation() {
    // Get all navigation links
    const navLinks = document.querySelectorAll('.navbar-nav .nav-link');
    
    // Get all sections
    const sections = {
        landing: document.getElementById('landing'),
        calculator: document.getElementById('calculator'),
        chatbot: document.getElementById('chatbot')
    };
    
    // Function to show a section and hide others
    function showSection(sectionId) {
        // Hide all sections
        Object.values(sections).forEach(section => {
            section.classList.add('d-none');
        });
        
        // Show the selected section
        sections[sectionId].classList.remove('d-none');
        
        // Update active nav link
        navLinks.forEach(link => {
            const href = link.getAttribute('href').substring(1); // Remove the # from href
            if (href === sectionId) {
                link.classList.add('active');
            } else {
                link.classList.remove('active');
            }
        });
        
        // Update URL hash
        window.location.hash = sectionId;
    }
    
    // Add click event listeners to nav links
    navLinks.forEach(link => {
        link.addEventListener('click', (e) => {
            e.preventDefault();
            const sectionId = link.getAttribute('href').substring(1); // Remove the # from href
            showSection(sectionId);
        });
    });
    
    // Add click event listeners to landing page buttons
    document.querySelectorAll('#landing a[href^="#"]').forEach(button => {
        button.addEventListener('click', (e) => {
            e.preventDefault();
            const sectionId = button.getAttribute('href').substring(1); // Remove the # from href
            showSection(sectionId);
        });
    });
    
    // Handle initial page load based on URL hash
    function handleInitialNavigation() {
        const hash = window.location.hash.substring(1); // Remove the # from hash
        
        if (hash && sections[hash]) {
            showSection(hash);
        } else {
            // Default to landing page
            showSection('landing');
        }
    }
    
    // Handle browser back/forward buttons
    window.addEventListener('hashchange', () => {
        const hash = window.location.hash.substring(1);
        if (hash && sections[hash]) {
            showSection(hash);
        }
    });
    
    // Initialize navigation on page load
    handleInitialNavigation();
} 