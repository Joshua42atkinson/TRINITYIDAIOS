import { useState, useEffect, useRef, useCallback } from 'react';

/**
 * useCreative — hook for ART Studio state management.
 * Polls sidecar status + logs, exposes generation functions, manages asset gallery.
 */
export function useCreative() {
  const [status, setStatus] = useState({ comfyui: null, musicgpt: null, hunyuan3d: null });
  const [logs, setLogs] = useState([]);
  const [assets, setAssets] = useState([]);
  const [settings, setSettings] = useState({ visual_style: 'steampunk', music_style: 'orchestral', creative_enabled: true });
  const [generating, setGenerating] = useState({ image: false, music: false, video: false, mesh3d: false });
  const logIdsRef = useRef(new Set());

  // Fetch generated assets from the workspace
  const fetchAssets = useCallback(async () => {
    try {
      const res = await fetch('/api/creative/assets');
      if (res.ok) {
        const data = await res.json();
        if (data?.assets) setAssets(data.assets);
      }
    } catch (_) {}
  }, []);

  // Fetch assets on mount
  useEffect(() => {
    fetchAssets();
  }, [fetchAssets]);

  // Poll sidecar status every 30s (lazy — skips first tick)
  useEffect(() => {
    const poll = async () => {
      try {
        const res = await fetch('/api/creative/status');
        if (res.ok) setStatus(await res.json());
      } catch (_) {}
    };
    const delay = setTimeout(poll, 500);
    const id = setInterval(poll, 30000);
    return () => { clearTimeout(delay); clearInterval(id); };
  }, []);

  // Poll logs every 5s (lazy — skips first tick)
  useEffect(() => {
    const poll = async () => {
      try {
        const res = await fetch('/api/creative/logs');
        if (!res.ok) return;
        const data = await res.json();
        if (data?.logs) {
          const newLogs = [];
          data.logs.forEach((log) => {
            const logId = `${log.timestamp}-${log.message}`;
            if (!logIdsRef.current.has(logId)) {
              logIdsRef.current.add(logId);
              newLogs.push({ ...log, id: logId });
            }
          });
          if (newLogs.length > 0) {
            setLogs((prev) => [...newLogs, ...prev].slice(0, 100));
          }
        }
      } catch (_) {}
    };
    const delay = setTimeout(poll, 800);
    const id = setInterval(poll, 5000);
    return () => { clearTimeout(delay); clearInterval(id); };
  }, []);

  // Fetch settings on mount
  useEffect(() => {
    (async () => {
      try {
        const res = await fetch('/api/creative/settings');
        if (res.ok) setSettings(await res.json());
      } catch (_) {}
    })();
  }, []);

  const addLocalLog = useCallback((tag, message) => {
    const now = new Date().toISOString();
    const logId = `${now}-${message}`;
    logIdsRef.current.add(logId);
    setLogs((prev) => [{ id: logId, tag, message, timestamp: now }, ...prev].slice(0, 100));
  }, []);

  const generateImage = useCallback(async (prompt, style, width = 1024, height = 1024) => {
    setGenerating((g) => ({ ...g, image: true }));
    addLocalLog('SYSTEM', `Generating image: "${prompt}"…`);
    try {
      const res = await fetch('/api/creative/image', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt, style, width, height }),
      });
      const data = await res.json();
      if (data.success) {
        addLocalLog('SUCCESS', `Image saved: ${data.image_path || 'generated'} (${data.generation_time_ms}ms)`);
        fetchAssets();
      } else {
        addLocalLog('ERROR', data.message || 'Image generation failed');
      }
      return data;
    } catch (e) {
      addLocalLog('ERROR', `Image error: ${e.message}`);
      return { success: false, message: e.message };
    } finally {
      setGenerating((g) => ({ ...g, image: false }));
    }
  }, [addLocalLog, fetchAssets]);

  const generateMusic = useCallback(async (mood, style, duration = 60) => {
    setGenerating((g) => ({ ...g, music: true }));
    addLocalLog('SYSTEM', `Composing music: "${mood}" (${style}, ${duration}s)…`);
    try {
      const res = await fetch('/api/creative/music', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ mood, style, duration_secs: duration }),
      });
      const data = await res.json();
      if (data.success) {
        addLocalLog('SUCCESS', `Music saved: ${data.audio_path || 'generated'} (${data.generation_time_ms}ms)`);
        fetchAssets();
      } else {
        addLocalLog('ERROR', data.message || 'Music generation failed');
      }
      return data;
    } catch (e) {
      addLocalLog('ERROR', `Music error: ${e.message}`);
      return { success: false, message: e.message };
    } finally {
      setGenerating((g) => ({ ...g, music: false }));
    }
  }, [addLocalLog, fetchAssets]);

  const generateVideo = useCallback(async (prompt, duration = 4) => {
    setGenerating((g) => ({ ...g, video: true }));
    addLocalLog('SYSTEM', `Generating video: "${prompt}" (${duration}s)…`);
    try {
      const res = await fetch('/api/creative/video', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt, duration_secs: duration }),
      });
      const data = await res.json();
      if (data.success) {
        addLocalLog('SUCCESS', `Video saved: ${data.video_path || 'generated'} (${data.generation_time_ms}ms)`);
        fetchAssets();
      } else {
        addLocalLog('ERROR', data.message || 'Video generation failed');
      }
      return data;
    } catch (e) {
      addLocalLog('ERROR', `Video error: ${e.message}`);
      return { success: false, message: e.message };
    } finally {
      setGenerating((g) => ({ ...g, video: false }));
    }
  }, [addLocalLog, fetchAssets]);

  const generate3DMesh = useCallback(async (prompt, imageBase64) => {
    setGenerating((g) => ({ ...g, mesh3d: true }));
    addLocalLog('SYSTEM', `Generating 3D mesh: "${prompt}"…`);
    try {
      const res = await fetch('/api/creative/mesh3d', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt, image_base64: imageBase64 }),
      });
      const data = await res.json();
      if (data.success) {
        addLocalLog('SUCCESS', `Mesh saved: ${data.mesh_path || 'generated'} (${data.generation_time_ms}ms)`);
        fetchAssets();
      } else {
        addLocalLog('ERROR', data.message || '3D mesh generation failed');
      }
      return data;
    } catch (e) {
      addLocalLog('ERROR', `Mesh error: ${e.message}`);
      return { success: false, message: e.message };
    } finally {
      setGenerating((g) => ({ ...g, mesh3d: false }));
    }
  }, [addLocalLog, fetchAssets]);

  return {
    status,
    logs,
    assets,
    settings,
    generating,
    generateImage,
    generateMusic,
    generateVideo,
    generate3DMesh,
    fetchAssets,
    addLocalLog,
  };
}
