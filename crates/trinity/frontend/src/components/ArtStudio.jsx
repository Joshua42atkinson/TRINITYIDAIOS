import React, { useState } from 'react';
import { useCreative } from '../hooks/useCreative';

/* ─── Style selector options ─── */
const VISUAL_STYLES = ['steampunk', 'cyberpunk', 'fantasy', 'minimalist', 'retro', 'noir'];
const MUSIC_STYLES = ['orchestral', 'lofi', 'electronic', 'jazz', 'ambient', 'classical'];

/* ─── Beast Logger tag color helper ─── */
function tagClass(tag) {
  const t = (tag || '').toUpperCase();
  if (['SUCCESS', 'COMPLETE', 'DONE'].includes(t)) return 'beast-tag--success';
  if (['ERROR', 'FAILED', 'CRITICAL'].includes(t)) return 'beast-tag--error';
  if (t === 'COMFYUI') return 'beast-tag--comfyui';
  if (t === 'ACE_STEP') return 'beast-tag--ace';
  if (t === 'AVATAR') return 'beast-tag--avatar';
  return '';
}

function fmtTime(ts) {
  try {
    return new Date(ts).toLocaleTimeString([], { hour12: false, hour: '2-digit', minute: '2-digit', second: '2-digit' });
  } catch { return '—'; }
}

