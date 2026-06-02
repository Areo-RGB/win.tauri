import React, { useState } from 'react';
import { HudPanel } from './HudPanel';
import { CyberButton } from './CyberButton';
import { GitBranch, Upload, Archive, RefreshCw, Cloud } from 'lucide-react';

export const ProjectsTab: React.FC = () => {
  const [cwd, setCwd] = useState('C:\\Users\\paul\\Documents\\.projects\\pc.tool');
  const [remoteUrl, setRemoteUrl] = useState('');
  const [output, setOutput] = useState('');

  const run = async (label: string, fn: () => Promise<any>) => {
    try {
      const res = await fn();
      setOutput(`[${label}]\n${JSON.stringify(res, null, 2)}`);
    } catch (e: any) {
      setOutput(`[${label}] ERROR\n${e?.message || String(e)}`);
    }
  };

  return (
    <HudPanel title="PROJECTS // GIT" delay={0.1}>
      <div style={{ display: 'flex', flexDirection: 'column', gap: '14px' }}>
        <div className="glass-card" style={{ padding: '18px', display: 'flex', flexDirection: 'column', gap: '12px' }}>
          <label className="hud-label">PROJECT DIRECTORY</label>
          <input className="hud-input" value={cwd} onChange={e => setCwd(e.target.value)} />
          <label className="hud-label">REMOTE URL</label>
          <input className="hud-input" value={remoteUrl} onChange={e => setRemoteUrl(e.target.value)} placeholder="https://github.com/Areo-RGB/repo.git" />
          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '8px' }}>
            <CyberButton onClick={() => run('git status', () => window.cc3API.gitStatus(cwd))}><GitBranch size={14} /> STATUS</CyberButton>
            <CyberButton onClick={() => run('git remote', () => window.cc3API.gitRemote(cwd))}><Cloud size={14} /> REMOTE</CyberButton>
            <CyberButton onClick={() => run('connect remote', () => window.cc3API.gitConnectRemote(cwd, remoteUrl))}>CONNECT REMOTE</CyberButton>
            <CyberButton onClick={() => run('create remote', () => window.cc3API.gitCreateRemote(cwd))}>CREATE REMOTE</CyberButton>
            <CyberButton onClick={() => run('fetch', () => window.cc3API.gitFetch(cwd))}><RefreshCw size={14} /> FETCH</CyberButton>
            <CyberButton onClick={() => run('stage push', () => window.cc3API.gitStagePush(cwd))}><Upload size={14} /> STAGE + PUSH</CyberButton>
            <CyberButton onClick={() => run('zip project', () => window.cc3API.zipProject(cwd))}><Archive size={14} /> ZIP</CyberButton>
          </div>
        </div>
        <pre className="terminal-output">{output || 'No project output yet.'}</pre>
      </div>
    </HudPanel>
  );
};
