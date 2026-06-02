import React, { useEffect, useState } from 'react';
import { HudPanel } from './HudPanel';
import { CyberButton } from './CyberButton';
import { FlagsConfigModal } from './FlagsConfigModal';
import { Smartphone, RefreshCw, Camera, PlugZap, SlidersHorizontal } from 'lucide-react';

export const ScrcpyTab: React.FC = () => {
  const [devices, setDevices] = useState<{ id: string; state: string }[]>([]);
  const [selected, setSelected] = useState('');
  const [ip, setIp] = useState('');
  const [args, setArgs] = useState<string[]>(['--max-size', '1280']);
  const [modal, setModal] = useState(false);
  const [output, setOutput] = useState('');

  const refresh = async () => {
    const list = await window.cc3API.getDevices();
    setDevices(list);
    if (!selected && list[0]) setSelected(list[0].id);
  };

  useEffect(() => {
    refresh();
  }, []);

  const connect = async () => {
    const res = await window.cc3API.adbConnect(ip);
    setOutput(JSON.stringify(res, null, 2));
    refresh();
  };

  const screenshot = async () => {
    const res = await window.cc3API.adbScreenshot(selected);
    setOutput(JSON.stringify(res, null, 2));
  };

  return (
    <HudPanel title="SCRCPY // DEVICE CONTROL" delay={0.1}>
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))', gap: '18px' }}>
        <div className="glass-card" style={{ padding: '18px', display: 'flex', flexDirection: 'column', gap: '12px' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '10px', color: 'var(--color-primary)' }}>
            <Smartphone size={18} />
            <span style={{ fontFamily: 'var(--font-display)', fontSize: '12px', letterSpacing: '1px' }}>ADB DEVICES</span>
          </div>
          <select className="hud-input" value={selected} onChange={e => setSelected(e.target.value)}>
            <option value="">Default device</option>
            {devices.map(d => <option key={d.id} value={d.id}>{d.id} [{d.state}]</option>)}
          </select>
          <div style={{ display: 'flex', gap: '8px' }}>
            <CyberButton onClick={refresh}><RefreshCw size={14} /> REFRESH</CyberButton>
            <CyberButton onClick={() => window.cc3API.launchScrcpy(selected || undefined, args)}>LAUNCH SCRCPY</CyberButton>
          </div>
          <CyberButton onClick={() => setModal(true)}><SlidersHorizontal size={14} /> FLAGS</CyberButton>
        </div>

        <div className="glass-card" style={{ padding: '18px', display: 'flex', flexDirection: 'column', gap: '12px' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '10px', color: 'var(--color-primary)' }}>
            <PlugZap size={18} />
            <span style={{ fontFamily: 'var(--font-display)', fontSize: '12px', letterSpacing: '1px' }}>WIRELESS ADB</span>
          </div>
          <input className="hud-input" value={ip} onChange={e => setIp(e.target.value)} placeholder="192.168.1.20:5555" />
          <CyberButton onClick={connect}>ADB CONNECT</CyberButton>
          <CyberButton onClick={screenshot}><Camera size={14} /> SCREENSHOT</CyberButton>
        </div>
      </div>

      <pre className="terminal-output">{output || 'No device output yet.'}</pre>

      {modal && <FlagsConfigModal initialArgs={args} onClose={() => setModal(false)} onSave={(next) => { setArgs(next); setModal(false); }} />}
    </HudPanel>
  );
};
