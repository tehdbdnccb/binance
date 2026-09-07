'use client';

import { useSentinelStore } from '@/store/useSentinelStore';
import { KillSwitch } from '@/components/KillSwitch';
import { ShieldCheck, TrendingUp, AlertTriangle } from 'lucide-react';
import { ResponsiveContainer, AreaChart, Area, XAxis, Tooltip } from 'recharts';

const mockChartData = Array.from({ length: 20 }).map((_, i) => ({
  time: i,
  risk: Math.floor(Math.random() * 20) + 20,
}));

export default function Dashboard() {
  const { dashboard, agentStatus } = useSentinelStore();

  return (
    <div className="max-w-6xl mx-auto space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
      
      <header className="flex justify-between items-end mb-8">
        <div>
          <h1 className="text-3xl font-bold text-slate-100">AI Execution Firewall</h1>
          <p className="text-slate-500 mt-2">Agent: Claude 3.5 Sonnet (Trading Role)</p>
        </div>
        <div className="flex items-center space-x-2 bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 px-4 py-2 rounded-full text-sm font-medium">
          <ShieldCheck size={16} />
          <span>Protection {agentStatus}</span>
        </div>
      </header>

      {/* Top Metrics Row */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <MetricCard label="Portfolio Value" value={`$${dashboard.portfolio}`} icon={<TrendingUp size={20} />} />
        <MetricCard label="Trust Score" value={`${dashboard.trustScore} / 100`} subtext="Highly Reliable" />
        <MetricCard label="Today's P&L" value={`+$${dashboard.pnlToday}`} valueColor="text-emerald-400" />
        <MetricCard label="Risk Utilization" value={`${dashboard.riskUtilization}%`} valueColor="text-amber-400" />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        
        {/* Left Column: Charts & Controls */}
        <div className="lg:col-span-2 space-y-6">
          <div className="bg-slate-900 border border-slate-800 rounded-xl p-6">
            <h3 className="text-lg font-medium text-slate-300 mb-6 flex justify-between">
              <span>Live Risk Projection</span>
              <span className="text-xs text-slate-500 bg-slate-800 px-2 py-1 rounded">Daily Loss Limit: 3%</span>
            </h3>
            <div className="h-64">
              <ResponsiveContainer width="100%" height="100%">
                <AreaChart data={mockChartData}>
                  <defs>
                    <linearGradient id="riskGradient" x1="0" y1="0" x2="0" y2="1">
                      <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.3}/>
                      <stop offset="95%" stopColor="#3b82f6" stopOpacity={0}/>
                    </linearGradient>
                  </defs>
                  <XAxis dataKey="time" hide />
                  <Tooltip 
                    contentStyle={{ backgroundColor: '#0f172a', borderColor: '#1e293b', color: '#f8fafc' }} 
                    itemStyle={{ color: '#38bdf8' }}
                  />
                  <Area type="monotone" dataKey="risk" stroke="#38bdf8" fillOpacity={1} fill="url(#riskGradient)" />
                </AreaChart>
              </ResponsiveContainer>
            </div>
          </div>
        </div>

        {/* Right Column: Kill Switch & Quick Stats */}
        <div className="space-y-6">
          <KillSwitch />
          
          <div className="bg-slate-900 border border-slate-800 rounded-xl p-6">
            <h3 className="text-sm font-medium text-slate-400 uppercase tracking-wider mb-4">Today's Executions</h3>
            <div className="space-y-4">
              <div className="flex justify-between items-center pb-4 border-b border-slate-800">
                <span className="text-slate-300">Approved Trades</span>
                <span className="text-xl font-bold text-emerald-400">5</span>
              </div>
              <div className="flex justify-between items-center pb-4 border-b border-slate-800">
                <span className="text-slate-300">Blocked Anomalies</span>
                <span className="text-xl font-bold text-red-500">2</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-slate-300 flex items-center"><AlertTriangle size={16} className="mr-2 text-amber-500"/> Capital Protected</span>
                <span className="text-xl font-bold text-slate-100">$742.00</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

function MetricCard({ label, value, subtext, icon, valueColor = "text-slate-100" }: any) {
  return (
    <div className="bg-slate-900 border border-slate-800 rounded-xl p-5 flex flex-col justify-between">
      <div className="flex justify-between items-start text-slate-400 mb-4">
        <span className="text-sm font-medium">{label}</span>
        {icon && <span className="text-slate-500">{icon}</span>}
      </div>
      <div>
        <h2 className={`text-3xl font-bold ${valueColor}`}>{value}</h2>
        {subtext && <p className="text-xs text-slate-500 mt-1">{subtext}</p>}
      </div>
    </div>
  );
}