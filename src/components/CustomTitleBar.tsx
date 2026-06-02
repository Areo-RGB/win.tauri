import React from 'react';
import { Minus, Square, X } from 'lucide-react';

export const CustomTitleBar: React.FC = () => {
  return (
    <div className="title-bar" data-tauri-drag-region>
      <div className="title-bar-title" data-tauri-drag-region>CC3 // TAURI</div>
      <div className="title-bar-buttons">
        <button onClick={() => window.cc3API.minimize()}><Minus size={14} /></button>
        <button onClick={() => window.cc3API.maximize()}><Square size={12} /></button>
        <button className="close" onClick={() => window.cc3API.close()}><X size={14} /></button>
      </div>
    </div>
  );
};
