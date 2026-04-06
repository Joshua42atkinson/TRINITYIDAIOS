import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ArtStudio from './ArtStudio';
import * as useCreativeHooks from '../hooks/useCreative';

// Mock the hook
vi.mock('../hooks/useCreative', () => ({
  useCreative: () => ({
    status: { comfyui: { running: true }, musicgpt: { running: false } },
    assets: [
      { filename: 'Uncategorized_image.png', url: 'test.png', asset_type: 'image', size_bytes: 1024, created_at: 0 }
    ],
    fetchAssets: vi.fn(),
  }),
}));

// Mock the fetch API for chat streaming
global.fetch = vi.fn(() =>
  Promise.resolve({
    ok: true,
    body: {
      getReader: () => {
        let readOnce = false;
        return {
          read: () => {
            if (!readOnce) {
              readOnce = true;
              return Promise.resolve({ done: false, value: new TextEncoder().encode('data: {"content":"Hello World"}\n\n') });
            }
            return Promise.resolve({ done: true });
          }
        };
      }
    }
  })
);

describe('ArtStudio Component', () => {
  it('renders the Art Studio status bar and tools', () => {
    render(<ArtStudio />);
    expect(screen.getByText('✦ ART · Window to the Imagination')).toBeInTheDocument();
    expect(screen.getByText(/ComfyUI/)).toBeInTheDocument();
    expect(screen.getByText(/MusicGPT/)).toBeInTheDocument();
  });

  it('switches tabs correctly', () => {
    render(<ArtStudio />);
    
    // Default is chat
    expect(screen.getByText(/Tell Pete what you want to create/)).toBeInTheDocument();
    
    // Click Gallery
    fireEvent.click(screen.getByRole('button', { name: /Gallery/ }));
    expect(screen.getByText('/vault')).toBeInTheDocument();
    expect(screen.getByText('Uncategorized')).toBeInTheDocument();
    
    // Click Tools
    fireEvent.click(screen.getByRole('button', { name: /Tools/ }));
    expect(screen.getByText('⚒️ FORGE TOOLS')).toBeInTheDocument();
  });

  it('allows sending a chat message', async () => {
    render(<ArtStudio />);
    const input = screen.getByPlaceholderText('Design a scary monster, Pete...');
    const sendBtn = screen.getByRole('button', { name: '↵' });

    fireEvent.change(input, { target: { value: 'Make a cube' } });
    expect(input.value).toBe('Make a cube');
    
    fireEvent.click(sendBtn);
    
    // Should display student message
    expect(await screen.findByText('"Make a cube"')).toBeInTheDocument();
  });
});
