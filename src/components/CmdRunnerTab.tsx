import React, { useState } from 'react';
import { HudPanel } from './HudPanel';
import { CyberButton } from './CyberButton';
import { Zap, RadioTower, Server, Skull } from 'lucide-react';

export const CmdRunnerTab: React.FC = () => {
  const [killPort, setKillPort] = useState('37373');
  const [proxyPort, setProxyPort] = useState('8000');
  const [serenaPort, setSerenaPort] = useState('8001');

  return (
    <HudPanel title="CMD RUNNER // WINDOWS" delay={0.1}>
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(260px, 1fr))', gap: '18px' }}>
        <RunnerCard icon={<Skull size={18} />} title="KILL PORT" desc="Runs npx kill-port for the selected port.">
          <input className="hud-input" value={killPort} onChange={e => setKillPort(e.target.value)} />
          <CyberButton onClick={() => window.cc3API.launchKillPort(killPort)}>KILL</CyberButton>
        </RunnerCard>

        <RunnerCard icon={<Server size={18} />} title="MCP PROXY" desc="Starts mcp-proxy for DesktopCommanderMCP.">
          <input className="hud-input" value={proxyPort} onChange={e => setProxyPort(e.target.value)} />
          <CyberButton onClick={() => window.cc3API.launchMcpProxy(proxyPort)}>START</CyberButton>
        </RunnerCard>

        <RunnerCard icon={<RadioTower size={18} />} title="SERENA" desc="Starts Serena in streamable-http mode.">
          <input className="hud-input" value={serenaPort} onChange={e => setSerenaPort(e.target.value)} />
          <CyberButton onClick={() => window.cc3API.launchSerena(serenaPort)}>START</CyberButton>
        </RunnerCard>
      </div>
    </HudPanel>
  );
};

const RunnerCard = ({ icon, title, desc, children }: { icon: React.ReactNode, title: string, desc: string, children: React.ReactNode }) => (
  <div className="glass-card" style={{ padding: '18px', display: 'flex', flexDirection: 'column', gap: '12px' }}>
    <div style={{ display: 'flex', alignItems: 'center', gap: '10px', color: 'var(--color-primary)' }}>
      {icon}
      <span style={{ fontFamily: 'var(--font-display)', fontSize: '12px', letterSpacing: '1px' }}>{title}</span>
    </div>
    <div style={{ color: 'var(--color-text-muted)', fontFamily: 'var(--font-mono)', fontSize: '10px' }}>{desc}</div>
    {children}
  </div>
);
