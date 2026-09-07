'use client';

import { useSentinelStore } from '@/store/useSentinelStore';
import { Power, AlertOctagon } from 'lucide-react';
import { clsx } from 'clsx';

export function KillSwitch() {
  const { agentStatus, toggleKillSwitch } = useSentinelStore();
  const isActive = agentStatus === 'ACTIVE';

  return (
    <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 flex flex-col items-center justify-center space-y-4">
      <div className="flex items-center space-x-2 text-sm uppercase tracking-wider text-slate-400 font-semibold">
        <AlertOctagon size={16} />
        <span>Emergency Override</span>
      </div>
      
      <button
        onClick={toggleKillSwitch}
        className={clsx(
          "relative group overflow-hidden rounded-full font-bold uppercase tracking-widest transition-all duration-300",
          "w-48 h-16 flex items-center justify-center space-x-2",
          isActive 
            ? "bg-red-600/10 text-red-500 border-2 border-red-600 hover:bg-red-600 hover:text-white shadow-[0_0_20px_rgba(220,38,38,0.3)]" 
            : "bg-emerald-600/10 text-emerald-500 border-2 border-emerald-600 hover:bg-emerald-600 hover:text-white"
        )}
      >
        <Power size={20} className={clsx(isActive && "animate-pulse")} />
        <span>{isActive ? 'KILL AGENT' : 'RESUME AGENT'}</span>
      </button>

      <p className="text-xs text-slate-500">
        {isActive ? 'Halts all external execution via MCP.' : 'Agent execution is currently suspended.'}
      </p>
    </div>
  );
}