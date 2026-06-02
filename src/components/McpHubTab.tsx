import React, { useEffect, useState } from 'react';
import { HudPanel } from './HudPanel';
import { CyberButton } from './CyberButton';
import { McpHubApiPanel } from './McpHubApiPanel';
import { Server, RadioTower, RefreshCw } from 'lucide-react';

export const McpHubTab: React.FC = () => {
  const [port, setPort] = useState('37373');
  const [config, setConfig] = useState('mcp.json');
  const [ngrokTarget, setNgrokTarget] = useState('37373');
  const [ngrokUrl, setNgrokUrl] = useState('');
  const [mcpConfigs, setMcpConfigs] = useState<string[]>([]);
  const [ngrokConfigs, setNgrokConfigs] = useState<string[]>([]);

  const scan = async () => {
    const res = await window.cc3API.scanConfigs();
    setMcpConfigs(res.mcpConfigs || []);
    setNgrokConfigs(res.ngrokConfigs || []);
  };

  useEffect(() => {
    scan();
  }, []);

  return (
    <HudPanel title="MCP HUB // CONTROL" delay={0.1}>
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))', gap: '18px' }}>
        <div className="glass-card" style={{ padding: '18px', display: 'flex', flexDirection: 'column', gap: '12px' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '10px', color: 'var(--color-primary)' }}>
            <Server size={18} />
            <span style={{ fontFamily: 'var(--font-display)', fontSize: '12px', letterSpacing: '1px' }}>MCP-HUB</span>
          </div>
          <input className="hud-input" value={port} onChange={e => setPort(e.target.value)} placeholder="Port" />
          <select className="hud-input" value={config} onChange={e => setConfig(e.target.value)}>
            <option value={config}>{config}</option>
            {mcpConfigs.filter(c => c !== config).map(c => <option key={c} value={c}>{c}</option>)}
          </select>
          <CyberButton onClick={() => window.cc3API.launchMcp(port, config)}>START MCP HUB</CyberButton>
          <CyberButton onClick={scan}><RefreshCw size={14} /> RESCAN CONFIGS</CyberButton>
        </div>

        <div className="glass-card" style={{ padding: '18px', display: 'flex', flexDirection: 'column', gap: '12px' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '10px', color: 'var(--color-primary)' }}>
            <RadioTower size={18} />
            <span style={{ fontFamily: 'var(--font-display)', fontSize: '12px', letterSpacing: '1px' }}>NGROK</span>
          </div>
          <input className="hud-input" value={ngrokTarget} onChange={e => setNgrokTarget(e.target.value)} placeholder="Port/address" />
          <input className="hud-input" value={ngrokUrl} onChange={e => setNgrokUrl(e.target.value)} placeholder="Reserved domain / URL" />
          <CyberButton onClick={() => window.cc3API.launchNgrok(ngrokTarget, ngrokUrl)}>START NGROK</CyberButton>
          <div style={{ fontFamily: 'var(--font-mono)', fontSize: '10px', color: 'var(--color-text-muted)' }}>
            Configs found: {ngrokConfigs.join(', ') || 'none'}
          </div>
        </div>
      </div>

      <div style={{ marginTop: '18px' }}>
        <McpHubApiPanel />
      </div>
    </HudPanel>
  );
};
