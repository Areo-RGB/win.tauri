import React, { useState, useEffect } from 'react';
import { CustomTitleBar } from './components/CustomTitleBar';
import { HudPanel } from './components/HudPanel';
import { CyberButton } from './components/CyberButton';
import { ProjectsTab } from './components/ProjectsTab';
import { ScrcpyTab } from './components/ScrcpyTab';
import { McpHubTab } from './components/McpHubTab';
import { CmdRunnerTab } from './components/CmdRunnerTab';
import { CliTab } from './components/CliTab';
import { SquareTerminal, X } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

export const App: React.FC = () => {
  const [sysUsage, setSysUsage] = useState(42);
  const [netTraffic, setNetTraffic] = useState(128.4);
  const [activeTab, setActiveTab] = useState('PROJECTS');
  const [isEventLogOpen, setIsEventLogOpen] = useState(false);

  useEffect(() => {
    const interval = setInterval(() => {
      setSysUsage(prev => Math.min(100, Math.max(0, prev + (Math.random() * 10 - 5))));
      setNetTraffic(prev => Math.max(0, prev + (Math.random() * 20 - 10)));
    }, 1500);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="hud-container">
      <CustomTitleBar />

      <div style={{
        padding: '30px',
        display: 'grid',
        gridTemplateColumns: '200px 1fr',
        gap: '24px',
        height: 'calc(100vh - 32px)',
        overflow: 'hidden',
        position: 'relative'
      }}>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
          <TabButton active={activeTab === 'PROJECTS'} onClick={() => setActiveTab('PROJECTS')}>PROJECTS</TabButton>
          <TabButton active={activeTab === 'NETWORK'} onClick={() => setActiveTab('NETWORK')}>NETWORK</TabButton>
          <TabButton active={activeTab === 'SECURITY'} onClick={() => setActiveTab('SECURITY')}>SECURITY</TabButton>
          <TabButton active={activeTab === 'MCP-HUB'} onClick={() => setActiveTab('MCP-HUB')}>MCP-HUB</TabButton>
          <TabButton active={activeTab === 'CMD RUNNER'} onClick={() => setActiveTab('CMD RUNNER')}>CMD RUNNER</TabButton>
          <TabButton active={activeTab === 'CLI'} onClick={() => setActiveTab('CLI')}>CLI</TabButton>
          <TabButton active={activeTab === 'SCRCPY'} onClick={() => setActiveTab('SCRCPY')}>SCRCPY</TabButton>
          <TabButton active={activeTab === 'SETTINGS'} onClick={() => setActiveTab('SETTINGS')}>SETTINGS</TabButton>

          <div style={{ marginTop: 'auto' }}>
            <CyberButton onClick={() => setIsEventLogOpen(!isEventLogOpen)}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <SquareTerminal size={14} />
                <span>LOGS</span>
              </div>
            </CyberButton>
          </div>
        </div>

        <div style={{ display: 'flex', flexDirection: 'column', gap: '24px', overflowY: 'auto' }}>
          {activeTab === 'PROJECTS' && <ProjectsTab />}
          {activeTab === 'SCRCPY' && <ScrcpyTab />}
          {activeTab === 'MCP-HUB' && <McpHubTab />}
          {activeTab === 'CMD RUNNER' && <CmdRunnerTab />}
          {activeTab === 'CLI' && <CliTab />}

          {activeTab !== 'PROJECTS' && activeTab !== 'SCRCPY' && activeTab !== 'MCP-HUB' && activeTab !== 'CMD RUNNER' && activeTab !== 'CLI' && (
            <HudPanel title={`${activeTab} // OFFLINE`} delay={0.1}>
              <div style={{ color: 'var(--color-text-muted)', fontFamily: 'var(--font-mono)' }}>
                Module '{activeTab}' is currently offline. Awaiting uplink...
              </div>
            </HudPanel>
          )}
        </div>

        <AnimatePresence>
          {isEventLogOpen && (
            <motion.div
              initial={{ y: 300, opacity: 0 }}
              animate={{ y: 0, opacity: 1 }}
              exit={{ y: 300, opacity: 0 }}
              transition={{ type: 'spring', damping: 25, stiffness: 200 }}
              style={{
                position: 'absolute',
                bottom: 0, left: '224px', right: '30px',
                height: '250px',
                zIndex: 40
              }}
            >
              <HudPanel title="EVENT_LOG // LIVE" delay={0} className="event-log-panel">
                <button 
                  onClick={() => setIsEventLogOpen(false)}
                  style={{ position: 'absolute', top: '20px', right: '20px', background: 'none', border: 'none', color: 'var(--color-primary)', cursor: 'pointer' }}
                >
                  <X size={16} />
                </button>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '8px', overflowY: 'auto', height: '100%', paddingRight: '8px' }}>
                  {Array.from({ length: 15 }).map((_, i) => (
                    <div key={i} style={{ fontFamily: 'var(--font-mono)', fontSize: '10px', color: 'var(--color-text-muted)' }}>
                      [{new Date(Date.now() - i * 142000).toISOString().split('T')[1].substring(0, 8)}]
                      <span style={{ color: i % 4 === 0 ? 'var(--color-secondary)' : 'var(--color-text-main)', marginLeft: '8px' }}>
                        {i % 4 === 0 ? 'WARN' : 'INFO'} : SYS_PROCESS_{9000 - i * 3}
                      </span>
                    </div>
                  ))}
                </div>
              </HudPanel>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
};

const TabButton = ({ children, active, onClick }: { children: React.ReactNode, active: boolean, onClick: () => void }) => (
  <button
    onClick={onClick}
    style={{
      background: active ? 'rgba(0, 243, 255, 0.15)' : 'transparent',
      border: 'none',
      borderLeft: active ? '4px solid var(--color-primary)' : '1px solid rgba(0, 243, 255, 0.2)',
      color: active ? 'var(--color-primary)' : 'var(--color-text-muted)',
      fontFamily: 'var(--font-display)',
      fontSize: '10px',
      letterSpacing: '2px',
      textAlign: 'left',
      padding: '16px 20px',
      cursor: 'pointer',
      transition: 'all 0.2s',
      textShadow: active ? 'var(--hud-glow)' : 'none',
      boxShadow: active ? 'inset 20px 0 20px -20px rgba(0,243,255,0.5)' : 'none'
    }}
  >
    {children}
  </button>
);