function fmtSize(bytes) {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function typeIcon(type) {
  switch (type) {
    case 'image': return '🖼️';
    case 'video': return '🎬';
    case 'audio': return '🎵';
    case 'mesh':  return '🎲';
    default:      return '📄';
  }
}

/* ─── Sidecar Status Badge ─── */
function StatusBadge({ label, icon, sidecar }) {
  const running = sidecar?.running;
  return (
    <div 
      className={`art-status-badge ${running ? 'art-status-badge--on' : ''}`}
      title={sidecar?.message || 'Checking status...'}
    >
      <span className="art-status-dot" />
      <span>{icon}</span>
      <span>{label}</span>
    </div>
  );
}

/* ─── Generation Card (one per pipeline) ─── */
function GenerateCard({ icon, title, fields, onGenerate, busy }) {
  const [values, setValues] = useState({});
  const set = (k, v) => setValues((p) => ({ ...p, [k]: v }));

  const handleSubmit = (e) => {
    e.preventDefault();
    onGenerate(values);
  };

  return (
    <form className="art-generate-card card" onSubmit={handleSubmit}>
      <div className="art-generate-card__header">
        <span className="art-generate-card__icon">{icon}</span>
        <span className="art-generate-card__title">{title}</span>
      </div>
      {fields.map((f) => (
        <div key={f.name} className="art-generate-card__field">
          {f.type === 'select' ? (
            <select
              className="art-input"
              value={values[f.name] || f.default || ''}
              onChange={(e) => set(f.name, e.target.value)}
            >
              {f.options.map((o) => (
                <option key={o} value={o}>{o}</option>
              ))}
            </select>
          ) : (
            <input
              className="art-input"
              type={f.type || 'text'}
              placeholder={f.placeholder}
              value={values[f.name] || ''}
              onChange={(e) => set(f.name, e.target.value)}
            />
          )}
        </div>
      ))}
      <button type="submit" className="chat-send art-generate-btn" disabled={busy}>
        {busy ? 'GENERATING…' : 'GENERATE'}
      </button>
    </form>
  );
}

/* ─── Asset Card ─── */
function AssetCard({ asset, onPreview }) {
  const isImage = asset.asset_type === 'image';
  const isVideo = asset.asset_type === 'video';
  const isAudio = asset.asset_type === 'audio';

  return (
    <div
      className={`art-asset-card ${isImage || isVideo ? 'art-asset-card--clickable' : ''}`}
      onClick={() => (isImage || isVideo) && onPreview(asset)}
    >
      {isImage && (
        <img
          className="art-asset-card__thumb"
          src={asset.url}
          alt={asset.filename}
          loading="lazy"
        />
      )}
      {isVideo && (
        <div className="art-asset-card__video-thumb">
          <video src={asset.url} muted preload="metadata" />
          <div className="art-asset-card__play-badge">▶</div>
        </div>
      )}
      {isAudio && (
        <div className="art-asset-card__audio">
          <audio controls preload="metadata" src={asset.url} />
        </div>
      )}
      {!isImage && !isVideo && !isAudio && (
        <div className="art-asset-card__icon-thumb">
          <span>{typeIcon(asset.asset_type)}</span>
        </div>
      )}
      <div className="art-asset-card__meta">
        <span className="art-asset-card__type-badge">{typeIcon(asset.asset_type)}</span>
        <span className="art-asset-card__name" title={asset.filename}>
          {asset.filename.length > 24 ? asset.filename.slice(0, 22) + '…' : asset.filename}
        </span>
        <span className="art-asset-card__size">{fmtSize(asset.size_bytes)}</span>
      </div>
    </div>
  );
}

/* ─── Preview Modal ─── */
function PreviewModal({ asset, onClose }) {
  if (!asset) return null;
  const isImage = asset.asset_type === 'image';
  const isVideo = asset.asset_type === 'video';

  return (
    <div className="art-preview-modal" onClick={onClose}>
      <div className="art-preview-modal__content" onClick={(e) => e.stopPropagation()}>
        <button className="art-preview-modal__close" onClick={onClose}>✕</button>
        {isImage && <img src={asset.url} alt={asset.filename} />}
        {isVideo && <video src={asset.url} controls autoPlay />}
        <div className="art-preview-modal__info">
          <span>{asset.filename}</span>
          <span>{fmtSize(asset.size_bytes)}</span>
          <span>{fmtTime(asset.created_at)}</span>
        </div>
      </div>
    </div>
  );
}

/* ═══════════════════════════════════════════════
   ART STUDIO — Main Component
   ═══════════════════════════════════════════════ */
export default function ArtStudio() {
  const {
    status,
    logs,
    assets,
    generating,
    generateImage,
    generateMusic,
    generateVideo,
    generate3DMesh,
    fetchAssets,
  } = useCreative();

  const [previewAsset, setPreviewAsset] = useState(null);

  return (
    <div className="art-studio">
      {/* ── Status Bar ── */}
      <div className="art-status-bar">
        <div className="art-status-bar__title">✦ AI · FUN (ART Studio)</div>
        <div className="art-status-bar__badges">
          <StatusBadge label="ComfyUI" icon="🖼️" sidecar={status.comfyui} />
          <StatusBadge label="MusicGPT" icon="🎵" sidecar={status.musicgpt} />
          <StatusBadge label="Hunyuan3D" icon="🎲" sidecar={status.hunyuan3d} />
        </div>
      </div>

      {/* ── Two-column layout ── */}
      <div className="art-layout">
        {/* Left: Tools + Logger */}
        <div className="art-tools">
          <GenerateCard
            icon="🖼️"
            title="Image"
            busy={generating.image}
            fields={[
              { name: 'prompt', placeholder: 'What should Trinity imagine?', type: 'text' },
              { name: 'style', type: 'select', options: VISUAL_STYLES, default: 'steampunk' },
            ]}
            onGenerate={(v) => generateImage(v.prompt, v.style)}
          />

          <GenerateCard
            icon="🎵"
            title="Music"
            busy={generating.music}
            fields={[
              { name: 'mood', placeholder: 'Mood or tempo…', type: 'text' },
              { name: 'style', type: 'select', options: MUSIC_STYLES, default: 'orchestral' },
            ]}
            onGenerate={(v) => generateMusic(v.mood, v.style)}
          />

          <GenerateCard
            icon="🎬"
            title="Video"
            busy={generating.video}
            fields={[
              { name: 'prompt', placeholder: 'Describe the scene…', type: 'text' },
            ]}
            onGenerate={(v) => generateVideo(v.prompt)}
          />

          <GenerateCard
            icon="🎲"
            title="3D Mesh"
            busy={generating.mesh3d}
            fields={[
              { name: 'prompt', placeholder: 'Describe the object…', type: 'text' },
            ]}
            onGenerate={(v) => generate3DMesh(v.prompt)}
          />

          {/* Beast Logger */}
          <div className="art-logger card">
            <div className="card-header">SIDECAR LOG</div>
            <div className="beast-logger" id="beast-logger">
              {logs.length === 0 && (
                <div className="beast-entry">
                  <span className="beast-time">—</span>
                  <span className="beast-tag">SYSTEM</span>
                  <span className="beast-msg">Waiting for creative activity…</span>
                </div>
              )}
              {logs.map((log) => (
                <div key={log.id} className="beast-entry">
                  <span className="beast-time">{fmtTime(log.timestamp)}</span>
                  <span className={`beast-tag ${tagClass(log.tag)}`}>{log.tag || 'INFO'}</span>
                  <span className="beast-msg">{log.message}</span>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* Right: Gallery */}
        <div className="art-canvas">
          <div className="card-header art-gallery-header">
            <span>ASSET GALLERY</span>
            <span className="art-gallery-count">{assets.length} asset{assets.length !== 1 ? 's' : ''}</span>
            <button className="art-gallery-refresh" onClick={fetchAssets} title="Refresh gallery">⟳</button>
          </div>
          <div className="art-gallery" id="art-gallery">
            {assets.length === 0 ? (
              <div className="art-empty">
                <div className="art-empty__icon">🎨</div>
                <div>No assets generated yet.</div>
                <div className="art-empty__hint">Use the tools on the left to create images, music, video, and 3D meshes.</div>
              </div>
            ) : (
              <div className="art-gallery__grid">
                {assets.map((asset) => (
                  <AssetCard
                    key={asset.filename}
                    asset={asset}
                    onPreview={setPreviewAsset}
                  />
                ))}
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Preview Modal */}
      <PreviewModal asset={previewAsset} onClose={() => setPreviewAsset(null)} />
    </div>
  );
}
