import React from 'react';

export const CyberButton: React.FC<React.ButtonHTMLAttributes<HTMLButtonElement>> = ({ children, style, ...props }) => {
  return (
    <button
      {...props}
      className={`cyber-button ${props.className || ''}`}
      style={{
        ...style,
      }}
    >
      {children}
    </button>
  );
};
