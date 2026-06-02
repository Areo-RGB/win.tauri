import React, { useState } from 'react';
import { CyberButton } from './CyberButton';
import { X, Plus, Trash2 } from 'lucide-react';

interface Props {
  initialArgs: string[];
  onClose: () => void;
  onSave: (args: string[]) => void;
}

export const FlagsConfigModal: React.FC<Props> = ({ initialArgs, onClose, onSave }) => {
  const [args, setArgs] = useState<string[]>(initialArgs.length ? initialArgs : ['']);

  const updateArg = (idx: number, value: string) => {
    setArgs(prev => prev.map((arg, i) => i === idx ? value : arg));
  };

  const addArg = () => setArgs(prev => [...prev, '']);
  const removeArg = (idx: number) => setArgs(prev => prev.filter((_, i) => i !== idx));

  return (
    <div className="modal-backdrop">
      <div className="modal-panel">
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '16px' }}>
          <div style={{ fontFamily: 'var(--font-display)', color: 'var(--color-primary)', letterSpacing: '1px' }}>SCRCPY FLAGS</div>
          <button className="icon-button" onClick={onClose}><X size={16} /></button>
        </div>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '10px', maxHeight: '360px', overflowY: 'auto' }}>
          {args.map((arg, idx) => (
            <div key={idx} style={{ display: 'flex', gap: '8px' }}>
              <input className="hud-input" value={arg} onChange={e => updateArg(idx, e.target.value)} placeholder="--turn-screen-off" />
              <button className="icon-button" onClick={() => removeArg(idx)}><Trash2 size={14} /></button>
            </div>
          ))}
        </div>
        <div style={{ display: 'flex', gap: '10px', marginTop: '18px' }}>
          <CyberButton onClick={addArg}><Plus size={14} /> ADD</CyberButton>
          <CyberButton onClick={() => onSave(args.map(a => a.trim()).filter(Boolean))}>SAVE</CyberButton>
        </div>
      </div>
    </div>
  );
};
