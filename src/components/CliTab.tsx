import React from 'react';
import { HudPanel } from './HudPanel';
import { CyberButton } from './CyberButton';
import { Terminal, Code2, Bot } from 'lucide-react';

const quickActions = [
  {
    title: 'CODex CLI',
    subtitle: 'Open Codex in project context',
    command: 'codex',
  },
  {
    title: 'Serena MCP',
    subtitle: 'Start Serena streamable-http server',
    command: 'serena',
  },
  {
    title: 'MCP Proxy',
    subtitle: 'Bridge local stdio MCP to HTTP',
    command: 'mcp-proxy',
  },
];

export const CliTab: React.FC = () => {
  const launchSerena = () => window.cc3API.launchSerena('8001');
  const launchMcpProxy = () => window.cc3API.launchMcpProxy('8000');

  return (
    <HudPanel title="CLI // CONTROL" delay={0.1}>
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(220px, 1fr))', gap: '18px' }}>
        {quickActions.map((action, idx) => (
          <div key={action.title} className="glass-card" style={{ padding: '18px', display: 'flex', flexDirection: 'column', gap: '12px' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '10px', color: 'var(--color-primary)' }}>
              {idx === 0 ? <Terminal size={18} /> : idx === 1 ? <Bot size={18} /> : <Code2 size={18} />}
              <span style={{ fontFamily: 'var(--font-display)', letterSpacing: '1px', fontSize: '12px' }}>{action.title}</span>
            </div>
            <div style={{ fontFamily: 'var(--font-mono)', fontSize: '10px', color: 'var(--color-text-muted)' }}>{action.subtitle}</div>
            <CyberButton onClick={() => {
              if (action.command === 'serena') launchSerena();
              if (action.command === 'mcp-proxy') launchMcpProxy();
            }}>
              LAUNCH
            </CyberButton>
          </div>
        ))}
      </div>
    </HudPanel>
  );
};
