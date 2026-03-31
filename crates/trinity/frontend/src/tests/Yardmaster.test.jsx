import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import { vi } from 'vitest';
import Yardmaster from './Yardmaster';
import * as useYardmasterHook from '../hooks/useYardmaster';
import * as useCreativeHook from '../hooks/useCreative';

const mockSendMessage = vi.fn();
const mockToggleFocus = vi.fn();

const mockHookState = {
  messages: [{ role: 'ai', speaker: 'YARDMASTER', content: 'Ready.' }],
  activityLogs: [],
  sending: false,
  focus: new Set(),
  hardware: { cpu: 'Ryzen AI Max 395', memory: '128GB', llm_status: 'connected' },
  tools: [{ name: 'shell', description: 'Run bash' }],
  thinking: '',
  questState: {
    quest: { quest_title: 'Demo Quest', current_phase: 'Development', phase_objectives: [] },
    stats: { coal_reserves: 45, total_xp: 1500 }
  },
  turnInfo: { turn: 0, maxTurns: 65, continuations: 0 },
  modelInfo: { name: 'Mistral Small 4 119B', reasoning: 'high', context: '256K', active_experts: 'embedded', status: 'mounted' },
  toggleFocus: mockToggleFocus,
  sendMessage: mockSendMessage,
  cancelRequest: vi.fn(),
};

beforeEach(() => {
  vi.clearAllMocks();
  vi.spyOn(useYardmasterHook, 'useYardmaster').mockReturnValue(mockHookState);
  vi.spyOn(useCreativeHook, 'useCreative').mockReturnValue({ logs: [] });
});

describe('Yardmaster Interface Tests', () => {
  it('renders the model and hardware data', () => {
    render(<Yardmaster />);
    
    // Model Status HUD
    expect(screen.getByText('Mistral Small 4 119B')).toBeInTheDocument();
    expect(screen.getByText('reasoning: high')).toBeInTheDocument();
    expect(screen.getByText('256K ctx')).toBeInTheDocument();
    
    // Hardware Status
    expect(screen.getByText('CPU')).toBeInTheDocument();
    expect(screen.getByText('Ryzen AI Max 395')).toBeInTheDocument();
  });

  it('renders the quest ADDIECRAPEYE system state', () => {
    render(<Yardmaster />);
    expect(screen.getByText('Demo Quest')).toBeInTheDocument();
    expect(screen.getByText('Development')).toBeInTheDocument();
    expect(screen.getByText(/45/)).toBeInTheDocument(); // Coal gauge
  });

  it('allows persona switching (Dev -> Recycler -> Pete)', () => {
    render(<Yardmaster />);
    
    const recyclerBtn = screen.getByRole('button', { name: /Recycler/i });
    const peteBtn = screen.getByRole('button', { name: /Pete/i });

    // Since state is managed inside the component for persona, we click and expect to see it update
    fireEvent.click(recyclerBtn);
    expect(recyclerBtn).toHaveClass('ym-persona-active');
    expect(screen.getByText('KV#0')).toBeInTheDocument(); // Dedicated slot check for Recycler

    fireEvent.click(peteBtn);
    expect(peteBtn).toHaveClass('ym-persona-active');
    expect(screen.getByText('KV#1')).toBeInTheDocument(); // Dedicated slot check for Pete
  });

  it('sends messages out via the hook when hitting SEND', async () => {
    render(<Yardmaster />);
    
    const input = screen.getByPlaceholderText(/What do you need done/i);
    const sendBtn = screen.getByText('SEND');
    
    fireEvent.change(input, { target: { value: 'Build the combat loop' } });
    fireEvent.click(sendBtn);

    await waitFor(() => {
      // By default, starts in Dev persona with Global scope.
      expect(mockSendMessage).toHaveBeenCalledWith('Build the combat loop', 'dev', null);
    });
  });
});
