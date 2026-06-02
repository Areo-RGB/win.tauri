import React from 'react';

export const DataReadout: React.FC<{ label: string; value: string | number; unit?: string }> = ({ label, value, unit }) => {
  return (
    <div className="data-readout">
      <div className="data-label">{label}</div>
      <div className="data-value">{value}<span>{unit}</span></div>
    </div>
  );
};
