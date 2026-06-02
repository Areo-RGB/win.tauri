import React from 'react';
import { motion } from 'framer-motion';

export const HudPanel: React.FC<{ title: string; delay?: number; className?: string; children: React.ReactNode }> = ({ title, delay = 0, className = '', children }) => {
  return (
    <motion.div
      initial={{ opacity: 0, y: 12 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.25, delay }}
      className={`hud-panel ${className}`}
    >
      <div className="hud-panel-title">{title}</div>
      <div className="hud-panel-content">{children}</div>
    </motion.div>
  );
};
