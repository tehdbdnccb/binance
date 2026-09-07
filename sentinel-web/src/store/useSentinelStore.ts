import { create } from 'zustand';
import axios from 'axios';

interface DashboardData {
  protectionStatus: 'ACTIVE' | 'SUSPENDED';
  portfolio: number;
  riskUtilization: number;
  pnlToday: number;
  trustScore: number;
}

interface SentinelState {
  agentStatus: 'ACTIVE' | 'SUSPENDED';
  dashboard: DashboardData;
  toggleKillSwitch: () => Promise<void>;
  fetchDashboard: () => Promise<void>;
}

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api';
const AGENT_ID = '00000000-0000-0000-0000-000000000001'; // Mock ID for MVP

export const useSentinelStore = create<SentinelState>((set, get) => ({
  agentStatus: 'ACTIVE',
  dashboard: {
    protectionStatus: 'ACTIVE',
    portfolio: 2430.22,
    riskUtilization: 42,
    pnlToday: 24.18,
    trustScore: 94,
  },
  toggleKillSwitch: async () => {
    try {
      const res = await axios.post(`${API_URL}/agents/${AGENT_ID}/kill`);
      set({ agentStatus: res.data.status });
    } catch (error) {
      console.error("Failed to toggle kill switch", error);
      // Fallback for UI demo if backend is offline
      set((state) => ({
        agentStatus: state.agentStatus === 'ACTIVE' ? 'SUSPENDED' : 'ACTIVE'
      }));
    }
  },
  fetchDashboard: async () => {
    try {
      const res = await axios.get(`${API_URL}/dashboard`);
      set({ dashboard: res.data });
    } catch (error) {
      console.error("Failed to fetch dashboard data", error);
    }
  }
}));