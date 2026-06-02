import React, { useState } from 'react';
import { CyberButton } from './CyberButton';
import { RefreshCw, Play, Square, Ban, HeartPulse } from 'lucide-react';

export const McpHubApiPanel: React.FC = () => {
  const [baseUrl, setBaseUrl] = useState('http://localhost:37373');
  const [serverName, setServerName] = useState('desktop-commander');
  const [output, setOutput] = useState('');

  const run = async (label: string, fn: () => Promise<any>) => {
    try {
      const result = await fn();
      setOutput(`[${label}]\n${JSON.stringify(result, null, 2)}`);
    } catch (e: any) {
      setOutput(`[${label}] ERROR\n${e?.message || String(e)}`);
    }
  };

  return (
    <div className="glass-card" style={{ padding: '18px', display: 'flex', flexDirection: 'column', gap: '12px' }}>
      <div style={{ fontFamily: 'var(--font-display)', color: 'var(--color-primary)', fontSize: '12px', letterSpacing: '1px' }}>MCP HUB API</div>
      <input className="hud-input" value={baseUrl} onChange={e => setBaseUrl(e.target.value)} />
      <input className="hud-input" value={serverName} onChange={e => setServerName(e.target.value)} />
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '8px' }}>
        <CyberButton onClick={() => run('health', () => window.cc3API.mcpApiHealth(baseUrl))}><HeartPulse size={14} /> HEALTH</CyberButton>
        <CyberButton onClick={() => run('refresh all', () => window.cc3API.mcpApiRefreshAll(baseUrl))}><RefreshCw size={14} /> REFRESH ALL</CyberButton>
        <CyberButton onClick={() => run('start server', () => window.cc3API.mcpApiServerStart(baseUrl, serverName))}><Play size={14} /> START</CyberButton>
        <CyberButton onClick={() => run('stop server', () => window.cc3API.mcpApiServerStop(baseUrl, serverName))}><Square size={14} /> STOP</CyberButton>
        <CyberButton onClick={() => run('disable server', () => window.cc3API.mcpApiServerDisable(baseUrl, serverName))}><Ban size={14} /> DISABLE</CyberButton>
        <CyberButton onClick={() => run('refresh server', () => window.cc3API.mcpApiServerRefresh(baseUrl, serverName))}><RefreshCw size={14} /> REFRESH SERVER</CyberButton>
        <CyberButton onClick={() => run('restart hub', () => window.cc3API.mcpApiRestart(baseUrl))}>RESTART HUB</CyberButton>
      </div>
      <pre className="terminal-output">{output || 'No API output yet.'}</pre>
    </div>
  );
};
